//! structures that are uses as resources in the game

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage,Entity},
    prelude::*,
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontHandle},
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