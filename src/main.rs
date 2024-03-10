use crate::container::get_all_containers;

pub mod container;

fn main() {
    let containers = get_all_containers().unwrap();
    println!("{:#?}", containers);
}
