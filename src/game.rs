use crate::audio::{FlapSound, HitSound, ScoreSound};
use crate::background::{Floor, FloorSpriteRes};
use crate::pipe::{
    Pipe, PipeSpriteRes, PIPES_H_SEPARATION, PIPES_V_SEPARATION, PIPE_CEILING_PADDING,
    PIPE_FLOOR_PADDING,
};
use crate::player::{Player, PlayerSpriteRes};
use crate::transform::{Position, Sprite, Velocity};
use crate::ui::{GameOverRes, PressStartRes, Score, ScoreUiRes, UI_SPRITE_SEP, UI_SPRITE_SIZE};
use bevy_ecs::prelude::*;
use macroquad::audio;
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use std::f32::consts::PI;
use std::str::FromStr;

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
    } else if is_key_pressed(KeyCode::Space) && game_state.game_state == GameStates::GameOver {
        game_state.score = 0;
        game_state.game_state = GameStates::Restart;
    }
}

pub fn restart_game(
    mut game_state: ResMut<GameState>,
    mut player_query: Single<(&mut Position, &mut Velocity), (With<Player>, Without<Pipe>)>,
    mut pipe_query: Query<(Entity, &mut Position), (With<Pipe>, Without<Player>)>,
    mut player_sprite: ResMut<PlayerSpriteRes>,
) {
    if game_state.is_changed() && game_state.game_state == GameStates::Restart {
        //reset pipes
        {
            let mut current_pipe = 0.0;
            for (_entity, mut position) in &mut pipe_query {
                let random: f32 = macroquad::rand::gen_range(0.0, 1.0);
                position.x = SCREEN_SIZE + current_pipe * PIPES_H_SEPARATION;
                position.y = clamp(
                    random * (-SCREEN_SIZE),
                    -SCREEN_SIZE + PIPE_CEILING_PADDING,
                    -PIPES_V_SEPARATION - PIPE_FLOOR_PADDING,
                );
                current_pipe += 1.0;
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
        player_sprite
            .animation
            .set_animation(rand::RandomRange::gen_range(0, 3));
    }
}

pub async fn initialize_resources(world: &mut World) {
    let sprite_sheet: Texture2D = load_texture("assets/sprite_sheet.png")
        .await
        .expect("Cannot load texture");
    sprite_sheet.set_filter(FilterMode::Nearest);

    let player_texture: Texture2D = load_texture("assets/bird.png")
        .await
        .expect("cannot load texture");
    player_texture.set_filter(FilterMode::Nearest);

    let pipe_texture: Texture2D = load_texture("assets/pipe.png").await.unwrap();
    pipe_texture.set_filter(FilterMode::Nearest);

    let floor_texture: Texture2D = load_texture("assets/base.png").await.unwrap();
    floor_texture.set_filter(FilterMode::Nearest);

    let pipe_sprite: PipeSpriteRes = PipeSpriteRes {
        texture: pipe_texture.clone(),
    };

    let game_over_texture: Texture2D = load_texture("assets/gameover.png").await.unwrap();
    game_over_texture.set_filter(FilterMode::Nearest);

    let press_start_texture: Texture2D = load_texture("assets/press_space.png").await.unwrap();
    press_start_texture.set_filter(FilterMode::Nearest);

    let score_ui_res: ScoreUiRes = ScoreUiRes {
        texture: sprite_sheet.clone(),
        animation: AnimatedSprite::new(
            36,
            36,
            &[Animation {
                name: "numbers".to_string(),
                row: 0,
                frames: 10,
                fps: 1,
            }],
            true,
        ),
    };

    let game_over_res: GameOverRes = GameOverRes {
        texture: game_over_texture.clone(),
    };

    let press_start_res: PressStartRes = PressStartRes {
        texture: press_start_texture.clone(),
    };

    let player_sprite: PlayerSpriteRes = PlayerSpriteRes {
        texture: sprite_sheet.clone(),
        animation: AnimatedSprite::new(
            36,
            36,
            &[
                Animation {
                    name: "yellow".to_string(),
                    row: 1,
                    frames: 3,
                    fps: 6,
                },
                Animation {
                    name: "blue".to_string(),
                    row: 2,
                    frames: 3,
                    fps: 6,
                },
                Animation {
                    name: "red".to_string(),
                    row: 3,
                    frames: 3,
                    fps: 6,
                },
            ],
            true,
        ),
    };

    let floor_sprite: FloorSpriteRes = FloorSpriteRes {
        texture: floor_texture.clone(),
    };

    let score_sound = audio::load_sound("assets/point.wav").await.unwrap();

    let score_sound_res: ScoreSound = ScoreSound {
        audio: score_sound.clone(),
    };

    let flap_sound = audio::load_sound("assets/wing.wav").await.unwrap();

    let flap_sound_res: FlapSound = FlapSound {
        audio: flap_sound.clone(),
    };

    let hit_sound = audio::load_sound("assets/audio_hit.wav").await.unwrap();

    let hit_sound_res: HitSound = HitSound {
        audio: hit_sound.clone(),
    };

    world.init_resource::<crate::game::GameState>();
    world.insert_resource(pipe_sprite);
    world.insert_resource(player_sprite);
    world.insert_resource(floor_sprite);
    world.insert_resource(score_ui_res);
    world.insert_resource(press_start_res);
    world.insert_resource(game_over_res);
    world.insert_resource(score_sound_res);
    world.insert_resource(flap_sound_res);
    world.insert_resource(hit_sound_res);
}

pub fn draw_call(
    floor_query: Query<(Entity, &Position), With<Floor>>,
    pipe_query: Query<(&Position, &Sprite), With<Pipe>>,
    player_query: Single<(&Position, &Sprite, &Velocity), With<Player>>,
    mut score_query: Single<(Entity, &Position), With<Score>>,
    floor_sprite: Res<FloorSpriteRes>,
    player_sprite: Res<PlayerSpriteRes>,
    game_state: Res<GameState>,
    mut score_resource: ResMut<ScoreUiRes>,
    mut game_over_resource: ResMut<GameOverRes>,
    mut press_start_resource: ResMut<PressStartRes>,
) {
    //draw pipes
    for (position, sprite) in &pipe_query {
        //above
        draw_texture_ex(
            &sprite.sprite,
            position.x,
            position.y - sprite.sprite.height(),
            WHITE,
            DrawTextureParams {
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: true,
                pivot: None,
                dest_size: Some(Vec2 {
                    x: sprite.sprite.width(),
                    y: sprite.sprite.height(),
                }),
            },
        );

        draw_texture_ex(
            &sprite.sprite,
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
                    x: sprite.sprite.width(),
                    y: sprite.sprite.height(),
                }),
            },
        );
    }

    //draw floors
    for (_entity, floor_position) in &floor_query {
        draw_texture(
            &floor_sprite.texture,
            floor_position.x,
            floor_position.y,
            WHITE,
        )
    }

    //draw player
    let (position, sprite, velocity) = player_query.into_inner();
    {
        match game_state.game_state {
            GameStates::Play => {
                draw_texture_ex(
                    &sprite.sprite,
                    (position.x).floor(),
                    (position.y).floor(),
                    WHITE,
                    DrawTextureParams {
                        rotation: clamp(PI / 20.0 * velocity.y, -PI / 4.0, PI / 2.0),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                        source: Some(player_sprite.animation.frame().source_rect),
                        dest_size: Some(player_sprite.animation.frame().dest_size),
                    },
                );
            }

            _ => {
                draw_texture_ex(
                    &sprite.sprite,
                    (position.x).floor(),
                    (position.y).floor(),
                    WHITE,
                    DrawTextureParams {
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                        source: Some(player_sprite.animation.frame().source_rect),
                        dest_size: Some(player_sprite.animation.frame().dest_size),
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
                    score_resource.animation.set_frame(c.to_digit(10).unwrap());

                    draw_texture_ex(
                        &score_resource.texture,
                        (position.x).floor() - total_length as f32 / 2.0
                            + count as f32 * (UI_SPRITE_SEP + UI_SPRITE_SIZE) as f32,
                        (position.y).floor() - UI_SPRITE_SIZE as f32 / 2.0,
                        WHITE,
                        DrawTextureParams {
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: None,
                            source: Some(score_resource.animation.frame().source_rect),
                            dest_size: Some(score_resource.animation.frame().dest_size),
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
                    score_resource.animation.set_frame(c.to_digit(10).unwrap());

                    draw_texture_ex(
                        &score_resource.texture,
                        (position.x).floor() - total_length as f32 / 2.0
                            + count as f32 * (UI_SPRITE_SEP + UI_SPRITE_SIZE) as f32,
                        (position.y).floor() - UI_SPRITE_SIZE as f32 / 2.0,
                        WHITE,
                        DrawTextureParams {
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: None,
                            source: Some(score_resource.animation.frame().source_rect),
                            dest_size: Some(score_resource.animation.frame().dest_size),
                        },
                    );
                    count += 1;
                }

                draw_texture(
                    &game_over_resource.texture,
                    SCREEN_SIZE / 2.0 - game_over_resource.texture.width() / 2.0,
                    -SCREEN_SIZE / 2.0 - game_over_resource.texture.height() / 2.0,
                    WHITE,
                )
            }

            GameStates::PressStart => draw_texture(
                &press_start_resource.texture,
                SCREEN_SIZE / 2.0 - press_start_resource.texture.width() / 2.0,
                -SCREEN_SIZE / 2.0 - press_start_resource.texture.height() / 2.0 - 10.0,
                WHITE,
            ),
            _ => {}
        }
    }
}
