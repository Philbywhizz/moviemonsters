// Components used by Movie Monster

use crate::prelude::*;

// A renderable component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandomMovement {
    pub current_delta: f32,
    pub trigger_delta: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

// A Monster Tag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monster;

// An Actor Tag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Actor;
