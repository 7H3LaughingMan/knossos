use crate::maze::formatters::{Formatter, GridWrapper};

pub struct CellGrid;

impl Formatter<GridWrapper> for CellGrid {
    fn format(&self, grid: &crate::maze::grid::Grid) -> GridWrapper {
        GridWrapper { width: grid.width, height: grid.height, cells: grid.cells.clone() }
    }
}