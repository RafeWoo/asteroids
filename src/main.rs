#![deny(missing_docs)]

//! an Asteroids clone
//! 
mod states;
mod game_constants;
mod entity;
mod resources;
mod systems;
mod maths;
 
use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    input::InputBundle,
    renderer::{ColorMask, DisplayConfig, DrawFlat2D, Pipeline,  RenderBundle, Stage, ALPHA},
    ui::{DrawUi, UiBundle},
    //utils::application_root_dir,
};



fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    //let root_dir = application_root_dir();
    let path_buf = std::env::current_dir().expect("get thing failed");
    let root_dir = path_buf.to_str().unwrap().to_string();

    //create the render pipeline
    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass( DrawFlat2D::new().with_transparency( ColorMask::all(), ALPHA, None ) )
            .with_pass(DrawUi::new())
        );

     //Set up global shared game data
    let path = format!(
        "{}/resources/display_config.ron",
        root_dir
    );
    let display_config = DisplayConfig::load(&path);

    let binding_path = format!("{}/resources/bindings_config.ron", root_dir );
    let input_bundle = InputBundle::<String, String>::new()
    .with_bindings_from_file(binding_path)?;

    let game_data =
        GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
            .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::ShipSystem, "ship_system", &[])
        .with(systems::ShooterSystem, "shooter_system", &[])
        .with(systems::LifetimeSystem, "lifetime_system", &[])
        .with(systems::MoveSystem, "move_system", &[])
        .with(systems::WrapSystem, "wrap_system", &["move_system"])
        .with(systems::UpdateSystem, "update_system", &["move_system", "wrap_system"])
        .with(systems::CollisionSystem, "collision_system", &["update_system"])
        ;
   

    //create the application
    let mut game = Application::new("./", states::LoadingState::default(), game_data)?;

    //start the game loop
    game.run();

    Ok(())
}
