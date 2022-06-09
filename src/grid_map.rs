#[allow(dead_code)]
pub mod grid_map {
    use array2d::Array2D;

    pub enum TILE {
        PATH,
        WALL,
        START,
        FINISH,
    }

    impl Clone for TILE {
        fn clone(&self) -> Self {
            match self {
                Self::PATH => Self::PATH,
                Self::WALL => Self::WALL,
                Self::START => Self::START,
                Self::FINISH => Self::FINISH,
            }
        }
    }

    pub struct Map {
        grid: Array2D<TILE>,
        start_x: u8,
        start_y: u8,
        end_x: u8,
        end_y: u8,
    }

    impl Map {
        pub fn new(_size: usize) -> Map {
            let new_map = Map {
                grid: Array2D::filled_with(TILE::PATH, _size, _size),
                start_x: 0,
                start_y: 0,
                end_x: 0,
                end_y: 0,
            };

            new_map
        }

        pub fn boudns(&self) -> (u8, u8) {
            (self.grid.row_len() as u8, self.grid.column_len() as u8)
        }

        pub fn paint_wall(&mut self, _x: usize, _y: usize) {
            self.grid[(_x, _y)] = TILE::WALL;
        }

        pub fn clear_map(&mut self) {
            let r_size = self.grid.row_len();
            let c_size = self.grid.column_len();
            self.grid = Array2D::filled_with(TILE::PATH, r_size, c_size);
        }

        pub fn get_map(&mut self) -> Array2D<TILE> {
            self.grid.clone()
        }

        pub fn set_start(&mut self, _x: u8, _y: u8) {
            if _x >= self.grid.row_len() as u8 || _y >= self.grid.column_len() as u8 {
                panic!("Invalid data for start point");
            }

            self.start_x = _x;
            self.start_y = _y;
            assing_array2d(&mut self.grid, _x, _y, TILE::START);
        }

        pub fn get_start(&self) -> (u8, u8) {
            (self.start_x, self.start_y)
        }

        pub fn set_end(&mut self, _x: u8, _y: u8) {
            if _x >= self.grid.row_len() as u8 || _y >= self.grid.column_len() as u8 {
                panic!("Invalid data for start point");
            }

            self.end_x = _x;
            self.end_y = _y;
            assing_array2d(&mut self.grid, _x, _y, TILE::FINISH);
        }

        pub fn get_end(&self) -> (u8, u8) {
            (self.end_x, self.end_y)
        }
    }

    fn assing_array2d(arr: &mut Array2D<TILE>, _x: u8, _y: u8, _val: TILE) {
        arr[(_x as usize, _y as usize)] = _val;
    }
}
