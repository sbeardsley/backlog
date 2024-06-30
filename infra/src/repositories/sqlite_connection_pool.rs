use core::domain::errors::ConnectionError;
use sqlx::{migrate::MigrateDatabase, pool::PoolConnection, Pool, Sqlite, SqlitePool};

#[derive(Debug, Clone)]
pub struct SqliteConnectionPool {
    pool: Pool<Sqlite>,
}

impl SqliteConnectionPool {
    pub async fn new(db_url: &str) -> Result<Self, ConnectionError> {
        Self::ensure_db_exists(db_url).await?;
        Self::run_migrations(db_url).await?;

        let pool = SqlitePool::connect(db_url)
            .await
            .map_err(|_| ConnectionError::ConnectionFailed)?;

        Ok(Self { pool })
    }

    pub async fn close(&self) {
        self.pool.close().await
    }

    pub async fn connect(&self) -> Result<PoolConnection<Sqlite>, ConnectionError> {
        let mut conn = self
            .pool
            .clone()
            .acquire()
            .await
            .map_err(|_| ConnectionError::ConnectionFailed)?;

        sqlx::query!(r#"PRAGMA foreign_keys = ON;"#)
            .execute(&mut *conn)
            .await
            .map_err(|_| ConnectionError::ConnectionFailed)?;

        Ok(conn)
    }

    pub async fn ensure_db_exists(url: &str) -> Result<(), ConnectionError> {
        if !Sqlite::database_exists(url).await.unwrap_or(false) {
            println!("Creating database {}", url);
            match Sqlite::create_database(url).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            };
        };

        Ok(())
    }

    pub async fn run_migrations(url: &str) -> Result<(), ConnectionError> {
        let db = SqlitePool::connect(url)
            .await
            .map_err(|_| ConnectionError::ConnectionFailed)?;

        let migrations = std::env::current_dir()
            .map_err(|_| ConnectionError::MigrationFailed)?
            .join("./migrations");

        sqlx::migrate::Migrator::new(migrations)
            .await
            .map_err(|_| ConnectionError::MigrationFailed)?
            .run(&db)
            .await
            .map_err(|_| ConnectionError::MigrationFailed)?;

        Ok(())
    }
}
