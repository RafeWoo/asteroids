//! game constants
//! 

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;


pub const COLOUR_WHITE: [f32; 4] = [1.,1.,1.,1.];
pub const COLOUR_RED: [f32; 4] = [1.,0.,0.,1.];

/// area around middle of screen where rocks should not be generated
pub const EXCLUSION_ZONE_RADIUS: f32 = 150.0;
pub const MIN_ROCK_SPEED: f32 = 5.0;
pub const MAX_SPEED: f32 = 800.0;

pub const PLAYER_ROT_SPEED: f32 = 3.12; //rad/s
pub const PLAYER_ACC: f32 = 200.0; //pixels/s-2
pub const PLAYER_SHOT_TIME_MS: u64 = 500; //milliseconds between shots

pub const SHIP_SPRITE: usize = 0;
pub const SHIP_SPRITE_FLAME: usize = 1;
pub const SHIP_RADIUS: f32 = 20.0; 

pub const BULLET_SPEED: f32 = 500.0;
pub const BULLET_LIFETIME_MS: u64 = 2000; //milliseconds of life before dying
pub const BULLET_RADIUS: f32 = 8.0; 

pub const ROCK_RADIUS_BIG: f32 = 60.0;
pub const ROCK_RADIUS_MEDIUM: f32 = 30.0;
pub const ROCK_RADIUS_SMALL: f32 = 15.0;