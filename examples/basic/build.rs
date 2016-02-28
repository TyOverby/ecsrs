extern crate ecsrs;
extern crate latin;

use ecsrs::*;

pub fn main() {
    let mut world = WorldBuilder::new();
    world.add_component("::Position");
    world.add_component("::Velocity");
    latin::file::write("./src/ecs.rs", world.render()).unwrap()
}
