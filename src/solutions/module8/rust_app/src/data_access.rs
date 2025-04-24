use std::env;
use sqlx::PgPool;
use crate::core::{ApplicationError, DataAccess, User};

pub struct PostgresUsers {
    db: PgPool,
}

impl PostgresUsers {
    pub async fn new() -> Result<Self, ApplicationError> {
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
        // Create the query and pass in parameters
        let email = sqlx::query!(
            r#"
            SELECT email_address, name, password
            FROM users
            WHERE email_address = $1
            "#,
            email_address,
        )
            // Query the database, returning an Option if no row is found
            .fetch_optional(&self.db)
            .await;
        
        // Check to see if the Option is Some or None
        match email {
            // If it is, parse the record
            Ok(record) => match record {
                Some(data) => {
                    // Create a new User instance from the data, the data struct
                    // is strongly typed based on database properties.
                    let user = User::from(&data.email_address, &data.name, &data.password);
                    
                    Ok(user)
                },
                None => Err(ApplicationError::UserDoesNotExist)
            },
            Err(_) => Err(ApplicationError::UserDoesNotExist)
        }
    }

    // Same again for inserts
    async fn store(&self, user: User) -> Result<(), ApplicationError> {
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