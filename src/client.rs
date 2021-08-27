use redis::{Commands, Connection};

pub struct Client {
    conn: Connection,
}

impl Client{
    pub fn new() -> Client {
        let err = "Could not establish connection";
        Client {
            conn: redis::Client::open("redis://127.0.0.1/")
                .expect(err)
                .get_connection()
                .expect(err)
        }
    }

    pub fn get_all(&mut self) -> Vec<(String, String)> {
        self.get_all_keys()
            .iter()
            .map(|k| {
                (k.clone(), self.get_val(&k))
        }).collect()
    }

    fn get_all_keys(&mut self) -> Vec<String>{
        redis::cmd("KEYS")
            .arg("*")
            .query(&mut self.conn)
            .expect("Could not retrieve bulk keys from redis")
    }

    fn get_val(&mut self, k: &str) -> String {
        self.conn.get(k).expect(&*format!("Could not retrieve value for key {}", k))
    }
}

