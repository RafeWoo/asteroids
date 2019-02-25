//! Lifetime system 
//! Kills entities with expired Lifetime component
//! 
use amethyst::{
    core::timing::Time,
    ecs::prelude::*,
};

use std::time::{Duration};
use std::vec::Vec;

use crate::resources::PauseFlag;

////////////////////////////////////////////////////////////////////

pub struct Lifetime{
    lifetime : u64,
    life_total : Duration,
} 

impl Lifetime{
    pub fn new(lifetime: u64)->Lifetime
    {
        Lifetime{
            lifetime,
            life_total: Duration::new(0,0),
        }
    }

    pub fn is_expired(&self)->bool
    {
        self.life_total >= Duration::from_millis(self.lifetime)
    }

    pub fn age_by(&mut self, time: Duration)
    {
        self.life_total += time;
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
        WriteStorage<'s, Lifetime>,
        Read<'s, Time>,
        Read<'s, PauseFlag>,
    );

    fn run(&mut self, (ents, mut lifetimes, time, pause_flag): Self::SystemData) {

        if !pause_flag.is_paused() {

            let delta_time = time.delta_time();
            let mut ents_to_delete = Vec::new();

            for (e, lifetime) in (&*ents, &mut lifetimes).join() {

                lifetime.age_by(delta_time);

                if lifetime.is_expired() {
                    ents_to_delete.push( e.clone() );
                }
            }

            for e in ents_to_delete{
                let _result = ents.delete(e);

            }
        }
       
    }
}

