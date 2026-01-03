use std::sync::{OnceLock, RwLock};
use std::collections::HashMap;

static DRIVERS: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();

fn get_drivers_lock() -> &'static RwLock<HashMap<String, String>> {
    DRIVERS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_driver(key: &str) -> Option<String> {
    let drivers_lock = get_drivers_lock();
    let map = drivers_lock.read().unwrap();
    map.get(key).cloned()
}

pub fn add_driver(driver_name: &str, driver_description: &str) {
    let drivers_lock = get_drivers_lock();
    let mut map = drivers_lock.write().unwrap();
    map.insert(driver_name.to_string(), driver_description.to_string());
}