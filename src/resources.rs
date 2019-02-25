//! structures that are uses as resources in the game

use amethyst::{
    renderer::{
        SpriteSheetHandle,
    },
};

pub struct RocksResource
{
    pub sprite_sheet : SpriteSheetHandle,
}

pub struct ShipResource
{
    pub sprite_sheet : SpriteSheetHandle,
}

pub struct BulletResource
{
    pub sprite_sheet : SpriteSheetHandle,
}

#[derive(Default)]
pub struct PlayerScore
{
    score : u32,
}

impl PlayerScore
{
    pub fn reset(&mut self)
    {
        self.score = 0;
    }

    pub fn add_score(&mut self, score: u32)
    {
        self.score += score;
    }

    pub fn score(&self)->u32{
        self.score
    }
}

pub struct PauseFlag
{
    is_paused: bool,
}

impl Default for PauseFlag
{
    fn default()->PauseFlag
    {
        PauseFlag::new()
    }
}

impl PauseFlag{

    pub fn new()->PauseFlag{
        PauseFlag{
            is_paused: false,
        }
    }

    pub fn is_paused(&self)->bool{
        self.is_paused
    }

    pub fn toggle_paused(&mut self)
    {
        self.is_paused = !self.is_paused;
    }
}