//! Collision system
//! Test ship and bullets against rocks

use amethyst::{
    ecs::prelude::*,
};

use std::time::{Duration, Instant};
use crate::game_constants::*;
use crate::systems::{
    Bullet,
    Mover,
    Rock,
    Ship, 
};
use crate::maths::*;
use crate::resources;


//////////////////////////////////////////////////////
// Bound  Component - gives bounding radius to entity 
#[derive(Default)]
pub struct Bound
{
    radius: f32,
}

impl Bound{
    pub fn new( radius: f32)->Bound{
        Bound{
            radius,
        }
    }

    pub fn radius(&self)->f32{
        self.radius
    }
}

impl Component for Bound{
    type Storage = DenseVecStorage<Self>;
}


pub struct CollisionSystem;


impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Bullet>,
        ReadStorage<'s, Rock>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Bound>,
        ReadStorage<'s, Mover>,
        Read<'s, resources::PauseFlag>,
    );

    fn run(&mut self, (ents, bullets, rocks, ships, bounds, movers, pause_flag): Self::SystemData) {

        if !pause_flag.is_paused() {

        
            //get ship pos and radius
            //test against each rock
            //if collision want to delete player entity and rock
            //spawn explosion
            for (se, _, mover, bound) in (&*ents, &ships, &movers, &bounds).join() {
                let ship_pos = mover.position();
                let ship_radius = bound.radius();

                for (re, _, rock, rock_bound) in (&*ents, &rocks, &movers, &bounds).join()
                {
                    let rock_pos = rock.position();
                    let rock_radius = rock_bound.radius();

                    let offset = rock_pos - ship_pos;
                    if offset.norm() < (ship_radius + rock_radius) {
                        //collision!
                        ents.delete(re).expect("error deleting entity");
                    }
                }
            }

            //get bullets pos and radius
            //test against each rock
            //if collision want to delete bullet and rock
            //spawn two new rocks of next size down
            //or explosion
            for (be, _, mover, bound) in (&*ents, &bullets, &movers, &bounds).join() {
                let bullet_pos = mover.position();
                let bullet_radius = bound.radius();

                for (re, _, rock, rock_bound) in (&*ents, &rocks, &movers, &bounds).join()
                {
                    let rock_pos = rock.position();
                    let rock_radius = rock_bound.radius();

                    let offset = rock_pos - bullet_pos;
                    if offset.norm() < (bullet_radius + rock_radius) {
                        //collision!
                        ents.delete(re).expect("error deleting entity");
                        ents.delete(be).expect("error deleting entity");;
                    }
                }
            }
        }

    }
}