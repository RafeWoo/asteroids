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
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontHandle},
};
use std::time::{Duration, Instant};

use crate::game_constants::*;
use crate::entity;

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
        let ship = entity::create_ship(world);

        for _ in 0..4 {
            entity::create_rock(world, None);
        }
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

//MOve to game state
//need lives as well
/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub player_score: Entity,
}
/// Initialises a ui scoreboard
fn initialise_ui(world: &mut World) {
    let font_handle = world.read_resource::<FontHandle>().clone();


    let score_transform = UiTransform::new(
        "SCORE".to_string(), Anchor::TopMiddle,
        -50., -50., 1., 200., 50., 0,
    );
  

    let player_score = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(
            font_handle,
            "0".to_string(),
            COLOUR_WHITE,
            50.,
        )).build();

    world.add_resource(ScoreText { player_score });
}