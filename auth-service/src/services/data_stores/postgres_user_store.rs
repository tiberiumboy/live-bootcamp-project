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

    pub fn verify_password_hash(
        expected_password_hash: String,
        password_candidate: String,
    ) -> Result<(), argon2::password_hash::Error> {
        let expected_password_hash = PasswordHash::new(&expected_password_hash)?;
        Argon2::default().verify_password(password_candidate.as_bytes(), &expected_password_hash)
    }

    pub fn compute_password_hash(password: String) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(&password.as_bytes(), &salt)?
        .to_string();
        Ok(password_hash)
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
    // TODO: fulfill the implementation for this trait
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email: &Email = user.as_ref();
        let user_pwd: &Password = user.as_ref();
        let pwd_str: String = user_pwd.as_ref().to_owned();
        let password_hash = tokio::task::spawn_blocking(move || {
            Self::compute_password_hash(pwd_str).map_err(|_| UserStoreError::UnexpectedError)
        })
        .await
        .map_err(|_| UserStoreError::UnexpectedError)??;
        let requires_2fa = user.requires_2fa();
        // is there sanitization for the database?
        sqlx::query_as!(
            UserDB,
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ( $1, $2, $3 );",
            email.as_ref(),
            password_hash,
            requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        Ok(())
    }

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

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<User, UserStoreError> {
        let user = self.get_user(email).await?;

        let pwd_str: String = password.as_ref().to_owned();
        let pwd: &Password = user.as_ref();
        let pwd = pwd.as_ref().to_string().to_owned();
        let result = tokio::task::spawn_blocking(move || {
            PostgresUserStore::verify_password_hash(pwd, pwd_str)
        })
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        match result {
            Ok(_) => Ok(user),
            Err(_) => Err(UserStoreError::InvalidCredentials),
        }
    }

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
