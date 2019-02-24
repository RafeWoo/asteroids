//! Ship System
//! 
use amethyst::{
    core::timing::Time,
    ecs::prelude::*,
    input::InputHandler,
    renderer::SpriteRender,
};


use crate::systems::moving::Mover;
use crate::game_constants::*;

#[derive(Default)]
pub struct Ship;

impl Component for Ship{
    type Storage = NullStorage<Self>;
}


///
/// 
pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Mover>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Ship>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut movers, mut sprites, ships, input, time): Self::SystemData) {

        let time_delta:f32 = time.delta_seconds();

        if let Some(rotation) = input.axis_value("rotate") {

            let rotate_amount = rotation as f32 * time_delta * PLAYER_ROT_SPEED;
            for (mover, _) in (&mut movers, &ships).join() {

                mover.inc_orientation( rotate_amount );

            }
        }

        if let Some( thrust ) = input.axis_value("thrust") {
    
            
            let impulse = thrust as f32; 
            if impulse > 0.0 {
                let acc = impulse * time_delta * PLAYER_ACC;

                for (mover, _) in (&mut movers, &ships).join() {
                    mover.accelerate_forwards( acc );
                }

                //show thrusters
                for (sprite, _) in (&mut sprites, &ships).join() {
                    sprite.sprite_number = SHIP_SPRITE_FLAME;
                }
            }
            else
            {
                //hide thrusters
                for (sprite, _) in (&mut sprites, &ships).join() {
                    sprite.sprite_number = SHIP_SPRITE;
                }
            }
  
        }
    }
}