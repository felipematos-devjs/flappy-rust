use bevy_ecs::prelude::*;
use macroquad::{
    prelude::{Vec2, WHITE},
    texture::{draw_texture, draw_texture_ex, DrawTextureParams, Texture2D},
    time::get_frame_time,
};

use crate::{
    game::{GameState, GameStates},
    pipe::{Pipe, PIPES_V_SEPARATION},
    transform::{Position, Sprite, Velocity},
};

const BACKGROUND_TILES_AMOUNT: u32 = 5;
const BACKGROUND_SPEED: Velocity = Velocity { x: -120.0, y: 0.0 };

#[derive(Resource)]
pub struct FloorSpriteRes {
    pub texture: Texture2D,
}

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

pub fn spawn_background(mut commands: Commands, sprite: Res<FloorSpriteRes>) {
    for i in 0..BACKGROUND_TILES_AMOUNT {
        commands.spawn(FloorBundle::new((i as f32) * sprite.texture.width()));
    }
}

pub fn draw_background(
    floor_query: Query<(Entity, &Position), With<Floor>>,
    pipe_query: Query<(&Position, &Sprite), With<Pipe>>,
    floor_sprite: Res<FloorSpriteRes>,
) {
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

    for (_entity, floor_position) in &floor_query {
        draw_texture(
            &floor_sprite.texture,
            floor_position.x,
            floor_position.y,
            WHITE,
        )
    }
}

pub fn update_background(
    mut floor_query: Query<(&mut Position, &Velocity), With<Floor>>,
    floor_sprite: Res<FloorSpriteRes>,
    game_state: Res<GameState>,
) {
    if game_state.game_state != GameStates::GameOver {
        for (mut position, velocity) in &mut floor_query {
            position.x += velocity.x * get_frame_time();

            if position.x < -floor_sprite.texture.width() {
                position.x += (BACKGROUND_TILES_AMOUNT as f32) * floor_sprite.texture.width();
            }
        }
    }
}

pub fn move_background() {}
