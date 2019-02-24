//! PauseState is entered when player has paused gameplay
//! 
//! Can go to gameplay state
//! Waiting for player to press unpause key
//! 
//! Display "Paused" on Screen
//! set a paused flag
use amethyst::{
    ecs::prelude::*,
    input::is_key_down,
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontHandle},
};


use crate::game_constants::*;
use crate::resources;

pub struct PauseState
{
    message: Option<Entity>,
}

impl PauseState
{
    pub fn new() -> PauseState
    {
        PauseState{
            message: None,
        }
    }
}

fn display_pause_message(world: &mut World)->Entity
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
            "PAUSED".to_string(),
            COLOUR_WHITE,
            50.,
        )).build()
}

impl SimpleState for PauseState
{

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //set global pause flag
        world.write_resource::<resources::PauseFlag>().toggle_paused();

        //display paused text
        self.message = Some( display_pause_message(world));

      
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>)
    {
        let world = data.world;
        //remove paused text
        if let Some(message_entity) = self.message {
            world.delete_entity( message_entity ).expect("failed to delete message");
            self.message = None;
        }
        //unset global pause flag
        world.write_resource::<resources::PauseFlag>().toggle_paused();
        
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans 
    {
        let mut transition = Trans::None;

        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                
                transition = Trans::Pop;
            }
        }
        
        transition
    }
}