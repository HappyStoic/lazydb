use redis::{Commands, Connection};
use crate::CliArgs;

pub struct Client {
    conn: Connection,
}

impl Client{
    pub fn new(args: &CliArgs) -> Client {
        let err = format!("Could not establish connection to {}:{}", args.host, args.port);
        Client {
            conn: redis::Client::open(format!("redis://{}:{}/", args.host, args.port))
                .expect(err.as_str())
                .get_connection()
                .expect(err.as_str())
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

