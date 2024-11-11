use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use sqlx::PgPool;

use crate::domain::{
    data_store::{UserStore, UserStoreError},
    email::Email,
    password::Password,
    user::User,
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "Verify password hash", skip_all)]
    pub async fn verify_password_hash(
        expected_password_hash: String,
        password_candidate: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let curr_span: tracing::Span = tracing::Span::current();

        let result = tokio::task::spawn_blocking(move || {
            curr_span.in_scope(|| {
                let expected_password_hash = PasswordHash::new(&expected_password_hash)?;
                Argon2::default()
                    .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                    .map_err(|e| e.into())
            })
        })
        .await;

        result?
    }

    pub async fn compute_password_hash(
        password: String,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let span = tracing::Span::current();
        let result = tokio::task::spawn_blocking(move || {
            span.in_scope(|| {
                let salt = SaltString::generate(&mut rand::thread_rng());
                let result = Argon2::new(
                    Algorithm::Argon2id,
                    Version::V0x13,
                    Params::new(15000, 2, 1, None)?,
                )
                .hash_password(&password.as_bytes(), &salt)?
                .to_string();

                Ok(result)
                // Temporary error to display Task 2 idiomatic errors part.
                // Err(
                //     Box::new(std::io::Error::other("Oh no! What shall we ever do?"))
                //         as Box<dyn Error + Send + Sync>,
                // )
            })
        })
        .await;

        result?
    }
}

#[derive(sqlx::FromRow, Debug)]
struct UserDB {
    email: String,
    password_hash: String,
    requires_2fa: bool,
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    #[tracing::instrument(name = "Add user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email: &Email = user.as_ref();
        let user_pwd: &Password = user.as_ref();
        let pwd_str = user_pwd.as_ref();
        // todo: need to read more about how I can move objects into thread properly?
        // I would like to learn more about how the move keyword works?
        let password_hash = Self::compute_password_hash(pwd_str.to_owned())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        sqlx::query!(
            "INSERT INTO users (email, password_hash, requires_2FA) VALUES( $1, $2, $3);",
            email.as_ref(),
            password_hash,
            user.requires_2fa()
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        Ok(())
    }

    #[tracing::instrument(name = "Fetch user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let sql = "SELECT email, password_hash, requires_2fa FROM users WHERE email = $1";
        let email = email.as_ref();
        match sqlx::query_as::<_, UserDB>(sql)
            .bind(email.to_string())
            .fetch_one(&self.pool)
            .await
        {
            Ok(userdb) => {
                // question - since our password contains salt in it, how do we verify? Do we need to validate through here?
                let email =
                    Email::parse(&userdb.email).map_err(|_| UserStoreError::UnexpectedError)?;

                // this seems wrong?
                let password = Password::parse(&userdb.password_hash)
                    .map_err(|_| UserStoreError::UnexpectedError)?;
                Ok(User::new(email, password, userdb.requires_2fa))
            }
            Err(_) => Err(UserStoreError::UserNotFound),
        }
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
            Err(_) => Err(UserStoreError::UnexpectedError),
        }
    }

    #[tracing::instrument(name = "Delete user from PostgreSQL", skip_all)]
    async fn delete_user(&mut self, email: Email) -> Result<(), UserStoreError> {
        let sql = "DELETE FROM users WHERE email = ?";
        match sqlx::query(sql)
            .bind(email.as_ref())
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(UserStoreError::UserNotFound),
        }
    }
}
