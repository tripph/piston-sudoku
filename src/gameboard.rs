//! Game board logic.

/// Size of game board.
const SIZE: usize = 9;

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[u8; SIZE]; SIZE],
    /// if a cell needs to be marked red, this option will contain the index of it
    pub error_cell: Option<[usize; 2]>
}

impl Gameboard {
    /// Creates a new game board.
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
            error_cell: None,
        }
    }
    /// Gets the character at cell location.
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]] {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None,
        })
    }
    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        // check that now other values across x/y have the same value as val; TODO: find less ugly method
        let mut already_exists = false;
        for i in 0..9 {
            if self.cells[ind[1]][i] == val || self.cells[i][ind[0]] == val {
                already_exists = true;
                break
            }
        }
        if !already_exists {
            self.cells[ind[1]][ind[0]] = val;
            self.error_cell = None;
        } else {
            self.error_cell = Some(ind);
        }
    }

}