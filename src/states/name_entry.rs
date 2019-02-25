//! NameEntryState 
//! Accepts input from user to associate with score
//! 
//! Display Congratulations You have a high score
//! Display score
//! Display ___
//! Get three key presses and then transition to scoresState
//! Can go to ScoresState 

use amethyst::{
    ecs::prelude::{Entity},
    input::is_key_down,
    prelude::*,
    renderer::{ VirtualKeyCode,},
    ui::{Anchor, UiText, UiTransform, FontHandle},
};
use std::time::{Duration, Instant};

use crate::game_constants::*;

use crate::resources::{PlayerScore,LeaderBoard};
use crate::states::{
    ScoresState,
};


pub struct NameEntryState {
    start_time: Instant,
    message: Option<Entity>,
}

impl NameEntryState {

    pub fn new()->NameEntryState
    {
        NameEntryState{
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
            "YOU GOT A HI SCORE".to_string(),
            COLOUR_WHITE,
            50.,
        )).build()
}

impl SimpleState for NameEntryState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let message_entity = display_message(world);
        self.message = Some( message_entity.clone() );

        println!("Entered name entry state");
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>)
    {
        let world = data.world;

        if let Some(entity) = self.message {
            world.delete_entity( entity ).expect("failed to delete loading screen");
            self.message = None;
        }


        //add new entry to leaderboards
        {
            let player_score = world.read_resource::<PlayerScore>().score();
            let mut scores = world.write_resource::<LeaderBoard>();
            scores.add_entry(player_score, ['W','O','O'],);
        }

        println!("Leaving name entry state");
    }

    
    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans
    {
        let mut transition = Trans::None;
 
        if Instant::now().duration_since( self.start_time ) > Duration::from_secs(3)
        {
            transition = Trans::Switch( Box::new( ScoresState::new() ) );
        }

        transition
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans 
    {
        let mut transition = Trans::None;


        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Return) {
                
                transition = Trans::Switch(Box::new(ScoresState::new()));
            }
        }
        
          
        transition
    }
}