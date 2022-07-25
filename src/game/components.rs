use bevy::prelude::*;

#[derive(Component)]
pub struct Snake;

#[derive(Component)]
pub struct SnakeBody;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Blocker;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Collision;

#[derive(Copy, Clone, Component, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct Age(pub u32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Component)]
pub struct SnakeMeta {
    pub len: u32,
    pub dir: Direction,
    // direction snake used to reach current position
    // used to forbid moving backwards by changing direction twice
    pub prev_dir: Direction,
}
