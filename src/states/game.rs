//! GameState 
//! Controls the main game logic
//!  
//! Can go to PausedState, StartState or NameEntry

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

use crate::resources::RocksResource;
use crate::states::{
    NameEntryState,
    PauseState,
    StartState, 
};

pub struct GameState{

}

impl GameState{
    pub fn new()->GameState
    {
        GameState{

        }
    }
}

impl SimpleState for GameState{


    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;


        //init level

        let sprite_render = 
        {
           let bob =  world.read_resource::<RocksResource>();
    
            SpriteRender {
                sprite_sheet: bob.sprite_sheet.clone(),
                sprite_number: 0, 
            }
        };

        let mut transform = Transform::default();
        transform.set_xyz(0.0, ARENA_HEIGHT * 0.5, 0.0);

         world
        .create_entity()
        .with(sprite_render)
        .with(transform)
        .build();
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
       
    }

    
    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans
    {

        //create flying saucer occasionally

        //after thirty seconds transition to scores
        //let mut transition = Trans::None;


        //if no lives left 
            //if high score
                //then transition to name entry
            //else 
                //transiition to start screen 
 /*
        if Instant::now().duration_since( self.start_time ) > Duration::from_secs(5)
        {
            return Trans::Switch( Box::new( StartState{} ) );
        }*/

        Trans::None
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans 
    {
        let mut transition = Trans::None;

        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Pause the game by going to the `PausedState`.
                transition = Trans::Push(Box::new(PauseState::new()));
            }
        }
        
          
        transition
    }
}