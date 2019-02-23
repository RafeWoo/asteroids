//! StartState 
//! Attract mode waiting for player to press start
//!  
//! Can go to GameState Scores


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

use crate::states::loading::RockSpriteSheet;
use crate::states::
{
    GameState,
    ScoresState,
};

pub struct StartState{
    start_time: Instant
}

impl StartState{

    pub fn new() -> StartState{
        StartState{
            start_time: Instant::now(),
        }
    }
}

impl SimpleState for StartState{


    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.start_time = Instant::now();
////////////////////////////////////////////////
        let sprite_render = 
        {
           let bob =  world.read_resource::<RockSpriteSheet>();
    
            SpriteRender {
                sprite_sheet: bob.sprite_sheet.clone(),
                sprite_number: 0, 
            }
        };

        let mut transform = Transform::default();
        transform.set_xyz(ARENA_HEIGHT * 0.5, ARENA_HEIGHT * 0.5, 0.0);

         world
        .create_entity()
        .with(sprite_render)
        .with(transform)
        .build();
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>){

        /*
        if let Some(entity) = self.loading_screen_handle {
            data.world.delete_entity( entity ).expect("failed to delete loading screen");
            self.loading_screen_handle = None;
        }
        */
       
    }

    
    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans
    {

        //after thirty seconds transition to scores
        let mut transition = Trans::None;
 
        if Instant::now().duration_since( self.start_time ) > Duration::from_secs(30)
        {
            transition = Trans::Switch( Box::new( ScoresState::new() ) );
        }

        transition
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans 
    {
        
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Return) {
                // Pause the game by going to the `PausedState`.
                return Trans::Switch(Box::new(GameState::new()));
            }
        }
        
     
        Trans::None
    }
}