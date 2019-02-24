//! Top level systems file
//! 
mod moving;
mod ship;
mod bullet;
mod lifetime;
mod collision;

pub use self::moving::{ Mover, MoveSystem, UpdateSystem, Wrapper, WrapSystem };
pub use self::ship::{ Ship, ShipSystem };
pub use self::bullet::{ Bullet, BulletSystem, Shooter, ShooterSystem,};
pub use self::lifetime::{ Lifetime, LifetimeSystem, };
pub use self::collision::{ Bound, CollisionSystem, };


use amethyst::ecs::prelude::*;

//////////////////////////////////////////////////////
// Rock Tag Component
#[derive(Default)]
pub struct Rock;

impl Component for Rock{
    type Storage = NullStorage<Self>;
}

