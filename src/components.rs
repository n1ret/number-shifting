use bevy::prelude::*;

pub enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Component)]
pub struct FieldLine {
    pub orientation: Orientation,
    pub position: usize
}

#[derive(Component)]
pub enum CellComponent {
    TopBorder { x: usize, y: usize },
    RightBorder { x: usize, y: usize },
    BottomBorder { x: usize, y: usize },
    LeftBorder { x: usize, y: usize },
    BG { x: usize, y: usize },
    Number { x: usize, y: usize }
}
