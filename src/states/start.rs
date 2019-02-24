//! StartState 
//! Attract mode waiting for player to press start
//!  
//! Can go to GameState Scores


use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::{ Transform, Parent, },
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

use crate::game_constants::{ ARENA_WIDTH, ARENA_HEIGHT, COLOUR_WHITE};

use crate::resources;
use crate::entity;
use crate::states::
{
    GameState,
    ScoresState,
};

pub struct StartState{
    start_time: Instant,
    message: Option<Entity>,
}

impl StartState{

    pub fn new() -> StartState{
        StartState{
            start_time: Instant::now(),
            message: None,
        }
    }
}


fn display_start_message(world: &mut World)->Entity
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
            "PRESS ENTER TO START".to_string(),
            COLOUR_WHITE,
            50.,
        )).build()
}
impl SimpleState for StartState{


    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.start_time = Instant::now();

        let message_entity = display_start_message(world);
        self.message = Some( message_entity.clone() );

        for _ in 0..4 {
            entity::create_rock(world, Some(message_entity.clone()));   
        }
        
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        if let Some(message_entity) = self.message {
            data.world.delete_entity( message_entity ).expect("failed to delete message");
            self.message = None;
        }

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