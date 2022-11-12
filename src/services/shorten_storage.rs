use redis;
use uuid::Uuid;

type Key = String;
#[derive(Clone)]
pub struct ShortenStorage{
    redis: redis::Client<>,
}

impl ShortenStorage{
    pub fn new(redis_uri: &str) -> ShortenStorage {
        ShortenStorage { redis: redis::Client::open(redis_uri).unwrap() }
    }

    fn get_key(&self, code: &str) -> Key {
        format!("shorten:{}", code)
    }

    fn generate_code(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub async fn get(&self, code: &str) -> Option<String> {
        let key = self.get_key(code);
        let mut connection = self.redis
        .get_tokio_connection_manager()
            .await
            .unwrap();
        let result = redis::Cmd::get(key)
            .query_async::<_, String>(&mut connection)
            .await;
        match result {
            Ok(value) => Some(value),
            Err(..) => None
        }
    }
    pub async fn save(&self, link: String) -> Result<String, String> {
        let code = self.generate_code();
        let mut connection = self.redis
        .get_tokio_connection_manager()
            .await
            .unwrap();
        let result = redis::Cmd::set(self.get_key(&code), link)
            .query_async::<_, String>(&mut connection)
            .await;
        match result {
            Ok(..) => Ok(code),
            Err(..) => Err("cannot write to db".to_string()),
        }
        
    }
}