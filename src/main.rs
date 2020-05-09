use amethyst::input::{InputBundle, StringBindings};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use amethyst::assets::AssetLoaderSystemData;
use amethyst::core::Transform;

use amethyst::renderer::rendy::util::types::vertex::PosTex;
use amethyst::renderer::rendy::util::types::vertex::{Normal, Position, Tangent, TexCoord};
use amethyst::renderer::shape::Shape;
use amethyst::renderer::Camera;
use amethyst::renderer::{Material, MaterialDefaults, Mesh};
mod spell_creator;
mod systems;
use spell_creator::GameState;
use systems::PlayerMovement;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config_path = config_dir.join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config_path)?,
        )?
        // .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(TransformBundle::new())?
        .with(PlayerMovement, "PlayerMovement", &[]);

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
