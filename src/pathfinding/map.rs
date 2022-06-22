use array2d::Array2D;
use super::node::Node;

pub struct Map {
    pub cells: Array2D<Node>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Map {
    pub fn bounds(&self) -> (usize, usize) {
        return (self.cells.row_len(), self.cells.column_len());
    }
}
