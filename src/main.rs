mod resources;
mod components;
mod drawings;
mod gen_level;

use std::io;
use crossbeam_channel::bounded;
use std::thread;

use bevy::{
    prelude::*,
    window::WindowResolution,
    log
};

use resources::Board;
use components::{FieldLine, CellComponent, StdinReceiver, CellPosition::*};
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

    let (tx, rx) = bounded(10);

    thread::spawn(move || {
        let stdin = io::stdin();
        let mut buf = String::new();
        loop {
            stdin.read_line(&mut buf).unwrap();
            tx.send(buf.clone()).unwrap();
        }
    });

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
        .insert_resource(StdinReceiver { rx })
        .insert_resource(board)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_update)
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

fn draw_update(
    window: Query<&Window>,
    mut board: ResMut<Board>,
    stdin: Res<StdinReceiver>,
    lines: Query<(&mut Transform, &FieldLine), Without<CellComponent>>,
    numbers: Query<(&mut Transform, &CellComponent), Without<FieldLine>>,
    mut numbers_texts: Query<(&mut Text, &CellComponent)>,
    numbers_entities: Query<(Entity, &CellComponent)>,
    mut commands: Commands
) {
    while let Ok(input) = stdin.rx.try_recv() {
        let not_enough = "not enough arguments";
        let parse_error = "usize parse error";
        let action_invalid = "action invalid";
        
        let mut spl = input.split_whitespace();

        let x1 = spl.next().expect(not_enough)
            .parse::<usize>().expect(parse_error);
        let y1 = spl.next().expect(not_enough)
            .parse::<usize>().expect(parse_error);
        let dir = spl.next().expect(not_enough);
        let add = spl.next().expect(not_enough) == "+";
        
        if x1 >= board.width() || y1 >= board.height() {
            panic!("{}", parse_error);
        }
        if board.grid[y1][x1] == 0 {
            panic!("cell is empty");
        }

        let value = board.grid[y1][x1];

        let mut x2 = None;
        let mut y2 = None;

        match dir {
            "U" => {
                if y1+value >= board.height() {
                    panic!("{}", action_invalid);
                }
                if board.grid[y1+value][x1] != 0 {
                    x2 = Some(x1);
                    y2 = Some(y1+value);
                }
            }
            "R" => {
                if x1+value >= board.width() {
                    panic!("{}", action_invalid);
                }
                if board.grid[y1][x1+value] != 0 {
                    x2 = Some(x1+value);
                    y2 = Some(y1);
                }
            }
            "D" => {
                if y1.checked_sub(value).is_none() {
                    panic!("{}", action_invalid);
                }
                if board.grid[y1-value][x1] != 0 {
                    x2 = Some(x1);
                    y2 = Some(y1-value);
                }
            }
            "L" => {
                if x1.checked_sub(value).is_none() {
                    panic!("{}", action_invalid);
                }
                if board.grid[y1][x1-value] != 0 {
                    x2 = Some(x1-value);
                    y2 = Some(y1);
                }
            }
            _ => panic!("invalid direction specified")
        }
        
        let x2 = x2.unwrap();
        let y2 = y2.unwrap();

        if add {
            board.grid[y2][x2] += value;
        } else {
            board.grid[y2][x2] = (board.grid[y2][x2] as i32 - value as i32).abs() as usize;
        }
        board.grid[y1][x1] = 0;

        for (mut text, cell_component) in numbers_texts.iter_mut() {
            if let Number = cell_component.position {
                if cell_component.x == x2 && cell_component.y == y2 {
                    text.sections[0].value = board.grid[y2][x2].to_string();
                }
            }
        }
        for (entity, cell_component) in numbers_entities.iter() {
            if cell_component.x == x1 && cell_component.y == y1 {
                commands.entity(entity).despawn();
            }
        }
    }

    drawings::update_draw(&window.single().resolution, board.as_ref(), lines, numbers);
}
