mod drivers;
mod postgresql_driver;

pub(super) use drivers::add_driver;
pub use drivers::{Driver, get_driver, get_supported_drivers};

#[ctor::ctor]
fn auto_register() {
    postgresql_driver::register();
}
