use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    dns: String,
    driver: String,
}

impl Connection {
    pub fn new(dns: &str, driver: &str) -> Self {
        Connection {
            dns: dns.to_string(),
            driver: driver.to_string(),
        }
    }

    pub fn dns(&self) -> &str {
        &self.dns
    }

    pub fn driver(&self) -> &str {
        &self.driver
    }
}
