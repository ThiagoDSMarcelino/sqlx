use std::collections::HashMap;
use std::sync::Arc;
use std::sync::{OnceLock, RwLock};

static DRIVERS: OnceLock<RwLock<HashMap<String, Arc<dyn Driver>>>> = OnceLock::new();

pub trait Driver: Send + Sync {
    fn test_connection(&self, dns: &str) -> Result<(), String>;
}

fn get_drivers_lock() -> &'static RwLock<HashMap<String, Arc<dyn Driver>>> {
    DRIVERS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_driver(key: &str) -> Option<Arc<dyn Driver>> {
    let drivers_lock = get_drivers_lock();
    let map = drivers_lock.read().unwrap();
    map.get(key).cloned()
}

pub(crate) fn add_driver(aliases: &[&str], driver: Arc<dyn Driver>) {
    let drivers_lock = get_drivers_lock();
    let mut map = drivers_lock.write().unwrap();
    for alias in aliases.iter() {
        map.insert(alias.to_string(), driver.clone());
    }
}
