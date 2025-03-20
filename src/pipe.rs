use bevy_ecs::prelude::*;
use macroquad::{prelude::clamp, time::get_frame_time};

use crate::transform::{Position, Velocity};
use crate::{
    asset_loader::AssetLoader,
    game::{GameState, GameStates, SCREEN_SIZE},
};

#[derive(Component)]
pub struct Pipe {
    pub can_score: bool,
}

pub const OBSTACLE_AMOUNT: u32 = 5;
pub const PIPES_V_SEPARATION: f32 = 110.0;
pub const PIPES_H_SEPARATION: f32 = 150.0;
pub const PIPE_WIDTH: f32 = 52.0;
pub const PIPE_FLOOR_PADDING: f32 = 65.0;
pub const PIPE_CEILING_PADDING: f32 = 20.0;

#[derive(Bundle)]
pub struct PipeBundle {
    position: Position,
    velocity: Velocity,
    is_type: Pipe,
}

impl PipeBundle {
    pub fn new(x: f32, y: f32) -> Self {
        PipeBundle {
            position: Position { x, y },
            velocity: Velocity { x: -120.0, y: 0.0 },
            is_type: Pipe { can_score: true },
        }
    }
}

pub fn move_pipe(
    game_state: Res<GameState>,
    mut query: Query<(&mut Position, &Velocity, &mut Pipe)>,
    assets: Res<AssetLoader>,
) {
    let pipe_texture = assets.get_texture("pipe");
    if game_state.game_state == GameStates::Play {
        for (mut position, velocity, mut pipe) in &mut query {
            position.x += velocity.x * get_frame_time();
            position.y += velocity.y * get_frame_time();

            if position.x < -pipe_texture.width() {
                let random: f32 = macroquad::rand::gen_range(0.0, 1.0);
                position.x += (OBSTACLE_AMOUNT as f32) * PIPES_H_SEPARATION;
                position.y = clamp(
                    random * (-SCREEN_SIZE),
                    -SCREEN_SIZE + PIPE_CEILING_PADDING,
                    -PIPES_V_SEPARATION - PIPE_FLOOR_PADDING,
                );
                pipe.can_score = true;
            }
        }
    }
}

pub fn spawn_pipes(mut commands: Commands) {
    for i in 0..OBSTACLE_AMOUNT {
        let random: f32 = macroquad::rand::gen_range(0.0, 1.0);

        commands.spawn(PipeBundle::new(
            SCREEN_SIZE + (i as f32) * PIPES_H_SEPARATION,
            clamp(
                random * (-SCREEN_SIZE),
                -SCREEN_SIZE + PIPE_CEILING_PADDING,
                -PIPES_V_SEPARATION - PIPE_FLOOR_PADDING,
            ),
        ));
    }
}
