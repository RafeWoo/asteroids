//! game constants
//! 

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;


pub const COLOUR_WHITE: [f32; 4] = [1.,1.,1.,1.];

/// area around middle of screen where rocks should not be generated
pub const EXCLUSION_ZONE_RADIUS: f32 = 150.0;
pub const MIN_ROCK_SPEED: f32 = 5.0;
pub const MAX_SPEED: f32 = 800.0;

pub const PLAYER_ROT_SPEED: f32 = 2.0; //rad/s
pub const PLAYER_ACC: f32 = 200.0; //pixels/s-2

pub const SHIP_SPRITE: usize = 0;
pub const SHIP_SPRITE_FLAME: usize = 1;