#![allow(dead_code, unused_imports)]

extern crate ecsrs;

mod ecs;

struct Position(f32, f32);
struct Velocity(f32, f32);

struct MovementSystem;
