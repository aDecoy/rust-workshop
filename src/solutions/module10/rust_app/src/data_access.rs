use std::env;
use sqlx::PgPool;
use crate::core::{ApplicationError, DataAccess, User};

pub struct PostgresUsers {
    db: PgPool,
}

impl PostgresUsers {
    pub async fn new() -> Result<Self, ApplicationError> {
        log::info!("Setting up database connection");
        
        let db_url = &env::var("DATABASE_URL")
            .map_err(|e| ApplicationError::DatabaseError(e.to_string()))?;

        let database_pool = PgPool::connect(db_url)
            .await
            .map_err(|e| ApplicationError::DatabaseError(e.to_string()))?;


        Ok(Self {
            db: database_pool,
        })
    }
}

#[async_trait::async_trait]
impl DataAccess for PostgresUsers {
    async fn with_email_address(&self, email_address: &str) -> Result<User, ApplicationError> {
        log::info!("Attempting to retrieve user from email address");
        
        let email = sqlx::query!(
            r#"
            SELECT email_address, name, password
            FROM users
            WHERE email_address = $1
            "#,
            email_address,
        )
            .fetch_optional(&self.db)
            .await;
        
        match email {
            Ok(record) => match record {
                Some(data) => {
                    let user = User::from(&data.email_address, &data.name, &data.password);
                    
                    Ok(user)
                },
                None => Err(ApplicationError::UserDoesNotExist)
            },
            Err(_) => Err(ApplicationError::UserDoesNotExist)
        }
    }

    async fn store(&self, user: User) -> Result<(), ApplicationError> {
        log::info!("Attempting to create user in the database");
        
        let _rec = sqlx::query!(
            r#"
    INSERT INTO users ( email_address, name, password )
    VALUES ( $1, $2, $3 )
            "#,
            user.email_address(),
            user.name(),
            user.password()
        )
            .fetch_one(&self.db)
            .await;

        Ok(())
    }
}