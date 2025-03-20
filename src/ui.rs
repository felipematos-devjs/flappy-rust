use bevy_ecs::prelude::*;
use macroquad::prelude::{animation::AnimatedSprite, *};

pub const UI_SPRITE_SIZE: i32 = 36;
pub const UI_SPRITE_SEP: i32 = -10;

use crate::{
    game::{GameState, SCREEN_SIZE},
    transform::Position,
};

#[derive(Resource)]
pub struct ScoreUiRes {
    pub texture: Texture2D,
    pub animation: AnimatedSprite,
}

#[derive(Component)]
pub struct Score;

#[derive(Bundle)]
pub struct ScoreUI {
    position: Position,
    is_type: Score,
}

#[derive(Component)]
pub struct GameOver;

#[derive(Resource)]
pub struct GameOverRes {
    pub texture: Texture2D,
}

#[derive(Bundle)]
pub struct GameOverUI {
    position: Position,
    is_type: GameOver,
}

#[derive(Component)]
pub struct PressStart;

#[derive(Resource)]
pub struct PressStartRes {
    pub texture: Texture2D,
}

#[derive(Bundle)]
pub struct PressStartUI {
    position: Position,
    is_type: PressStart,
}

pub fn add_ui(mut commands: Commands) {
    commands.spawn(ScoreUI {
        position: Position {
            x: SCREEN_SIZE / 2.0,
            y: -SCREEN_SIZE + 30.0,
        },
        is_type: Score {},
    });
}
