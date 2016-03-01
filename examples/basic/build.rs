extern crate ecsrs_build;
extern crate latin;

use ecsrs_build::*;

pub fn main() {
    let mut world = WorldBuilder::new();
    let position = world.add_component("::Position");
    let velocity = world.add_component("::Velocity");

    world.add_system(
        System::new("::MovementSystem")
        .with_component(position)
        .with_component(velocity));
   

    latin::file::write("./src/ecs.rs", world.render()).unwrap()
}
