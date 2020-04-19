extern crate rust_test;
use log::info;
// use log4rs;
use rust_test::child_module::test::test2_a::a1;
fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("INFO");
    a1()
}