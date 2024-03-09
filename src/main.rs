use crate::container::{get_all_containers, Container};

mod container;

fn main() {
    let containers = get_all_containers().ok().unwrap();
    let first = containers.first().unwrap();
    first.try_start().ok().unwrap();
    println!("Containers: {:#?}", containers);
}
