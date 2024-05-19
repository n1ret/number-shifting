mod resources;
mod components;
mod drawings;
mod gen_level;

use bevy::{
    prelude::*,
    window::WindowResolution,
    log
};

use resources::Board;
use components::{FieldLine, CellComponent};
use drawings::WINDOW_SIZE;

fn main() {
    let board = gen_level::generate_level(100);

    println!("{} {}", board.width(), board.height());

    for y in 0..board.height() {
        for x in 0..board.width() {
            print!("{} ", board.grid[y][x]);
        }
        println!();
    }

    let (width, height) = if board.width() > board.height() {
        (WINDOW_SIZE, WINDOW_SIZE * board.height() as f32 / board.width() as f32)
    } else {
        (WINDOW_SIZE * board.width() as f32 / board.height() as f32, WINDOW_SIZE)
    };

    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "The best game OTW".to_string(),
                    resolution: WindowResolution::new(width, height),
                    ..default()
                }),
                ..default()
            })
            .set(log::LogPlugin {
                level: log::Level::ERROR,
                ..default()
            })
        )
        .insert_resource(ClearColor(Color::rgb_u8(0xbb, 0xbb, 0xdd)))
        .insert_resource(board)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    window: Query<&Window>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    board: ResMut<Board>
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.,
            viewport_origin: Vec2::new(0., 0.),
            ..default()
        },
        ..default()
    });

    drawings::setup_draw(
        &window.single().resolution, board.as_ref(),
        commands, meshes, materials
    );
}

fn update(
    window: Query<&Window>,
    board: Res<Board>,
    lines: Query<(&mut Transform, &FieldLine), Without<CellComponent>>,
    numbers: Query<(&mut Transform, &CellComponent), Without<FieldLine>>
) {
    drawings::update_draw(&window.single().resolution, board.as_ref(), lines, numbers);
}
