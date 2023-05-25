#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

#[derive(Clone, Debug)]
pub struct Board {
    pub board: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut col = Vec::with_capacity(height);
        let mut board = Vec::with_capacity(width);

        col.fill(Cell::Dead);
        board.fill(col);

        Self {
            board,
            width,
            height,
        }
    }

    pub fn update_board() {
        todo!()
    }

    fn get_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        let x_set = if x == 0 {
            [0, 1, self.width - 1]
        } else if x == self.width - 1 {
            [x - 1, x, 0]
        } else {
            [x - 1, x, x + 1]
        };

        let y_set = if y == 0 {
            [0, 1, self.height - 1]
        } else if y == self.height - 1 {
            [y - 1, y, 0]
        } else {
            [y - 1, y, y + 1]
        };

        for (x, y) in x_set.iter().zip(y_set.iter()) {
            if let Cell::Alive = self.board[*x][*y] {
                count += 1;
            }
        }

        count
    }
}
