use bevy_ecs::prelude::*;
use macroquad::audio;
use macroquad::prelude::*;

use crate::{
    asset_loader::AssetLoader,
    game::{GameState, GameStates, SCREEN_SIZE},
    pipe::{Pipe, PIPES_V_SEPARATION, PIPE_WIDTH},
    transform::{Position, Velocity},
};
pub const JUMP_SPEED: f32 = 450.0;
pub const GRAVITY: f32 = 1900.0;
pub const PLAYER_SPRITE_SIZES: Vec2 = Vec2 { x: 36.0, y: 36.0 };
pub const PLAYER_COLLISION: Vec2 = Vec2 { x: 20.0, y: 20.0 };

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub player: Player,
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            position: Position {
                x: 50.0,
                y: -SCREEN_SIZE / 2.0,
            },
            velocity: Velocity { x: 0.0, y: 0.0 },
            player: Player {},
        }
    }
}

pub fn move_player(
    game_state: ResMut<GameState>,
    mut assets: ResMut<AssetLoader>,
    mut query: Query<(&mut Position, &mut Velocity), With<Player>>,
) {
    let flap_sound = assets.get_sound("flap_sound");

    match game_state.game_state {
        GameStates::PressStart => {
            assets.update_animation("player_animations");
        }
        GameStates::Play => {
            for (mut position, mut velocity) in &mut query {
                position.y += velocity.y * get_frame_time();

                if is_key_pressed(KeyCode::Space) {
                    velocity.y = -JUMP_SPEED;
                    audio::play_sound_once(flap_sound);
                }
            }
            assets.update_animation("player_animations");
        }
        _ => {}
    }
}

pub fn add_gravity(game_state: Res<GameState>, player_query: Single<&mut Velocity, With<Player>>) {
    if game_state.game_state == GameStates::Play {
        let mut velocity = player_query.into_inner();
        velocity.y += GRAVITY * get_frame_time();
    }
}

pub fn collide_player(
    mut game_state: ResMut<GameState>,
    player_query: Single<&Position, With<Player>>,
    mut pipe_query: Query<(Entity, &Position, &mut Pipe)>,
    assets: Res<AssetLoader>,
) {
    let hit_sound = assets.get_sound("hit_sound");
    let score_sound = assets.get_sound("score_sound");

    if game_state.game_state == GameStates::Play {
        let player_position = player_query.into_inner();

        for (_entity, pipe_position, mut pipe) in &mut pipe_query {
            let player_center = Vec2 {
                x: player_position.x + PLAYER_SPRITE_SIZES.x / 2.0,
                y: player_position.y + PLAYER_SPRITE_SIZES.y / 2.0,
            };

            let (x_min, x_max): (f32, f32) = (pipe_position.x, pipe_position.x + PIPE_WIDTH);

            let (y_min, y_max): (f32, f32) =
                (pipe_position.y, pipe_position.y + PIPES_V_SEPARATION);

            //pipe collision with pipes and floor
            if player_center.x - PLAYER_COLLISION.x / 2.0 < x_max
                && player_center.x + PLAYER_COLLISION.x / 2.0 > x_min
                && !(player_center.y - PLAYER_COLLISION.y / 2.0 > y_min
                    && player_center.y + PLAYER_COLLISION.y / 2.0 < y_max - 10.0)
                || player_center.y + PLAYER_COLLISION.y / 2.0 > -36.0
            {
                game_state.game_state = GameStates::GameOver;
                audio::play_sound_once(hit_sound);
                println!("{} In pipe bounds", pipe_position.x)
            }

            //if player passes through pipe, score point
            if player_center.x > x_max && pipe.can_score {
                pipe.can_score = false;
                game_state.score += 1;
                audio::play_sound_once(score_sound);
                println!("Scored a point: {}", game_state.score);
            }
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::new());
}
