//! NameEntryState 
//! Accepts input from user to associate with score
//! 
//! Display Congratulations You have a high score
//! Display score
//! Display ___
//! Get three key presses and then transition to scoresState
//! Can go to ScoresState 

use amethyst::{
    core::transform::Parent,
    ecs::prelude::{Entity},
    input::is_key_down,
    prelude::*,
    renderer::{ VirtualKeyCode,},
    ui::{Anchor, UiText, UiTransform, FontHandle},
};
use std::time::Instant;
use std::char;

use crate::game_constants::*;

use crate::resources::{PlayerScore,LeaderBoard};
use crate::states::{
    ScoresState,
};


pub struct NameEntryState {
    start_time: Instant,
    message: Option<Entity>,
    initials: [char;3],
    current: usize,
    blah: [Option<Entity>;3],
    underscore : Option<Entity>,
}

impl NameEntryState {

    pub fn new()->NameEntryState
    {
        NameEntryState{
            start_time: Instant::now(),
            message: None,
            initials: ['A', 'A', 'A'],
            current: 0,
            blah: [None, None, None],
            underscore: None,
        }
    }

    fn create_initials(&mut self, world: &mut World, entity: Entity)
    {
        let font_handle = world.read_resource::<FontHandle>().clone();
        let font_size = 60.0;

        for i in 0..3 {
            let letter_transform = UiTransform::new(
            i.to_string(), Anchor::TopMiddle,
            -80. + 80. * i as f32, -300., 1., 
            600., 100., 
            0,
            );

            self.blah[i] = Some(world.create_entity()
                .with( letter_transform )
                .with( Parent{entity})
                .with( UiText::new(
                    font_handle.clone(),
                    self.initials[i].to_string(),
                    COLOUR_WHITE,
                    font_size,
                )).build());
        }

        {
            let letter_transform = UiTransform::new(
            "UNDERSCORE".to_string(), Anchor::TopMiddle,
            -80. , -305., 1., 
            600., 100., 
            0,
            );

            self.underscore = Some(world.create_entity()
                .with( letter_transform )
                .with( Parent{entity})
                .with( UiText::new(
                    font_handle.clone(),
                    "_".to_string(),
                    COLOUR_WHITE,
                    font_size,
                )).build());
             
        }
    }

}

fn display_message(world: &mut World)->Entity
{
    let font_handle = world.read_resource::<FontHandle>().clone();
        
    let message_transform = UiTransform::new(
        "MESSAGE".to_string(), Anchor::TopMiddle,
        0., -100., 1., 
        600., 100., 
        0,
    );
  
    let font_size = 40.0;
    let top =  world.create_entity()
        .with( message_transform )
        .with( UiText::new(
            font_handle.clone(),
            "You Got a High Score".to_string(),
            COLOUR_WHITE,
            font_size,
        )).build();


    let message2_transform = UiTransform::new(
        "MESSAGE2".to_string(), Anchor::TopMiddle,
        0., -200., 1., 
        600., 100., 
        0,
    );

    world.create_entity()
        .with( message2_transform )
        .with( Parent{entity:top})
        .with( UiText::new(
            font_handle,
            "Please Enter Your Name".to_string(),
            COLOUR_WHITE,
            font_size,
        )).build();


    top
}



impl SimpleState for NameEntryState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let message_entity = display_message(world);
        self.message = Some( message_entity.clone() );

        self.create_initials(world, message_entity);
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
            scores.add_entry(player_score, self.initials,);
        }

        //println!("Leaving name entry state");
    }

    
    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans
    {
        let mut transition = Trans::None;
 
        let world = data.world;
       
        if let Some(underscore) = self.underscore {
            let mut storage = world.write_storage::<UiTransform>();
            let mut underscore_transform = storage.get_mut(underscore).expect("failed to get ui transform for underscore");
            underscore_transform.local_x = -80.0 + 80.0 * self.current as f32;
        }

        let mut text_storage = world.write_storage::<UiText>();
        for i in 0..3
        {
            if let Some(letter) = self.blah[i] {
                let letter_text = text_storage.get_mut(letter).expect("failed to get ui text");
                letter_text.text = self.initials[i].to_string();
            }
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
            else if is_key_down(&event, VirtualKeyCode::Left){
                if self.current > 0 {
                    self.current -= 1;
                }
            }
            else if is_key_down(&event, VirtualKeyCode::Right){
                if self.current < 2 {
                    self.current += 1;
                }
            }
            else if is_key_down(&event, VirtualKeyCode::Up){
                let mut letter = self.initials[self.current] as u32;
                let base = 'A' as u32;
                letter = ((letter - base +1) % 26) + base; 
                self.initials[self.current] = char::from_u32(letter).unwrap();
            }
            else if is_key_down(&event, VirtualKeyCode::Down){
                  let mut letter = self.initials[self.current] as u32;
                let base = 'A' as u32;
                letter = ((letter - base + 25 ) % 26) + base; 
                self.initials[self.current] = char::from_u32(letter).unwrap();
            }
        }
        
          
        transition
    }
}