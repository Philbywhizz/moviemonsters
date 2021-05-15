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
    pub speed: i32,
}

// A Monster Tag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monster;

// An Actor Tag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Actor;
