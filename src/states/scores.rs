//! ScoresState 
//! Displays the 10 best scores
//!  
//! Can go to Start State after a time out or to GameState

use amethyst::{
    core::transform::{ Parent,},
    ecs::prelude::Entity,
    input::is_key_down,
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{Anchor, UiText, UiTransform, FontHandle},
};
use std::time::{Duration, Instant};

use crate::game_constants::*;
use crate::resources::LeaderBoard;
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
        "MESSAGE".to_string(), Anchor::TopMiddle,
        0., -50., 1., 200., 50., 0,
    );
  

    const FONT_SIZE: f32 = 30.0;
    world.create_entity()
        .with( message_transform )
        .with( UiText::new(
            font_handle,
            "HI SCORES".to_string(),
            COLOUR_WHITE,
            FONT_SIZE,
        )).build()
}

fn display_scores(world: &mut World, parent: Entity)
{
    let font_handle = world.read_resource::<FontHandle>().clone();
      const FONT_SIZE: f32 = 30.0;

    let leaderboard = world.read_resource::<LeaderBoard>().clone();

    for i in 0..10 {

        let colour = if let Some(new_entry_index) = leaderboard.new_entry_index() {
            if i == new_entry_index {
                COLOUR_RED
            }
            else
            {
                COLOUR_WHITE
            }
        }
        else
        {
            COLOUR_WHITE
        };


        let score_transform = UiTransform::new(
            i.to_string(), Anchor::TopMiddle,
            100., -150. - 50.0 * (i as f32), 1., 200., 50., 0,
        );

        world.create_entity()
            .with( score_transform )
            .with( Parent{entity:parent} )
            .with( UiText::new(
                font_handle.clone(),
                leaderboard.score_at(i).to_string(),
                colour,
                FONT_SIZE,
            )).build();


        let name_transform = UiTransform::new(
            i.to_string(), Anchor::TopMiddle,
            -100., -150. - 50.0 * (i as f32), 1., 200., 50., 0,
        );
        world.create_entity()
            .with( name_transform )
            .with( Parent{entity:parent} )
            .with( UiText::new(
                font_handle.clone(),
                leaderboard.name_at(i),
                colour,
                FONT_SIZE,
            )).build();
            
    }
}

impl SimpleState for ScoresState{

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;


        //init level
        let message_entity = display_message(world);
        self.message = Some( message_entity.clone() );


        display_scores( world, message_entity);
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

        //clear new entry index state when we leave
        {
            let mut leaderboard = world.write_resource::<LeaderBoard>();
            leaderboard.clear_new_entry_index();
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