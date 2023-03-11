use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct UrlRepo {
    db_pool: PgPool,
}

impl UrlRepo {
    pub async fn new() -> Option<UrlRepo> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var is missing!");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .ok()?;
        sqlx::migrate!().run(&pool).await.ok()?;
        Some(UrlRepo { db_pool: pool })
    }

    pub async fn find(&self, key: u32) -> Option<String> {
        match sqlx::query!("SELECT * FROM urls WHERE id = $1", key as i32)
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
