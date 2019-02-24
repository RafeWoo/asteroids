//! Entity creation functions
//! 

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::{Transform, Parent,},
    ecs::prelude::{Component, DenseVecStorage,Entity},
    input::is_key_down,
    prelude::*,
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,VirtualKeyCode,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontHandle},
};

use rand::{thread_rng, Rng};
use crate::game_constants::*;

use crate::resources;
use crate::systems;
use crate::maths::*;

pub fn create_rock(world: &mut World , parent: Option<Entity>)->Entity
{
    let mut rng = rand::thread_rng();

        let sprite_render = 
        {
            let rocks =  world.read_resource::<resources::RocksResource>();

            
            let rock_number = rng.gen_range(0,3);

            SpriteRender {
                sprite_sheet: rocks.sprite_sheet.clone(),
                sprite_number: rock_number, 
            }
        };


        //choose random direction
        let angle = rng.gen_range(-PI, PI);
        let distance = EXCLUSION_ZONE_RADIUS + rng.gen_range(0.0, 200.0);
        let vec = vector_from_angle(angle) * distance;


        let mut transform = Transform::default();
        let x_pos = vec.x;
        let y_pos = vec.y;
        transform.set_xyz(x_pos, y_pos, 0.0);

        let speed = rng.gen_range( MIN_ROCK_SPEED, MAX_SPEED);
        let v_angle = rng.gen_range( -PI, PI);
        let vel = vector_from_angle( v_angle ) * speed;
        let mover = systems::Mover::new( x_pos, y_pos).with_velocity( vel );

        let mut builder =  world
            .create_entity()
            .with(sprite_render)
            .with(transform)
            .with( mover )
            .with( systems::Wrapper );

        if let Some( entity ) = parent{
            builder = builder.with( Parent{ entity } );
        }
            
        builder.build()
}
