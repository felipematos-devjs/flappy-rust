use crate::asset_loader::AssetLoader;
use crate::pipe::{
    Pipe, PIPES_H_SEPARATION, PIPES_V_SEPARATION, PIPE_CEILING_PADDING, PIPE_FLOOR_PADDING,
};
use crate::player::Player;
use crate::transform::{Position, Velocity};
use bevy_ecs::prelude::*;
use macroquad::prelude::*;

pub const SCREEN_SIZE: f32 = 320.0;

#[derive(Default, Component, Debug, PartialEq)]
pub enum GameStates {
    Play,
    Paused,
    GameOver,
    #[default]
    PressStart,
    Restart,
}

#[derive(Default, Resource, Debug)]
pub struct GameState {
    pub game_state: GameStates,
    pub score: u32,
}

pub fn check_inputs(mut game_state: ResMut<GameState>) {
    if is_key_pressed(KeyCode::Space) && game_state.game_state == GameStates::PressStart {
        game_state.score = 0;
        game_state.game_state = GameStates::Play;
    } else if is_key_pressed(KeyCode::R) && game_state.game_state == GameStates::GameOver {
        game_state.score = 0;
        game_state.game_state = GameStates::Restart;
    }
}

pub fn restart_game(
    mut game_state: ResMut<GameState>,
    mut player_query: Single<(&mut Position, &mut Velocity), (With<Player>, Without<Pipe>)>,
    mut pipe_query: Query<(Entity, &mut Position, &mut Pipe), (With<Pipe>, Without<Player>)>,
    mut assets: ResMut<AssetLoader>,
) {
    if game_state.is_changed() && game_state.game_state == GameStates::Restart {
        //reset pipes
        {
            let mut current_pipe = 0.0;
            for (_entity, mut position, mut pipe) in &mut pipe_query {
                let random: f32 = macroquad::rand::gen_range(0.0, 1.0);
                position.x = SCREEN_SIZE + current_pipe * PIPES_H_SEPARATION;
                position.y = clamp(
                    random * (-SCREEN_SIZE),
                    -SCREEN_SIZE + PIPE_CEILING_PADDING,
                    -PIPES_V_SEPARATION - PIPE_FLOOR_PADDING,
                );
                current_pipe += 1.0;
                pipe.can_score = true;
            }
        }

        {
            let (mut position, mut velocity) = player_query.into_inner();

            position.x = 50.0;
            position.y = -SCREEN_SIZE / 2.0;

            velocity.x = 0.0;
            velocity.y = 0.0;
        }

        game_state.game_state = GameStates::PressStart;
        assets.set_animation("player_animations", rand::RandomRange::gen_range(0, 3));
    }
}
