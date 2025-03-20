use std::f32::consts::PI;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;
use crate::background::Floor;
use crate::game::{GameState, GameStates, SCREEN_SIZE};
use crate::pipe::{Pipe, PIPES_V_SEPARATION};
use crate::player::Player;
use crate::transform::{Position, Velocity};
use crate::ui::{Score, UI_SPRITE_SEP, UI_SPRITE_SIZE};

pub fn draw_call(
    floor_query: Query<(Entity, &Position), With<Floor>>,
    pipe_query: Query<(Entity, &Position), With<Pipe>>,
    player_query: Single<(&Position, &Velocity), With<Player>>,
    score_query: Single<(Entity, &Position), With<Score>>,
    game_state: Res<GameState>,
    assets: ResMut<AssetLoader>,
) {
    let pipe_texture = assets.get_texture("pipe");
    let sprite_sheet = assets.get_texture("sprite_sheet");
    let player_animations = assets.get_animation("player_animations");
    let mut score_animations = assets.get_animation("score_animations").clone();
    let press_space: &Texture2D = assets.get_texture("press_space");
    let game_over = assets.get_texture("game_over");

    //draw pipes
    {
        for (_entity, position) in &pipe_query {
            //above
            draw_texture_ex(
                pipe_texture,
                position.x,
                position.y - pipe_texture.height(),
                WHITE,
                DrawTextureParams {
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: true,
                    pivot: None,
                    dest_size: Some(Vec2 {
                        x: pipe_texture.width(),
                        y: pipe_texture.height(),
                    }),
                },
            );

            draw_texture_ex(
                pipe_texture,
                position.x,
                position.y + PIPES_V_SEPARATION,
                WHITE,
                DrawTextureParams {
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                    dest_size: Some(Vec2 {
                        x: pipe_texture.width(),
                        y: pipe_texture.height(),
                    }),
                },
            );
        }
    }
    {
        //draw floors
        for (_entity, floor_position) in &floor_query {
            draw_texture(
                assets.get_texture("floor"),
                floor_position.x,
                floor_position.y,
                WHITE,
            )
        }
    }

    //draw player
    let (position, velocity) = player_query.into_inner();
    {
        match game_state.game_state {
            GameStates::Play => {
                draw_texture_ex(
                    sprite_sheet,
                    (position.x).floor(),
                    (position.y).floor(),
                    WHITE,
                    DrawTextureParams {
                        rotation: clamp(
                            PI / 20.0 * (velocity.y * get_frame_time()),
                            -PI / 4.0,
                            PI / 2.0,
                        ),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                        source: Some(player_animations.frame().source_rect),
                        dest_size: Some(player_animations.frame().dest_size),
                    },
                );
            }

            _ => {
                draw_texture_ex(
                    sprite_sheet,
                    (position.x).floor(),
                    (position.y).floor(),
                    WHITE,
                    DrawTextureParams {
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                        source: Some(player_animations.frame().source_rect),
                        dest_size: Some(player_animations.frame().dest_size),
                    },
                );
            }
        }
    }

    //draw UI
    {
        match game_state.game_state {
            GameStates::Play => {
                let (_entity, position) = score_query.into_inner();

                let score_str: String = game_state.score.to_string();
                let length = score_str.len();
                let mut count: u32 = 0;
                let total_length =
                    (length as i32 - 1) * UI_SPRITE_SEP + ((length as i32) * UI_SPRITE_SIZE);

                for c in score_str.chars() {
                    score_animations.set_frame(c.to_digit(10).unwrap());

                    draw_texture_ex(
                        sprite_sheet,
                        (position.x).floor() - total_length as f32 / 2.0
                            + count as f32 * (UI_SPRITE_SEP + UI_SPRITE_SIZE) as f32,
                        (position.y).floor() - UI_SPRITE_SIZE as f32 / 2.0,
                        WHITE,
                        DrawTextureParams {
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: None,
                            source: Some(score_animations.frame().source_rect),
                            dest_size: Some(score_animations.frame().dest_size),
                        },
                    );
                    count += 1;
                }
            }
            GameStates::GameOver => {
                let (_entity, position) = score_query.into_inner();

                let score_str: String = game_state.score.to_string();
                let length = score_str.len();
                let mut count: u32 = 0;
                let total_length =
                    (length as i32 - 1) * UI_SPRITE_SEP + ((length as i32) * UI_SPRITE_SIZE);

                for c in score_str.chars() {
                    score_animations.set_frame(c.to_digit(10).unwrap());

                    draw_texture_ex(
                        sprite_sheet,
                        (position.x).floor() - total_length as f32 / 2.0
                            + count as f32 * (UI_SPRITE_SEP + UI_SPRITE_SIZE) as f32,
                        (position.y).floor() - UI_SPRITE_SIZE as f32 / 2.0,
                        WHITE,
                        DrawTextureParams {
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: None,
                            source: Some(score_animations.frame().source_rect),
                            dest_size: Some(score_animations.frame().dest_size),
                        },
                    );
                    count += 1;
                }

                draw_texture(
                    game_over,
                    SCREEN_SIZE / 2.0 - game_over.width() / 2.0,
                    -SCREEN_SIZE / 2.0 - game_over.height() / 2.0,
                    WHITE,
                )
            }

            GameStates::PressStart => draw_texture(
                press_space,
                SCREEN_SIZE / 2.0 - press_space.width() / 2.0,
                -SCREEN_SIZE / 2.0 - press_space.height() / 2.0 - 10.0,
                WHITE,
            ),
            _ => {}
        }
    }
}
