#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Block,
    Wall,
    Diamond
}

pub struct Level {
    cells: [[Cell; 16]; 12]
}

impl Level {

    pub fn new() -> Level {
        let mut cells = [[Cell::Wall; 16]; 12];

        (4..8).for_each(|row| {
            (2..10).for_each(|col| {
                cells[row as usize][col as usize] = Cell::Empty;
            });
        });

        Level {cells}
    }

    pub fn each<Predicate>(&self, cell_type: &Cell, mut predicate: Predicate) where Predicate: FnMut(u32, u32) {
        let mut row_index = 0;

        for row in self.cells.iter() {
            let mut col_index = 0;

            for cell in row.iter() {
                if (cell == cell_type) {
                    predicate(row_index, col_index);
                }

                col_index += 1;
            }

            row_index += 1;
        }
    }

    pub fn each_wall<Predicate>(&self, mut predicate: Predicate) where Predicate: FnMut(u32, u32) {
        self.each(&Cell::Wall, predicate);
    }

    pub fn is_wall(&self, row: u32, col: u32) -> bool {
        match self.cells[row as usize][col as usize] {
            Cell::Wall => true,
            _ => false
        }
    }

}
