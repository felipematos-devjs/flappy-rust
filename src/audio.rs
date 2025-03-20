use bevy_ecs::system::Resource;
use macroquad::audio;

#[derive(Resource)]
pub struct ScoreSound {
    pub audio: audio::Sound,
}

#[derive(Resource)]
pub struct FlapSound {
    pub audio: audio::Sound,
}

#[derive(Resource)]
pub struct HitSound {
    pub audio: audio::Sound,
}
