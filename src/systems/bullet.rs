//! Bullet system
//! 
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::*,
    input::InputHandler,
    renderer::SpriteRender,
};
use std::time::{Duration, Instant};
use crate::game_constants::*;
use crate::systems;
use crate::maths::*;
use crate::resources;

//////////////////////////////////////////////////////////
pub struct Shooter{
    last_shot : Instant,
}

impl Shooter{

    pub fn new() ->Shooter{
        Shooter{
            last_shot : Instant::now(),
        }
    }

    pub fn can_shoot( &self) -> bool
    {
        Instant::now().duration_since( self.last_shot ) > Duration::from_millis(PLAYER_SHOT_TIME_MS) 
    }

    pub fn on_shot_fired(&mut self)
    {
        self.last_shot = Instant::now();
    }
}

impl Component for Shooter{
    type Storage = DenseVecStorage<Self>;
}

//////////////////////////////////////////////////////////
#[derive(Default)]
pub struct Bullet;

impl Component for Bullet{
    type Storage = NullStorage<Self>;
}

//////////////////////////////////////////////////////////
pub struct ShooterSystem;


impl<'s> System<'s> for ShooterSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, systems::Mover>,
        WriteStorage<'s, Shooter>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Bullet>,
        WriteStorage<'s, systems::Wrapper>,
        WriteStorage<'s, systems::Lifetime>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, resources::BulletResource>,
    );

    fn run(&mut self, 
        (mut ents, mut movers, mut shooters, mut transforms, mut sprites, mut bullets, mut wrappers, mut lifetimes,
    input, bullet_resource): Self::SystemData) 
    {

        if let Some(fire) = input.action_is_down("fire") {

            if fire {
            
                let mut shot_fired = false;
                let mut angle = 0.0f32;
                let mut pos = Point2::new(0.0, 0.0);
                let mut vel = Vector2::new(0.0, 0.0);

                for (mover, shooter) in (&mut movers, &mut shooters).join() {

                    if shooter.can_shoot() {

                        angle = mover.orientation();
                        pos = mover.position();
                        vel = vector_from_angle(angle) * BULLET_SPEED;
                      
                       shot_fired = true;
                       shooter.on_shot_fired();
                       break;
                    }
                }

                if shot_fired {
                 //spawn_shot
                    spawn_bullet( pos, vel, angle,
                        &bullet_resource,
                        &mut ents, 
                        &mut transforms,
                        &mut sprites,
                        &mut movers,
                        &mut bullets,
                        &mut wrappers,
                        &mut lifetimes,
                        );
                }
            }
        }
    }
}


fn spawn_bullet<'s>( pos: Point2, vel: Vector2, angle: f32,
    bullet_resource: &resources::BulletResource,
    ents :&mut Entities<'s>, 
    mut transforms: &mut  WriteStorage<'s, Transform>,
    mut sprites: &mut  WriteStorage<'s, SpriteRender>,
    mut movers: &mut WriteStorage<'s, systems::Mover>,
    mut bullets: &mut WriteStorage<'s, Bullet>,
    mut wrappers: &mut WriteStorage<'s, systems::Wrapper>,
    mut lifetimes: &mut WriteStorage<'s, systems::Lifetime>,
    ) ->Entity
{

    let sprite_render = 
        {
            let bullet_number = 0;

            SpriteRender {
                sprite_sheet: bullet_resource.sprite_sheet.clone(),
                sprite_number: bullet_number, 
            }
        };

    let mut transform = Transform::default();
    transform.set_xyz( pos.x, pos.y, 0.0);


    let mover = systems::Mover::new( pos.x, pos.y )
                    .with_velocity( vel )
                    .with_orientation(angle);
    
    ents.build_entity()
        .with( transform, &mut transforms)
        .with( sprite_render, &mut sprites)
        .with( mover, &mut movers)
        .with( Bullet, &mut bullets)
        .with( systems::Wrapper, &mut wrappers)
        .with( systems::Lifetime::new(BULLET_LIFETIME_MS), &mut lifetimes)
        .build()
}
//////////////////////////////////////////////////////////

pub struct BulletSystem;