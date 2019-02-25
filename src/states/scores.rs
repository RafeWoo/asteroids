//! ScoresState 
//! Displays the 10 best scores
//!  
//! Can go to Start State after a time out or to GameState

use amethyst::{
    assets::{Loader},
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

use crate::states::{
    StartState,
    GameState,
};

pub struct ScoresState{
     start_time: Instant,
     message: Option<Entity>,
}

impl ScoresState
{
    pub fn new() -> ScoresState
    {
        ScoresState
        {
            start_time: Instant::now(),
            message: None,
        }
    }
}

fn display_message(world: &mut World)->Entity
{
    let font_handle = world.read_resource::<FontHandle>().clone();
        
    let message_transform = UiTransform::new(
        "MESSAGE".to_string(), Anchor::Middle,
        0., 0., 1., 
        600., 100., 
        0,
    );
  

    world.create_entity()
        .with( message_transform )
        .with( UiText::new(
            font_handle,
            "HI SCORES - TODO".to_string(),
            COLOUR_WHITE,
            50.,
        )).build()
}

impl SimpleState for ScoresState{

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;


        //init level
        let message_entity = display_message(world);
        self.message = Some( message_entity.clone() );

        println!("Entered scores state");
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>)
    {
        let world = data.world;
        
        //delete all the game entities
        if let Some(entity) = self.message {
            world.delete_entity( entity ).expect("failed to delete loading screen");
            self.message = None;
        }
        println!("Leaving scores state");
    }

    
    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans
    {
        //after thirty seconds transition to start
        let mut transition = Trans::None;
 
        if Instant::now().duration_since( self.start_time ) > Duration::from_secs(10)
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