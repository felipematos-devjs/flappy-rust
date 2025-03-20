use bevy_ecs::prelude::*;
use flappy::background::{spawn_background, update_background};
use flappy::game::{check_inputs, draw_call, initialize_resources, restart_game, SCREEN_SIZE};
use flappy::pipe::{move_pipe, spawn_pipes};
use flappy::player::{add_gravity, collide_player, move_player, spawn_player};
use flappy::ui::add_ui;
use macroquad::prelude::{clear_background, next_frame, set_camera, Camera2D, Rect, SKYBLUE};
use macroquad::window::request_new_screen_size;

#[macroquad::main("Flappy")]
async fn main() {
    //initialize resources such as sprites
    let mut world = World::default();
    initialize_resources(&mut world).await;

    //Startup systems. Run once
    let mut startup_schedule = Schedule::default();
    startup_schedule.add_systems(spawn_pipes);
    startup_schedule.add_systems(spawn_player);
    startup_schedule.add_systems(spawn_background);
    startup_schedule.add_systems(add_ui);
    startup_schedule.run(&mut world);

    //Update Systems. Run in a loop
    let mut schedule = Schedule::default();
    schedule.add_systems(check_inputs);

    schedule.add_systems(add_gravity);

    schedule.add_systems(collide_player);
    schedule.add_systems(restart_game);

    //update

    schedule.add_systems(move_player);
    schedule.add_systems(move_pipe);
    schedule.add_systems(update_background);

    //draw
    schedule.add_systems(draw_call);

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, SCREEN_SIZE, -SCREEN_SIZE));
    request_new_screen_size(SCREEN_SIZE * 2.0, SCREEN_SIZE * 2.0);

    loop {
        clear_background(SKYBLUE);
        set_camera(&camera);
        schedule.run(&mut world);
        next_frame().await
    }
}
