#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Character,
    Ground,
    Block,
    Wall,
    Diamond
}

pub struct Level {
    cells: [[Cell; 16]; 12]
}

fn set_next_cell(cells: &mut [[Cell; 16]; 12], row: &mut u32, col: &mut u32, cell: &Cell) {
    cells[*row as usize][*col as usize] = *cell;
    *col += 1;

    if (*col == 16) {
        *col = 0;
        *row += 1;
    }
}

macro_rules! set_next_cell {
    (X, $cells:expr, $row:expr, $col:expr) => (set_next_cell(&mut $cells, &mut $row, &mut $col, &Cell::Wall));
    (o, $cells:expr, $row:expr, $col:expr) => (set_next_cell(&mut $cells, &mut $row, &mut $col, &Cell::Ground));
    (H, $cells:expr, $row:expr, $col:expr) => (set_next_cell(&mut $cells, &mut $row, &mut $col, &Cell::Character));
    (B, $cells:expr, $row:expr, $col:expr) => (set_next_cell(&mut $cells, &mut $row, &mut $col, &Cell::Block));
    (q, $cells:expr, $row:expr, $col:expr) => (set_next_cell(&mut $cells, &mut $row, &mut $col, &Cell::Diamond));
}

macro_rules! level {
    ( $( $cell:ident )* ) => {
        {
            let mut cells = [[Cell::Wall; 16]; 12];
            let mut row: u32 = 0;
            let mut col: u32 = 0;

            $(
                set_next_cell!($cell, cells, row, col);
            )*

            Level {cells}
        }
    };
}

impl Level {

    pub fn new() -> Level {
        level! {
            o o o o o o o o o o o o o o o o
            o o o o o o o o o o o o o o o o
            o o o o o o X X X o o o o o o o
            o o o o o o X q X o o o o o o o
            o o o o o o X o X X X X o o o o
            o o o o X X X B o B q X o o o o
            o o o o X q o B H X X X o o o o
            o o o o X X X X B X o o o o o o
            o o o o o o o X q X o o o o o o
            o o o o o o o X X X o o o o o o
            o o o o o o o o o o o o o o o o
            o o o o o o o o o o o o o o o o
        }
    }

    pub fn each<Predicate>(&self, mut predicate: Predicate) where Predicate: FnMut(u32, u32, &Cell) {
        let mut row_index = 0;

        for row in self.cells.iter() {
            let mut col_index = 0;

            for cell in row.iter() {
                predicate(row_index, col_index, cell);
                col_index += 1;
            }

            row_index += 1;
        }
    }

    pub fn each_type<Predicate>(&self, cell_type: &Cell, mut predicate: Predicate) where Predicate: FnMut(u32, u32) {
        self.each(|row, col, cell| {
            if (cell == cell_type) {
                predicate(row, col);
            }
        })
    }

    pub fn each_wall<Predicate>(&self, mut predicate: Predicate) where Predicate: FnMut(u32, u32) {
        self.each_type(&Cell::Wall, predicate);
    }

    pub fn each_block<Predicate>(&self, mut predicate: Predicate) where Predicate: FnMut(u32, u32) {
        self.each_type(&Cell::Block, predicate);
    }

    pub fn each_diamond<Predicate>(&self, mut predicate: Predicate) where Predicate: FnMut(u32, u32) {
        self.each_type(&Cell::Diamond, predicate);
    }

    pub fn each_ground<Predicate>(&self, mut predicate: Predicate) where Predicate: FnMut(u32, u32) {
        self.each(|row, col, cell| {
            predicate(row, col);
        });
    }

    pub fn contains_wall(&self, row: i32, col: i32) -> bool {
        self.cells[row as usize][col as usize] == Cell::Wall
    }

    pub fn contains_diamond(&self, row: i32, col: i32) -> bool {
        self.cells[row as usize][col as usize] == Cell::Diamond
    }

}
