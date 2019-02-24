//! ScoresState 
//! Displays the 10 best scores
//!  
//! Can go to Start State after a time out or to GameState

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

use crate::states::{
    StartState,
    GameState,
};

pub struct ScoresState{
     start_time: Instant,
}

impl ScoresState
{
    pub fn new() -> ScoresState
    {
        ScoresState
        {
            start_time: Instant::now(),
        }
    }
}

impl SimpleState for ScoresState{

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;


        //init level

      println!("Entered scores state");
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>)
    {
        //delete all the game entities
/*
        if let Some(entity) = self.loading_screen_handle {
            data.world.delete_entity( entity ).expect("failed to delete loading screen");
            self.loading_screen_handle = None;
        }
        */
        println!("Leaving scores state");
    }

    
    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans
    {
        //after thirty seconds transition to start
        let mut transition = Trans::None;
 
        if Instant::now().duration_since( self.start_time ) > Duration::from_secs(30)
        {
            transition = Trans::Switch( Box::new( StartState::new() ) );
        }

        transition
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans 
    {
        let mut transition = Trans::None;


        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Return) {
                
                transition = Trans::Switch(Box::new(GameState::new()));
            }
        }
        
          
        transition
    }
}