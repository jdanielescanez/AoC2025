#[derive(Clone, Debug)]
pub struct Board<T> {
    board: Vec<Vec<T>>,
    number_of_rows: usize,
    number_of_columns: usize,
}

impl<T> Board<T>
where
    T: Copy + PartialEq,
{
    pub fn new(board: Vec<Vec<T>>) -> Self {
        let number_of_rows = board.len();
        let number_of_columns = board[0].len();
        Board {
            board,
            number_of_rows,
            number_of_columns,
        }
    }

    pub fn set(&mut self, row: i32, column: i32, x: T) -> Option<T> {
        if 0 <= row
            && row < self.number_of_rows as i32
            && 0 <= column
            && column < self.number_of_columns as i32
        {
            self.board[row as usize][column as usize] = x;
            Some(x)
        } else {
            None
        }
    }

    pub fn get(&self, row: i32, column: i32) -> Option<T> {
        if 0 <= row
            && row < self.number_of_rows as i32
            && 0 <= column
            && column < self.number_of_columns as i32
        {
            Some(self.board[row as usize][column as usize])
        } else {
            None
        }
    }

    pub fn get_number_of_rows(&self) -> usize {
        self.number_of_rows
    }

    pub fn get_number_of_columns(&self) -> usize {
        self.number_of_columns
    }

    pub fn find(&self, element: T) -> Option<(usize, usize)> {
        (0..self.number_of_rows)
            .flat_map(move |row| (0..self.number_of_columns).map(move |column| (row, column)))
            .find(|&(row, column)| self.board[row][column] == element)
    }
}
