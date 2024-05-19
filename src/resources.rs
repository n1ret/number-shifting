use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Board {
    width: usize,
    height: usize,
    pub grid: Vec<Vec<usize>>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![0; width]; height]
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
