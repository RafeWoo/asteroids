//! Lifetime system 
//! Kills entities with expired Lifetime component
//! 
use amethyst::{
    core::timing::Time,
    ecs::prelude::*,
};

use std::time::{Duration, Instant};
use std::vec::Vec;



////////////////////////////////////////////////////////////////////

pub struct Lifetime{
    lifetime : u64,
    life_start : Instant,
} 

impl Lifetime{
    pub fn new(lifetime: u64)->Lifetime
    {
        Lifetime{
            lifetime,
            life_start: Instant::now(),
        }
    }

    pub fn is_expired(&self)->bool
    {
        Instant::now().duration_since( self.life_start ) >= Duration::from_millis(self.lifetime)
    }
}
impl Component for Lifetime{
    type Storage = DenseVecStorage<Self>;
}

////////////////////////////////////////////////////////////////////


pub struct LifetimeSystem;


impl<'s> System<'s> for LifetimeSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Lifetime>,
    );

    fn run(&mut self, (mut ents, lifetimes): Self::SystemData) {

        let mut ents_to_delete = Vec::new();

        for (e, lifetime) in (&*ents, &lifetimes).join() {

            if lifetime.is_expired() {
                ents_to_delete.push( e.clone() );
            }
        }

        for e in ents_to_delete{
            ents.delete(e);
        }
       
    }
}

