//! moving system
//! responsible for updating mover components

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::*,
};


use crate::maths::*;
use crate::game_constants::*;
use crate::resources::PauseFlag;


/////////////////////////////////////////////////////////////////////////////
pub struct Mover
{
    pos: Point2,
    vel: Vector2,
    angle_rad: f32,     //direction facing
    rot_vel: f32,       // rads per second
}

impl Mover{
    pub fn new(x: f32, y: f32)->Mover{
        Mover{
            pos: Point2::new(x,y),
            vel: Vector2::new(0.0,0.0),
            angle_rad: 0.0,
            rot_vel: 0.0,
        }
    }

    pub fn with_velocity(self, vel: Vector2)->Mover{
        Mover{
            pos: self.pos,
            vel, 
            angle_rad: self.angle_rad,
            rot_vel: self.rot_vel,
        }
    }

    pub fn with_orientation(self, angle: f32)->Mover{
        Mover{
            pos: self.pos,
            vel: self.vel,
            angle_rad: angle,
            rot_vel: self.rot_vel,
        }
    }

    pub fn with_rot_velocity(self, rot_vel: f32)->Mover{
        Mover{
            pos: self.pos,
            vel: self.vel,
            angle_rad: self.angle_rad,
            rot_vel,
        }
    }

    pub fn inc_orientation(&mut self, angle:f32)
    {
       self.angle_rad = angle_clamp( self.angle_rad + angle );
    }

    pub fn accelerate_forwards(&mut self, acceleration: f32)
    {
        let acc_vec = vector_from_angle( self.angle_rad) * acceleration;
        self.vel += acc_vec;
    }

    pub fn orientation(&self)->f32{
        self.angle_rad
    }

    pub fn position(&self)->Point2
    {
        self.pos
    }

    //pub fn velocity(&self)->Vector2{
    //    self.vel
    //}
}

impl Component for Mover {
    type Storage = DenseVecStorage<Self>;
}

//////////////////////////////////////////////////////////////////
// Wrapper component indicates that object should be wrapped on screen
#[derive(Default)]
pub struct Wrapper;

impl Component for Wrapper {
    type Storage = NullStorage<Self>;
}

//////////////////////////////////////////////////////////////////

pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Mover>,
        Read<'s, Time>,
        Read<'s, PauseFlag>,
    );



    fn run(&mut self, (mut movers, time, pause_flag): Self::SystemData) {
        
        if !pause_flag.is_paused() {

            let delta = time.delta_seconds();
            
            for mover in (&mut movers).join() {

                //update and clamp orientation
                mover.angle_rad = angle_clamp( mover.angle_rad + mover.rot_vel * delta);
                
                //update and clamp position
                let speed = mover.vel.norm_squared();
                if speed > MAX_SPEED * MAX_SPEED {
                    mover.vel = (mover.vel / speed.sqrt()) * MAX_SPEED;
                }
                let offset =  mover.vel * delta;
                mover.pos += offset;

            }
        }
        
    }
}

//////////////////////////////////////////////////////////////////
/// 
pub struct UpdateSystem;

impl<'s> System<'s> for UpdateSystem {
    type SystemData = (
        ReadStorage<'s, Mover>,
        WriteStorage<'s, Transform>,
        Read<'s, PauseFlag>,
    );

 
    //Copy the Mover pos and orientation to the Transform
    fn run(&mut self, ( movers, mut locals, pause_flag): Self::SystemData) {
       
        if !pause_flag.is_paused() {

            for (mover, local) in (&movers, &mut locals).join() {

                local.set_xyz( mover.pos.x, mover.pos.y , 0.0);
                local.set_rotation_euler(0.0, 0.0, mover.angle_rad);
        
            }
        }
    }
}

//////////////////////////////////////////////////////////////////


pub struct WrapSystem;

impl<'s> System<'s> for WrapSystem {
    type SystemData = (
        ReadStorage<'s, Wrapper>,
        WriteStorage<'s, Mover>,
        Read<'s, PauseFlag>,
    );

    //Copy the Mover pos and orientation to the Transform
    fn run(&mut self, ( wrappers, mut movers, pause_flag): Self::SystemData) {
       
        if !pause_flag.is_paused() {

            for (_, mover) in (&wrappers, &mut movers).join() {

                if mover.pos.x < 0.0 {
                    mover.pos.x += ARENA_WIDTH;
                }
                else if mover.pos.x > ARENA_WIDTH {
                    mover.pos.x -= ARENA_WIDTH;
                }

                if mover.pos.y < 0.0 {
                    mover.pos.y += ARENA_HEIGHT;
                }
                else if mover.pos.y > ARENA_HEIGHT {
                    mover.pos.y -= ARENA_HEIGHT;
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////////////