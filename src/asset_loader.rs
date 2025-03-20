use bevy_ecs::prelude::*;
use macroquad::audio;
use macroquad::prelude::animation::{AnimatedSprite, Animation};
use macroquad::texture::{FilterMode, Texture2D, load_texture};
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct AssetLoader {
    pub texture_assets: HashMap<&'static str, Texture2D>,
    pub sound_assets: HashMap<&'static str, audio::Sound>,
    pub animation_assets: HashMap<&'static str, AnimatedSprite>,
}

pub enum AssetType {
    Texture,
    Sound,
    Animation,
}

impl AssetLoader {
    pub async fn load_texture(&mut self, name: &'static str, path: &'static str) -> &mut Self {
        let texture: Texture2D = load_texture(path).await.expect("Cannot load texture");
        texture.set_filter(FilterMode::Nearest);

        self.texture_assets.insert(name, texture);

        self
    }

    pub async fn load_sound(&mut self, name: &'static str, path: &'static str) -> &mut Self {
        let sound: audio::Sound = audio::load_sound(path).await.unwrap();
        self.sound_assets.insert(name, sound);

        self
    }

    pub fn load_animation(&mut self, name: &'static str, animation: AnimatedSprite) -> &mut Self {
        self.animation_assets.insert(name, animation);
        self
    }

    pub fn get_animation(&self, name: &str) -> &AnimatedSprite {
        self.animation_assets.get(name).unwrap()
    }

    pub fn get_texture(&self, name: &str) -> &Texture2D {
        self.texture_assets.get(name).unwrap()
    }

    pub fn get_sound(&self, name: &str) -> &audio::Sound {
        self.sound_assets.get(name).unwrap()
    }

    pub fn update_animation(&mut self, name: &str) {
        let anim = self.animation_assets.get_mut(name).unwrap();
        anim.update();
    }

    pub fn set_animation(&mut self, name: &str, number: usize) {
        let anim = self.animation_assets.get_mut(name).unwrap();
        anim.set_animation(number);
    }
}

pub async fn initialize_assets(world: &mut World) {
    let mut assets = AssetLoader::default();
    assets
        .load_texture("pipe", "assets/textures/pipe.png")
        .await;
    assets
        .load_texture("floor", "assets/textures/base.png")
        .await;
    assets
        .load_texture("game_over", "assets/textures/gameover.png")
        .await;
    assets
        .load_texture("press_space", "assets/textures/press_space.png")
        .await;
    assets
        .load_texture("sprite_sheet", "assets/textures/sprite_sheet.png")
        .await;
    assets
        .load_sound("hit_sound", "assets/audio/audio_hit.wav")
        .await;
    assets
        .load_sound("score_sound", "assets/audio/point.wav")
        .await;
    assets
        .load_sound("flap_sound", "assets/audio/wing.wav")
        .await;
    assets.load_animation(
        "player_animations",
        AnimatedSprite::new(
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
    );
    assets.load_animation(
        "score_animations",
        AnimatedSprite::new(
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
    );

    world.init_resource::<crate::game::GameState>();
    world.insert_resource(assets);
}
