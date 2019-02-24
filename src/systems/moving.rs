//! moving system
//! responsible for updating mover components

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::*,
};

use std::f32::consts::PI;

use crate::maths::*;

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
            rot_vel: 0.4,
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
}

impl Component for Mover {
    type Storage = DenseVecStorage<Self>;
}

pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Mover>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );



    fn run(&mut self, (mut movers, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.

        let delta = time.delta_seconds();
        for (mover, local) in (&mut movers, &mut locals).join() {

            //update and clamp orientation
            mover.angle_rad = angle_clamp( mover.angle_rad + mover.rot_vel * delta);
            
            //todo clamp speed to maximum
            let offset =  mover.vel * delta;
            mover.pos += offset;

            local.set_xyz( mover.pos.x, mover.pos.y , 0.0);
            local.set_rotation_euler(0.0, 0.0, mover.angle_rad);
           // local.translate_x(ball.velocity[0] * time.delta_seconds());
           // local.translate_y(ball.velocity[1] * time.delta_seconds());
           // local.roll_local( time.delta_seconds() );
        }
    }
}

