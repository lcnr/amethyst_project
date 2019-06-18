use amethyst::{
    ecs::prelude::*,
    prelude::*,
};

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

pub struct Collider {
    pub w: f32,
    pub h: f32
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}