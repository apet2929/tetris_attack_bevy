use bevy::ecs::component::{ComponentId, Components, StorageType};
use bevy::ecs::storage::Storages;
use bevy::prelude::*;
use bevy::ptr::OwningPtr;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_blocks)
        .add_startup_system(spawn_timer)
        .add_system(move_blocks_up)
        .add_system(update_board)
        .run()
}

#[derive(Component)]
pub struct BoardTimer {
    time_since_tick: f32
}

impl Default for BoardTimer {
    fn default() -> Self {
        BoardTimer {
            time_since_tick: 0.0
        }
    }
}

impl BoardTimer {
    pub fn tick(&mut self, mut board: Query<&mut Block>) {
        if self.time_since_tick > 1.0 {
            println!("Tick!");
            for mut block in board.iter_mut() {
                block.y += 1;
            }
            self.time_since_tick = 0.0;
        }
    }

    pub fn update(&mut self, time: f32, mut board: Query<&mut Block>) {
        self.time_since_tick += time;
        self.tick(board)
    }
}

pub fn spawn_timer(
    mut commands: Commands
) {
    commands.spawn(
        BoardTimer::default()
    );
}

pub fn update_board(
    mut board_timer_query: Query<&mut BoardTimer>,
    mut board_query: Query<&mut Block>,
    time: Res<Time>
) {
    let mut board_timer = board_timer_query.single_mut();
    board_timer.update(time.delta_seconds(), board_query);
}

#[derive(Component)]
pub struct Block {
    x: i32,
    y: i32
}

pub fn spawn_blocks(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let cols = 10;
    let left = window.width() * 0.1;
    let top = window.height() * 0.1;
    let block_width = (window.width() * 0.8) / cols as f32;

    for i in 0..10 {
        for j in 0..cols {
            let x = left.clone() + (j.clone() as f32 * block_width.clone());
            let y = top.clone() + (i.clone() as f32 * block_width.clone());
            commands.spawn(
                (
                    Block {
                        x: j.clone(),
                        y: i.clone()
                    },
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0),
                        texture: asset_server.load("sprites/ball_blue_large.png"),
                        ..default()
                    },
                )
            );
        }
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width()/2.0,window.height()/2.0,0.0),
            ..default()
        }
    );
}

pub fn move_blocks_up(
    mut block_query: Query<&mut Transform, With<Block>>
) {
    for (mut transform) in block_query.iter_mut() {
        transform.translation.y += 1.0;
    }
}