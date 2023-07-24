use std::ptr::null;
use bevy::ecs::component::{ComponentId, Components, StorageType};
use bevy::ecs::storage::Storages;
use bevy::prelude::*;
use bevy::ptr::OwningPtr;
use bevy::window::PrimaryWindow;

const COLS: i32 =  10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_blocks)
        .add_startup_system(spawn_timer)
        .add_system(move_blocks_up)
        .add_system(update_board)
        .add_system(fall_system)
        .run()
}

#[derive(Component)]
pub struct BoardTimer {
    time_since_tick: f32,
    fall_timer: f32
}

impl Default for BoardTimer {
    fn default() -> Self {
        BoardTimer {
            time_since_tick: 0.0,
            fall_timer: 0.0
        }
    }
}

impl BoardTimer {
    pub fn tick(&mut self, mut board: Query<&mut Block>) {
        let should_tick = false;
        if should_tick {
            if self.time_since_tick > 1.0 {
                println!("Tick!");
                for mut block in board.iter_mut() {
                    block.y += 1;
                }
                self.time_since_tick = 0.0;
            }
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
    y: i32,
}

#[derive(Component)]
pub struct Falling {
    timer: f32
}

#[derive(Component)]
pub struct Removed {}

pub fn spawn_blocks(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let left = window.width() * 0.1;
    let top = window.height() * 0.1;
    let block_width = (window.width() * 0.8) / COLS as f32;

    for i in 0..5 {
        for j in 0..COLS {
            let x = left.clone() + (j.clone() as f32 * block_width.clone());
            let y = top.clone() + (i.clone() as f32 * block_width.clone());
            let mut block = Block {
                x: j.clone(),
                y: i.clone()
            };
            let id = commands.spawn(
                (
                    block,
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0),
                        texture: asset_server.load("sprites/ball_blue_large.png"),
                        ..default()
                    },
                )
            ).id();
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
        transform.translation.y += 0.0;
    }
}

pub fn fall_system(
    mut commands: Commands,
    time: Res<Time>,
    mut board: Query<(Entity, &Block)>,
    mut removed_blocks_query: Query<(Entity, &Block), With<Removed>>,
    keys: Res<Input<KeyCode>>
) {
    if keys.just_pressed(KeyCode::Space) {
        for (e, block) in board.iter() {
            println!("x: {}, y: {}", block.x, block.y);
            commands.entity(e).insert(Removed{});
            return;
        }
    }
    let removed_positions: Vec<(i32, i32)> = removed_blocks_query.iter().map(|(entity, block)|  {
        (block.x, block.y)
    } ).collect();

    // println!("{:?}", removed_positions);

    for(e, block) in board.iter() {
        for (rx, ry) in &removed_positions {
            if (block.x == *rx) && (block.y > *ry) {
                commands.entity(e).insert(Falling{ timer: 0.0 });
            }
        }
    }

    for (entity, block) in removed_blocks_query.iter() {
        commands.entity(entity).despawn();
    };
}

pub fn update_falling(
    mut commands: Commands,
    mut board: Query<(Entity, &mut Block, &mut Transform, Option<&mut Falling>)>,
    time: Res<Time>
) {
    for (entity, mut block,mut transform, mut fall) in board.iter_mut() {
        if let Some(mut falling) = fall {
            if falling.timer > 0.3 {
                let column: Vec<&Block> = board.iter()
                    .filter(|(e, b, t, f)| b.x == block.x)
                    .map(|(e, b,t,  f)| b).collect();
                if block.y > 0 {
                    let block_below = column.iter().filter(|b| b.y == block.y - 1).count() > 0;
                    if !block_below { // no block below
                        block.y -= 1;
                        transform.translation.y -= 32.0;
                    }
                } else {
                    println!("Removing falling");
                    commands.entity(entity).remove::<Falling>();
                }

            } else {
                falling.timer += time.delta_seconds();
            }
        }

    }
}

