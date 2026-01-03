mod drivers;
mod postgresql_driver;

pub(super) use drivers::add_driver;
pub use drivers::{Driver, get_driver};

#[ctor::ctor]
fn auto_register() {
    postgresql_driver::register();
}
