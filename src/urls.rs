use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

#[derive(Clone, Debug)]
pub struct UrlRepo {
    db_pool: SqlitePool,
}

impl UrlRepo {
    pub async fn new() -> Option<UrlRepo> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var is missing!");
        if Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            println!("DB is already created, skipping...")
        } else {
            println!("Creating database {}...", &db_url);
            match Sqlite::create_database(&db_url).await {
                Ok(_) => println!("DB created!"),
                Err(error) => panic!("error: {}", error),
            }
        }
        let pool = SqlitePool::connect(&db_url).await.ok()?;
        sqlx::migrate!().run(&pool).await.ok()?;
        Some(UrlRepo { db_pool: pool })
    }

    pub async fn find(&self, key: u32) -> Option<String> {
        match sqlx::query!("SELECT * FROM urls WHERE id = $1", key)
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(record) => {
                println!("Saved URL: {}", record.url);
                Some(record.url)
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }

    pub async fn put(&self, url: &str) -> Option<u32> {
        match sqlx::query!("INSERT INTO urls (url) VALUES ($1) RETURNING id;", url)
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(result) => Some(result.id as u32),
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }
    }
}
