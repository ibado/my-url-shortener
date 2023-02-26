use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct UrlRepo {
    db: Arc<RwLock<HashMap<String, String>>>,
}

impl UrlRepo {
    pub fn new() -> UrlRepo {
        let mut hm = HashMap::new();
        hm.insert("asd123".to_string(), "https://github.com/ibado".to_string());
        UrlRepo {
            db: Arc::new(RwLock::new(hm)),
        }
    }

    pub fn find(&self, key: &str) -> Option<String> {
        println!("db: {:?}", self.db);
        self.db
            .read()
            .expect("ups reading")
            .get(key)
            .map(|s| s.clone())
    }

    pub fn put(&self, key: &str, value: &str) {
        println!("db before: {:?}", self.db);
        self.db
            .write()
            .expect("ups writing")
            .insert(key.to_string(), value.to_string());
        println!("db after: {:?}", self.db);
    }
}
