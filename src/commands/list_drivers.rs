use crate::drivers;

pub fn list_drivers() {
    let drivers = drivers::get_supported_drivers();
    println!("Supported Drivers:");
    for driver in drivers {
        let alias = driver.1.join(", ");
        println!(" - {}: {}", driver.0, alias);
    }
}
