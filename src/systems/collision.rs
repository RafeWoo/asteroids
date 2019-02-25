//! Collision system
//! Test ship and bullets against rocks

use amethyst::{
    ecs::prelude::*,
    core::transform::Transform,
    renderer::SpriteRender, 
};


use crate::game_constants::*;
use crate::systems::{
    Bullet,
    Mover,
    Rock,
    Ship,
    Wrapper,
};
use crate::maths::*;
use crate::resources;

use rand::Rng;
use std::vec::Vec;


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
        ReadStorage<'s, Ship>,
        WriteStorage<'s, Rock>,
        WriteStorage<'s, Bound>,
        WriteStorage<'s, Mover>,
        WriteStorage<'s, Wrapper>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, resources::RocksResource>,
        Read<'s, resources::PauseFlag>,
        Write<'s, resources::PlayerScore>,
    );

    fn run(&mut self, (mut ents, bullets, ships, mut rocks, mut bounds, mut movers, mut wrappers, mut sprites, mut transforms, 
    rock_resource, pause_flag, mut score): Self::SystemData) {

        if !pause_flag.is_paused() {

        
            //get ship pos and radius
            //test against each rock
            //if collision want to delete player entity and rock
            //TODO spawn explosion
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
                        ents.delete(se).expect("error deleting entity");
                        ents.delete(re).expect("error deleting entity");
                    }
                }
            }


            let mut rocks_to_spawn = Vec::new();
            //get bullets pos and radius
            //test against each rock
            //if collision want to delete bullet and rock
            //spawn two new rocks of next size down
            //or explosion for smallest rock
            for (be, _, mover, bound) in (&*ents, &bullets, &movers, &bounds).join() {
                let bullet_pos = mover.position();
                let bullet_radius = bound.radius();

                let bullet_angle = mover.orientation();
                let angle_offset = 0.6;

                for (re, rock_type, rock, rock_bound) in (&*ents, &rocks, &movers, &bounds).join()
                {
                    let rock_pos = rock.position();
                    let rock_radius = rock_bound.radius();

                    let offset = rock_pos - bullet_pos;
                    if offset.norm() < (bullet_radius + rock_radius) {
                        //collision!
                        ents.delete(re).expect("error deleting entity");
                        ents.delete(be).expect("error deleting entity");

                        //spawn two new rocks of next size down
                        match rock_type {
                            Rock::Big => {
                                score.add_score(50);
                                rocks_to_spawn.push( (rock_pos, bullet_angle + angle_offset, Rock::Medium) );
                                rocks_to_spawn.push( (rock_pos, bullet_angle - angle_offset, Rock::Medium) );
                            },

                            Rock::Medium => {
                                score.add_score(100);
                                rocks_to_spawn.push( (rock_pos, bullet_angle + angle_offset, Rock::Small) );
                                rocks_to_spawn.push( (rock_pos, bullet_angle - angle_offset, Rock::Small) );
                            },

                            Rock::Small => {
                                score.add_score(200);
                            },
                        }
                       
                    }
                }
            }

            for (pos, angle, rock_type) in rocks_to_spawn {
                spawn_rock(
                            pos,
                            angle,
                            rock_type,
                            &rock_resource,
                            &mut ents, 
                            &mut transforms,
                            &mut sprites,
                            &mut movers,
                            &mut rocks,
                            &mut wrappers,
                            &mut bounds,
                        );
            }
        }
    }
}


fn spawn_rock<'s>( pos: Point2, v_angle: f32,
    rock_type: Rock,
    rock_resource: &resources::RocksResource,
    ents :&mut Entities<'s>, 
    mut transforms: &mut  WriteStorage<'s, Transform>,
    mut sprites: &mut  WriteStorage<'s, SpriteRender>,
    mut movers: &mut WriteStorage<'s, Mover>,
    mut rocks: &mut WriteStorage<'s, Rock>,
    mut wrappers: &mut WriteStorage<'s, Wrapper>,
    mut bounds: &mut WriteStorage<'s, Bound>,
    ) ->Entity
{

    let mut rng = rand::thread_rng();

    let sprite_render = 
    {
        let (lo,hi) = match rock_type{
            Rock::Big => (0,3),
            Rock::Medium => (3,6),
            Rock::Small => (6,10),
        };
        //big rock numbers are 0,1,2
        //med rock numbers are 3,4,5
        //small rock numbers are 6, 7, 8, 9

        let rock_number = rng.gen_range(lo,hi);

        SpriteRender {
            sprite_sheet: rock_resource.sprite_sheet.clone(),
            sprite_number: rock_number, 
        }
    };


    let mut transform = Transform::default();
    transform.set_xyz( pos.x, pos.y, 0.0);

    let (min_speed, max_speed) = match rock_type
    {
        Rock::Big => (5.0, 50.0),
        Rock::Medium => (20.0, 100.0),
        Rock::Small => (40.0,150.0),
    };

    let speed = rng.gen_range( min_speed, max_speed);
    let vel = vector_from_angle( v_angle ) * speed;
    let rot_vel = rng.gen_range( 0.1, 0.5);
    let mover = Mover::new( pos.x, pos.y)
                    .with_velocity( vel )
                    .with_orientation(v_angle)
                    .with_rot_velocity(rot_vel);

    let rock_size = match rock_type {
        Rock::Big => ROCK_RADIUS_BIG,
        Rock::Medium => ROCK_RADIUS_MEDIUM,
        Rock::Small => ROCK_RADIUS_SMALL,
    };

    ents.build_entity()
        .with( sprite_render, &mut sprites)
        .with( transform, &mut transforms)
        .with( mover, &mut movers)
        .with( Wrapper, &mut wrappers )
        .with( rock_type, &mut rocks )
        .with( Bound::new( rock_size ), &mut bounds )
        .build()
}