use bevy_ecs::prelude::*;
use macroquad::time::get_frame_time;

use crate::{
    asset_loader::AssetLoader,
    game::{GameState, GameStates},
    transform::{Position, Velocity},
};

const BACKGROUND_TILES_AMOUNT: u32 = 5;
const BACKGROUND_SPEED: Velocity = Velocity { x: -120.0, y: 0.0 };

#[derive(Component)]
pub struct Floor;

#[derive(Bundle)]
pub struct FloorBundle {
    position: Position,
    velocity: Velocity,
    is_type: Floor,
}

impl FloorBundle {
    fn new(x: f32) -> Self {
        FloorBundle {
            position: Position { x, y: -30.0 },
            velocity: BACKGROUND_SPEED,
            is_type: Floor {},
        }
    }
}

pub fn spawn_background(mut commands: Commands, assets: Res<AssetLoader>) {
    let floor_sprite = assets.get_texture("floor");
    for i in 0..BACKGROUND_TILES_AMOUNT {
        commands.spawn(FloorBundle::new((i as f32) * floor_sprite.width()));
    }
}

pub fn update_background(
    mut floor_query: Query<(&mut Position, &Velocity), With<Floor>>,
    assets: Res<AssetLoader>,
    game_state: Res<GameState>,
) {
    let floor_sprite = assets.get_texture("floor");

    if game_state.game_state != GameStates::GameOver {
        for (mut position, velocity) in &mut floor_query {
            position.x += velocity.x * get_frame_time();

            if position.x < -floor_sprite.width() {
                position.x += (BACKGROUND_TILES_AMOUNT as f32) * floor_sprite.width();
            }
        }
    }
}

pub fn move_background() {}
