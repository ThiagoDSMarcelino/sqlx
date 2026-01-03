use std::collections::HashMap;
use std::sync::Arc;
use std::sync::{OnceLock, RwLock};

static DRIVERS: OnceLock<RwLock<HashMap<String, Arc<dyn Driver>>>> = OnceLock::new();

pub trait Driver: Send + Sync {
    fn name(&self) -> &str;

    fn test_connection(&self, dns: &str) -> Result<(), String>;

    fn execute_query(&self, dns: &str, query: &str) -> Result<String, String>;
}

fn get_drivers_lock() -> &'static RwLock<HashMap<String, Arc<dyn Driver>>> {
    DRIVERS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_supported_drivers() -> Vec<(String, Vec<String>)> {
    let drivers_lock = get_drivers_lock();
    let map = drivers_lock.read().unwrap();

    let mut grouped: HashMap<String, Vec<String>> = HashMap::new();

    for (alias, driver) in map.iter() {
        let driver_name = driver.name();

        grouped
            .entry(driver_name.to_string())
            .or_default()
            .push(alias.clone());
    }

    grouped.into_iter().collect()
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
