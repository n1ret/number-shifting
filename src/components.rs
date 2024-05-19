use crossbeam_channel::Receiver;

use bevy::prelude::*;

#[derive(Resource)]
pub struct StdinReceiver {
    pub rx: Receiver<String>
}

pub enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Component)]
pub struct FieldLine {
    pub orientation: Orientation,
    pub position: usize
}

pub enum CellPosition {
    TopBorder,
    RightBorder,
    BottomBorder,
    LeftBorder,
    BG,
    Number
}

#[derive(Component)]
pub struct CellComponent {
    pub x: usize,
    pub y: usize,
    pub position: CellPosition
}

impl CellComponent {
    pub fn new(x: usize, y: usize, position: CellPosition) -> Self {
        Self { x, y, position }
    }
}
