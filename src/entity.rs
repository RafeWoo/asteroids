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
use crate::game_constants::{ ARENA_WIDTH, ARENA_HEIGHT, COLOUR_WHITE};

use crate::resources;

pub fn create_rock(world: &mut World , parent: Option<Entity>)->Entity
{
        let sprite_render = 
        {
            let rocks =  world.read_resource::<resources::RocksResource>();

            let mut rng = rand::thread_rng();
            let rock_number = rng.gen_range(0,3);

            SpriteRender {
                sprite_sheet: rocks.sprite_sheet.clone(),
                sprite_number: 0, 
            }
        };

        let mut transform = Transform::default();
        transform.set_xyz(ARENA_HEIGHT * 0.5, ARENA_HEIGHT * 0.5, 0.0);

        let mut builder =  world
        .create_entity()
        .with(sprite_render)
        .with(transform);

        if let Some( entity ) = parent{
            builder = builder.with( Parent{ entity } );
        }
            
        builder.build()
}
