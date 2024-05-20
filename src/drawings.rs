use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution
};

use crate::resources::Board;
use crate::components::{
    FieldLine, CellComponent, Orientation::*,
    CellPosition::*
};

pub const WINDOW_SIZE: f32 = 720.;
const LINE_WIDTH: f32 = 5.;
const BOLD_LINE_WIDTH: f32 = 8.;
const FONT_SIZE: f32 = 0.8;

fn get_cell_size(window_res: &WindowResolution, board: &Board) -> f32 {
    let vertical = window_res.height() / board.height() as f32;
    let horizontal = window_res.width() / board.width() as f32;
    if vertical < horizontal { vertical } else { horizontal }
}

pub fn setup_draw(
    window_res: &WindowResolution,
    board: &Board,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cell_size = get_cell_size(window_res, board);

    let square_handle = meshes.add(
        Rectangle::new(1., 1.)
    );
    let gray_handle = materials.add(Color::GRAY);
    let black_handle = materials.add(Color::BLACK);

    // Draw vertical lines
    for i in 1..board.width() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(square_handle.clone()),
                material: gray_handle.clone(),
                transform: Transform::from_xyz(
                    cell_size * i as f32,
                    cell_size * board.height() as f32 / 2.,
                    0.
                ).with_scale(Vec3::new(
                    LINE_WIDTH,
                    cell_size * board.height() as f32,
                    1.
                )),
                ..default()
            },
            FieldLine { orientation: Vertical, position: i }
        ));
    }
    // Draw horizontal lines
    for i in 1..board.height() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(square_handle.clone()),
                material: gray_handle.clone(),
                transform: Transform::from_xyz(
                    cell_size * board.width() as f32 / 2.,
                    cell_size * i as f32,
                    0.
                ).with_scale(Vec3::new(
                    cell_size * board.height() as f32,
                    LINE_WIDTH,
                    1.
                )),
                ..default()
            },
            FieldLine { orientation: Horizontal, position: i }
        ));
    }

    let bg_material = materials.add(Color::rgb_u8(0xdd, 0xdd, 0xff));
    // Draw numbers
    for y in 0..board.height() { for x in 0..board.width() {
        if board.grid[y][x] == 0 { continue; }

        // Number
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    board.grid[y][x].to_string(),
                    TextStyle {
                        font: default(),
                        font_size: cell_size*FONT_SIZE,
                        color: Color::BLACK
                    }
                ),
                transform: Transform::from_xyz(
                    x as f32 * cell_size + cell_size/2.,
                    y as f32 * cell_size + cell_size/2.,
                    1.
                ),
                ..Default::default()
            },
            CellComponent::new(x, y, Number)
        ));

        // Background
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(square_handle.clone()),
                material: bg_material.clone(),
                transform: Transform::from_xyz(
                    x as f32 * cell_size + cell_size/2.,
                    y as f32 * cell_size + cell_size/2.,
                    -1.
                ).with_scale(Vec3::new(
                    cell_size,
                    cell_size,
                    1.
                )),
                ..default()
            },
            CellComponent::new(x, y, BG)
        ));

        // Borders
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(square_handle.clone()),
                material: black_handle.clone(),
                transform: Transform::from_xyz(
                    x as f32 * cell_size + cell_size/2.0,
                    y as f32 * cell_size + cell_size,
                    1.
                ).with_scale(Vec3::new(
                    cell_size + BOLD_LINE_WIDTH,
                    BOLD_LINE_WIDTH,
                    1.
                )),
                ..default()
            },
            CellComponent::new(x, y, TopBorder)
        ));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(square_handle.clone()),
                material: black_handle.clone(),
                transform: Transform::from_xyz(
                    x as f32 * cell_size + cell_size,
                    y as f32 * cell_size + cell_size/2.,
                    1.
                ).with_scale(Vec3::new(
                    BOLD_LINE_WIDTH,
                    cell_size + BOLD_LINE_WIDTH,
                    1.
                )),
                ..default()
            },
            CellComponent::new(x, y, RightBorder)
        ));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(square_handle.clone()),
                material: black_handle.clone(),
                transform: Transform::from_xyz(
                    x as f32 * cell_size + cell_size/2.,
                    y as f32 * cell_size,
                    1.
                ).with_scale(Vec3::new(
                    cell_size + BOLD_LINE_WIDTH,
                    BOLD_LINE_WIDTH,
                    1.
                )),
                ..default()
            },
            CellComponent::new(x, y, BottomBorder)
        ));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(square_handle.clone()),
                material: black_handle.clone(),
                transform: Transform::from_xyz(
                    x as f32 * cell_size,
                    y as f32 * cell_size + cell_size/2.,
                    1.
                ).with_scale(Vec3::new(
                    BOLD_LINE_WIDTH,
                    cell_size + BOLD_LINE_WIDTH,
                    1.
                )),
                ..default()
            },
            CellComponent::new(x, y, LeftBorder)
        ));
    } }
}

pub fn update_draw(
    window_res: &WindowResolution,
    board: &Board,
    mut lines: Query<(&mut Transform, &FieldLine), Without<CellComponent>>,
    mut numbers: Query<(&mut Transform, &CellComponent), Without<FieldLine>>
) {
    let cell_size = get_cell_size(window_res, board);
    let default_cell_size = get_cell_size(
        &WindowResolution::new(WINDOW_SIZE, WINDOW_SIZE),
        board
    );

    // Relocate field lines
    for (mut line_transform, line) in lines.iter_mut() {
        match line.orientation {
            Vertical => {
                line_transform.translation = Vec3::new(
                    cell_size * line.position as f32,
                    cell_size * board.height() as f32 / 2.,
                    0.
                );
                line_transform.scale = Vec3::new(1., cell_size * board.height() as f32, 1.);
            }
            Horizontal => {
                line_transform.translation = Vec3::new(
                    cell_size * board.width() as f32 / 2.,
                    cell_size * line.position as f32,
                    0.
                );
                line_transform.scale = Vec3::new(cell_size * board.width() as f32, 1., 1.);
            }
        }
    }

    // Relocate numbers
    for (mut component_transform, cell_component) in numbers.iter_mut() {
        let x = cell_component.x;
        let y = cell_component.y;
        match cell_component.position {
            Number => {
                component_transform.translation = Vec3::new(
                    x as f32 * cell_size + cell_size/2.,
                    y as f32 * cell_size + cell_size/2.,
                    1.
                );
                component_transform.scale = Vec3::new(
                    cell_size*FONT_SIZE / (default_cell_size*FONT_SIZE),
                    cell_size*FONT_SIZE / (default_cell_size*FONT_SIZE),
                    1.
                );
            }
            BG => {
                component_transform.translation = Vec3::new(
                    x as f32 * cell_size + cell_size/2.,
                    y as f32 * cell_size + cell_size/2.,
                    -1.
                );
                component_transform.scale = Vec3::new(
                    cell_size,
                    cell_size,
                    1.
                );
            }
            TopBorder => {
                component_transform.translation = Vec3::new(
                    x as f32 * cell_size + cell_size/2.0,
                    y as f32 * cell_size + cell_size,
                    1.
                );
                component_transform.scale = Vec3::new(
                    cell_size + BOLD_LINE_WIDTH,
                    BOLD_LINE_WIDTH,
                    1.
                );
            }
            RightBorder => {
                component_transform.translation = Vec3::new(
                    x as f32 * cell_size + cell_size,
                    y as f32 * cell_size + cell_size/2.,
                    1.
                );
                component_transform.scale = Vec3::new(
                    BOLD_LINE_WIDTH,
                    cell_size + BOLD_LINE_WIDTH,
                    1.
                );
            }
            BottomBorder => {
                component_transform.translation = Vec3::new(
                    x as f32 * cell_size + cell_size/2.,
                    y as f32 * cell_size,
                    1.
                );
                component_transform.scale = Vec3::new(
                    cell_size + BOLD_LINE_WIDTH,
                    BOLD_LINE_WIDTH,
                    1.
                );
            }
            LeftBorder => {
                component_transform.translation = Vec3::new(
                    x as f32 * cell_size,
                    y as f32 * cell_size + cell_size/2.,
                    1.
                );
                component_transform.scale = Vec3::new(
                    BOLD_LINE_WIDTH,
                    cell_size + BOLD_LINE_WIDTH,
                    1.
                );
            }
        }
    }
}

