//! LoadingState 
//! Loads the global persistant resources for the game
//!  
//! goes to the StartState
//! 


use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage,Entity},
    prelude::*,
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontHandle},
};
use std::time::{Duration, Instant};

use crate::game_constants::{ ARENA_WIDTH, ARENA_HEIGHT, COLOUR_WHITE};
use crate::states::StartState;

pub struct LoadingState
{
    start_time: Instant,
    loading_screen_handle: Option<Entity>,
}


impl Default for LoadingState{
    fn default()->Self
    {
        LoadingState{
            start_time: Instant::now(),
            loading_screen_handle: None,
        }
    }
}


impl SimpleState for LoadingState{

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;


        self.loading_screen_handle = Some(load_loading_screen(world));
       
        load_font(world);
        load_rock_sprites(world);
        initialise_camera(world);
        initialise_ui(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>){

        if let Some(entity) = self.loading_screen_handle {
            data.world.delete_entity( entity ).expect("failed to delete loading screen");
            self.loading_screen_handle = None;
        }
       
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans
    {
        let mut transition = Trans::None;
 
        if Instant::now().duration_since( self.start_time ) > Duration::from_secs(1)
        {
            transition = Trans::Switch( Box::new( StartState::new() ) );
        }

        transition
    }
}

//movr to separate module Resources
pub struct RockSpriteSheet
{
    pub sprite_sheet : SpriteSheetHandle,
}


fn load_font(world: &mut World)
{
     let font_handle = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    world.add_resource( font_handle );
}

fn load_sprite_sheet(world: &mut World, name: &str)->SpriteSheetHandle
{

    let texture_name = format!("texture/{}.png", name);
    let sheet_name = format!("texture/{}.ron", name);

    let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        texture_name,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &texture_storage,
    )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        sheet_name, 
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

fn load_rock_sprites(world: &mut World)
{ 
    let sprite_sheet = load_sprite_sheet(world, "rocks");
    
    world.add_resource( RockSpriteSheet{sprite_sheet} );
}

fn load_loading_screen(world: &mut World)->Entity
{
    let sprite_sheet = load_sprite_sheet(world, "loading");
    
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0, 
    };

    let mut screen_transform = Transform::default();
    screen_transform.set_xyz(ARENA_HEIGHT * 0.5, ARENA_HEIGHT * 0.5, 0.0);
  

    // Create and return the loading screen entity
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(screen_transform)
        .build()
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}


//MOve to game state
//need lives as well
/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub player_score: Entity,
}
/// Initialises a ui scoreboard
fn initialise_ui(world: &mut World) {
    let font_handle = world.read_resource::<FontHandle>().clone();


    let score_transform = UiTransform::new(
        "SCORE".to_string(), Anchor::TopMiddle,
        -50., -50., 1., 200., 50., 0,
    );
  

    let player_score = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(
            font_handle,
            "0".to_string(),
            COLOUR_WHITE,
            50.,
        )).build();

    world.add_resource(ScoreText { player_score });
}

/*




//Create a ScoreBoard resource
// ScoreEntry:10
// lowest score 

//create a default scoreboard
//functionality to insert new entry

//score entry 
3 chars and a score


*/


