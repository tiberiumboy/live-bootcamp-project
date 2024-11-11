use crate::domain::{
    data_store::{UserStore, UserStoreError},
    email::Email,
    password::Password,
    user::User,
};
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use color_eyre::eyre::{Context, Result};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "Verify password hash", skip_all)]
    pub async fn verify_password_hash(
        expected_password_hash: Secret<String>,
        password_candidate: Secret<String>,
    ) -> Result<()> {
        let curr_span: tracing::Span = tracing::Span::current();

        let result = tokio::task::spawn_blocking(move || {
            curr_span.in_scope(|| {
                let expected_password_hash =
                    PasswordHash::new(expected_password_hash.expose_secret())?;
                Argon2::default()
                    .verify_password(
                        password_candidate.expose_secret().as_bytes(),
                        &expected_password_hash,
                    )
                    .wrap_err("Fail to verify password hash")
            })
        })
        .await;

        result?
    }

    #[tracing::instrument(name = "Compute password hash", skip_all)]
    pub async fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>> {
        let span = tracing::Span::current();
        let result = tokio::task::spawn_blocking(move || {
            span.in_scope(|| {
                let salt = SaltString::generate(&mut rand::thread_rng());
                let result = Argon2::new(
                    Algorithm::Argon2id,
                    Version::V0x13,
                    Params::new(15000, 2, 1, None)?,
                )
                .hash_password(&password.expose_secret().as_bytes(), &salt)?
                .to_string();

                Ok(Secret::new(result))
            })
        })
        .await;

        result?
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    #[tracing::instrument(name = "Add user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email: &Email = user.as_ref();
        let user_pwd: &Password = user.as_ref();
        let password_hash = Self::compute_password_hash(user_pwd.as_ref().to_owned())
            .await
            .map_err(UserStoreError::UnexpectedError)?;

        sqlx::query!(
            "INSERT INTO users (email, password_hash, requires_2FA) VALUES( $1, $2, $3);",
            email.as_ref().expose_secret(),
            password_hash.expose_secret(),
            user.requires_2fa()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

        Ok(())
    }

    #[tracing::instrument(name = "Fetch user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        sqlx::query!(
            r#"SELECT email, password_hash, requires_2fa FROM users WHERE email = $1"#,
            email.as_ref().expose_secret()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(e.into()))
        .map(|row| {
            let email = Email::parse(row.email).map_err(UserStoreError::UnexpectedError)?;
            let secret = Secret::new(row.password_hash);
            let password = Password::parse(secret).map_err(UserStoreError::UnexpectedError)?;
            Ok(User::new(email, password, row.requires_2fa))
        })
        .ok_or(UserStoreError::UserNotFound)?
    }

    #[tracing::instrument(name = "Validate user from PostgreSQL", skip_all)]
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<User, UserStoreError> {
        let user = self.get_user(email).await?;

        let pwd_str = password.as_ref();
        let pwd: &Password = user.as_ref();
        let pwd = pwd.as_ref();
        let result =
            PostgresUserStore::verify_password_hash(pwd.to_owned(), pwd_str.to_owned()).await;

        match result {
            Ok(_) => Ok(user),
            Err(e) => Err(UserStoreError::UnexpectedError(e.into())),
            Err(e) => Err(UserStoreError::UnexpectedError(e.into())),
        }
    }

    #[tracing::instrument(name = "Delete user from PostgreSQL", skip_all)]
    async fn delete_user(&mut self, email: Email) -> Result<(), UserStoreError> {
        let sql = "DELETE FROM users WHERE email = ?";
        if sqlx::query(sql)
            .bind(email.as_ref().expose_secret())
            .execute(&self.pool)
            .await
            .is_err()
        {
            return Err(UserStoreError::UserNotFound);
        }

        Ok(())
    }
}
