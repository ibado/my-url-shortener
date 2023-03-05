use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct UrlRepo {
    db_pool: PgPool,
}

impl UrlRepo {
    pub async fn new() -> UrlRepo {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var is missing!");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await;
        UrlRepo { db_pool: pool }
    }

    pub async fn find(&self, key: &str) -> Option<String> {
        match sqlx::query!("SELECT * FROM urls WHERE id = $1", key)
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(record) => {
                println!("Saved URL: {}", record.url);
                Some(format!("{:?}", record.url))
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }

    pub async fn put(&self, url: &str) -> Option<String> {
        let id = "asd124";
        match sqlx::query!("INSERT INTO urls (id, url) VALUES ($1, $2);", id, url)
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => Some(id.to_string()),
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }
    }
}
