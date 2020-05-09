use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use amethyst::assets::AssetLoaderSystemData;
use amethyst::core::Transform;

use amethyst::core::math::{Dynamic, Matrix, Vector3, U1, U3};
use amethyst::core::SystemDesc;
use amethyst::ecs::prelude::{Component, DenseVecStorage, SystemData};
use amethyst::renderer::light::Light;
use amethyst::renderer::light::PointLight;
use amethyst::renderer::palette::rgb::{LinSrgba, Rgb};
use amethyst::renderer::rendy::texture::palette::load_from_linear_rgba;
use amethyst::renderer::rendy::util::types::vertex::PosTex;
use amethyst::renderer::rendy::util::types::vertex::{Normal, Position, Tangent, TexCoord};
use amethyst::renderer::shape::Shape;
use amethyst::renderer::Camera;
use amethyst::renderer::{Material, MaterialDefaults, Mesh, Texture};

pub struct GameState;

#[derive(Default)]
pub struct Player {}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub const ARENA_WIDTH: f32 = 1000.;
pub const ARENA_HEIGHT: f32 = 600.;
pub const PLAYER_WIDTH: f32 = 10.;
pub const PLAYER_HEIGHT: f32 = 20.;

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(0.0, 0.0, 1200.0);

            world
                .create_entity()
                .with(Camera::standard_3d(500.0, 500.0))
                .with(transform)
                .build();
        }
        {
            let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
                loader.load_from_data(
                    Shape::Sphere(100, 100)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    (),
                )
            });
            let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();

            let albedo = world.exec(|loader: AssetLoaderSystemData<Texture>| {
                loader.load_from_data(
                    load_from_linear_rgba(LinSrgba::new(1.0, 0.0, 0.0, 1.0)).into(),
                    (),
                )
            });
            let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
                loader.load_from_data(
                    Material {
                        albedo,
                        ..material_defaults
                    },
                    (),
                )
            });
            let mut transform = Transform::default();
            transform.set_translation_xyz(0.0, 0.0, 0.0);
            transform.set_scale(Matrix::from_vec_generic(
                U3,
                U1,
                vec![PLAYER_WIDTH, PLAYER_HEIGHT, 10.],
            ));
            world
                .create_entity()
                .with(mesh)
                .with(Player {})
                .with(material)
                .with(transform)
                .build();
        }

        {
            let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
            let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
                loader.load_from_data(
                    Material {
                        ..material_defaults
                    },
                    (),
                )
            });
            let arena_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
                loader.load_from_data(
                    Shape::Cube
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    (),
                )
            });
            let mut transform = Transform::default();
            transform.set_translation_xyz(0.0, 0.0, 0.0);
            transform.set_scale(Matrix::from_vec_generic(
                U3,
                U1,
                vec![ARENA_WIDTH, ARENA_HEIGHT, 1.],
            ));
            world
                .create_entity()
                .with(arena_mesh)
                .with(material)
                .with(transform)
                .build();
        }
        {
            let light: Light = PointLight {
                intensity: 10.0,
                color: Rgb::new(1.0, 1.0, 1.0),
                ..PointLight::default()
            }
            .into();

            let mut transform = Transform::default();
            transform.set_translation_xyz(-100.0, -100.0, 20.0);

            world.create_entity().with(light).with(transform).build();
        }
    }
}
