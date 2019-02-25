//! GameState 
//! Controls the main game logic
//!  
//! Can go to PausedState, StartState or NameEntry

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::*, 
    input::is_key_down,
    prelude::*,
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,VirtualKeyCode,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontHandle},
};
//use std::time::{Duration, Instant};
use std::vec::Vec;

use crate::game_constants::*;
use crate::entity;
use crate::systems::{Ship, Rock, Bullet,};

use crate::resources::PlayerScore;
use crate::states::{
    NameEntryState,
    PauseState,
    StartState, 
};

pub struct GameState{
    level : i32,
    lives : i32,
    score_text : Option<Entity>,
    score_num  : Option<Entity>,
    lives_text : Option<Entity>,
    lives_num  : Option<Entity>,
    level_text : Option<Entity>,
    level_num  : Option<Entity>,
}

impl GameState{
    pub fn new()->GameState
    {
        GameState{
            level : 1,
            lives : 3,
            score_text : None,
            score_num : None,
            lives_text : None,
            lives_num : None,
            level_text: None,
            level_num : None,
        }
    }


    pub fn start_new_level(&self, world: &mut World)
    {
        entity::create_ship(world);
        
        let max_speed = 50.0 + 10.0 * self.level as f32;
        let num_rocks = self.level+3;
        for _ in 0..num_rocks{
            entity::create_rock(world, max_speed, None);
        }
    }

    

    //  ScoreText contains the ui text components that display the score
    fn init_ui(&mut self, world: &mut World) {
        let font_handle = world.read_resource::<FontHandle>().clone();
        let font_size = 20.0;

        let score_transform = UiTransform::new(
            "SCORE_TEXT".to_string(), Anchor::TopMiddle,
            -30., -50., 1., 200., 50., 0,
        );
    
        self.score_text = 
        Some( world.create_entity()
            .with(score_transform)
            .with(UiText::new(
                font_handle.clone(),
                "SCORE:".to_string(),
                COLOUR_WHITE,
                font_size,
            )).build() );

        
        let score_num_transform = UiTransform::new(
            "SCORE_NUM".to_string(), Anchor::TopMiddle,
            30., -50., 1., 200., 50., 0,
        );
    
        self.score_num = 
        Some( world.create_entity()
            .with(score_num_transform)
            .with(UiText::new(
                font_handle.clone(),
                "0".to_string(),
                COLOUR_WHITE,
                font_size,
            )).build() );


        let lives_text_transform = UiTransform::new(
            "LIVES_TEXT".to_string(), Anchor::TopRight,
            -200., -50., 1., 200., 50., 0,
        );
    
        self.lives_text = 
        Some( world.create_entity()
            .with(lives_text_transform)
            .with(UiText::new(
                font_handle.clone(),
                "LIVES:".to_string(),
                COLOUR_WHITE,
                font_size,
            )).build() );

        
        let lives_num_transform = UiTransform::new(
            "LIVES_NUM".to_string(), Anchor::TopRight,
            -160., -50., 1., 200., 50., 0,
        );
    
        self.lives_num = 
        Some( world.create_entity()
            .with(lives_num_transform)
            .with(UiText::new(
                font_handle.clone(),
                "3".to_string(),
                COLOUR_WHITE,
                font_size,
            )).build() );

        let level_text_transform = UiTransform::new(
            "LEVEL_TEXT".to_string(), Anchor::TopLeft,
            100., -50., 1., 200., 50., 0,
        );
    
        self.level_text = 
        Some( world.create_entity()
            .with(level_text_transform)
            .with(UiText::new(
                font_handle.clone(),
                "LEVEL:".to_string(),
                COLOUR_WHITE,
                font_size,
            )).build() );

        
        let level_num_transform = UiTransform::new(
            "LEVEL_NUM".to_string(), Anchor::TopLeft,
            160., -50., 1., 200., 50., 0,
        );
    
        self.level_num = 
        Some( world.create_entity()
            .with(level_num_transform)
            .with(UiText::new(
                font_handle.clone(),
                "1".to_string(),
                COLOUR_WHITE,
                font_size,
            )).build() );

    }

    fn shutdown_ui(&mut self, world: &mut World) {
       

        if let Some(score_entity) = self.score_text{
            world.delete_entity( score_entity ).expect("failed to delete message");
            self.score_text = None;
        }
        
        if let Some(score_entity) = self.score_num{
            world.delete_entity( score_entity ).expect("failed to delete message");
            self.score_num = None;
        }

        if let Some(score_entity) = self.lives_text{
            world.delete_entity( score_entity ).expect("failed to delete message");
            self.lives_text = None;
        }
    
        if let Some(score_entity) = self.lives_num{
            world.delete_entity( score_entity ).expect("failed to delete message");
            self.lives_num = None;
        }

        if let Some(score_entity) = self.level_text{
            world.delete_entity( score_entity ).expect("failed to delete message");
            self.level_text = None;
        }
    
        if let Some(score_entity) = self.level_num{
            world.delete_entity( score_entity ).expect("failed to delete message");
            self.level_num = None;
        }

    }

    fn update_lives(&self, world: &mut World)
    {
        let mut ui_texts = world.write_storage::<UiText>();

        if let Some(e) = self.lives_num {
            if let Some(text) = ui_texts.get_mut( e ) {
                text.text = self.lives.to_string();
            }
        }

    }

    fn update_level(&self, world: &mut World)
    {
        let mut ui_texts = world.write_storage::<UiText>();

        if let Some(e) = self.level_num {
            if let Some(text) = ui_texts.get_mut( e ) {
                text.text = self.level.to_string();
            }
        }
    }

    fn update_score(&self, world: &mut World)
    {
        let score = world.read_resource::<PlayerScore>();
        let mut ui_texts = world.write_storage::<UiText>();

        if let Some(e) = self.score_num {
            if let Some(text) = ui_texts.get_mut( e ) {
                text.text = score.score().to_string();
            }
        }
    }


}



impl SimpleState for GameState{


    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        {
            let mut player_score = world.write_resource::<PlayerScore>();
            player_score.reset();
        }

        self.init_ui(world); 

        self.start_new_level(world);
  
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>)
    {
        let world = data.world;

        delete_game_entities(world);
        
        self.shutdown_ui(world);
    }

    
    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans
    {
        let mut transition = Trans::None;
        let world = data.world;


        self.update_score(world);

        let ship_count = {
            let ships = world.read_storage::<Ship>();
            (&ships).join().count() 
        };

        if ship_count == 0 {
            //we died
            //decrement lives
            self.lives -= 1;
            self.update_lives(world);
            if self.lives <= 0 {

                //if new high score
                    //then transition to name entry
                    //transition = Trans::Switch(Box::new( NameEntryState::new() ));
                //else 
                    //transiition to start screen 
                    transition = Trans::Switch(Box::new( StartState::new() ));
            }
            else{
                //respawn
                delete_game_entities(world);
                self.start_new_level(world);

            }
        }
        else
        {
       
            let rock_count = {
                let rocks = world.read_storage::<Rock>();
                (&rocks).join().count() 
            };
        

            if rock_count == 0 {
            
                self.level += 1;
                self.update_level(world);

                //start new level
                delete_game_entities(world);
                self.start_new_level(world);

            }
        }

        //TODO create flying saucer occasionally
        transition
       
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

//delete all the game entities that we've created
fn delete_game_entities(world: &mut World)
{
    let mut ents_to_delete = Vec::new();
      
    {
        let entities = world.entities();

        let ships = world.read_storage::<Ship>();
        for (ship_entity,_) in (&entities, &ships).join() {
            ents_to_delete.push(ship_entity.clone());
        }

        let rocks = world.read_storage::<Rock>();
        for (rock_entity,_) in (&entities, &rocks).join() {
            ents_to_delete.push(rock_entity.clone());
        }

        let bullets = world.read_storage::<Bullet>();
        for (bullet_entity,_) in (&entities, &bullets).join() {
            ents_to_delete.push(bullet_entity.clone());
        }
    }

    world.delete_entities(ents_to_delete.as_slice()).expect("failed to clean up");
}

