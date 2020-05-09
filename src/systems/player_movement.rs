use amethyst::core::math::{Dynamic, Matrix, Vector3, U1, U3};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::prelude::SystemDesc;

use crate::spell_creator::{Player, ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::core::Transform;

#[derive(SystemDesc)]
pub struct PlayerMovement;

impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let horizontal_movement = input.axis_value("player_horizontal").unwrap_or(0.0);
            let vertical_movement = input.axis_value("player_vertical").unwrap_or(0.0);
            println!("Moving! {}", horizontal_movement);
            if horizontal_movement != 0.0 || vertical_movement != 0.0 {
                println!("Moving! {}", horizontal_movement);
                let translation = Matrix::from_vec_generic(
                    U3,
                    U1,
                    vec![horizontal_movement * 10., vertical_movement * 10., 0.],
                );
                let transTranslation = transform.translation();
                let x = transTranslation + translation;
                transform.set_translation(x);
            }
        }
    }
}
