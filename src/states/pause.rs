//! PauseState is entered when player has paused gameplay
//! 
//! Can go to gameplay state
//! Waiting for player to press unpause key
//! 
//! Display "Paused" on Screen
//! set a paused flag
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage,Entity},
    input::is_key_down,
    prelude::*,
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,VirtualKeyCode,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use std::time::{Duration, Instant};

use crate::game_constants::{ ARENA_WIDTH, ARENA_HEIGHT,};

pub struct PauseState
{

}

impl PauseState
{
    pub fn new() -> PauseState
    {
        PauseState{

        }
    }
}

impl SimpleState for PauseState
{

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let _world = data.world;

        //set global pause flag
        //display paused text
      println!("Entered pause state");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>)
    {
        //remove paused text
        //unset global pause flag
        println!("Leaving pause state");
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans 
    {
        let mut transition = Trans::None;

        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                
                transition = Trans::Pop;
            }
        }
        
        transition
    }
}