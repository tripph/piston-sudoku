//! Gameboard controller.

use piston::input::{GenericEvent, Button, MouseButton};

use crate::Gameboard;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
    /// Currently selected cell which can take a number
    pub selected_cell: Option<[usize; 2]>,
    /// Current cursor position
    pub cursor_pos: [f64; 2],
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    fn find_clicked_cell(&mut self, pos: [f64; 2], size :f64) -> Option<[usize;2]> {
        // Find coords relative to upper left corner
        let x = self.cursor_pos[0] - pos[0];
        let y = self.cursor_pos[1] - pos[1];
        // Check coords are in board bounds
        if x >= 0.0 && x < size && y >= 0.0 && y < size {
            let cell_x = (x / size * 9.0) as usize;
            let cell_y = (y / size * 9.0) as usize;
            Some([cell_x, cell_y])
        } else {
            None
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, Key, MouseButton};
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let newly_selected_cell = self.find_clicked_cell(pos, size);
            if self.selected_cell != newly_selected_cell {
                self.selected_cell = newly_selected_cell;
            } else {
                self.selected_cell = None;
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if let Some(ind) = self.selected_cell {
                match key {
                    Key::D1 => self.gameboard.set(ind, 1),
                    Key::D2 => self.gameboard.set(ind, 2),
                    Key::D3 => self.gameboard.set(ind, 3),
                    Key::D4 => self.gameboard.set(ind, 4),
                    Key::D5 => self.gameboard.set(ind, 5),
                    Key::D6 => self.gameboard.set(ind, 6),
                    Key::D7 => self.gameboard.set(ind, 7),
                    Key::D8 => self.gameboard.set(ind, 8),
                    Key::D9 => self.gameboard.set(ind, 9),
                    Key::Backspace | Key::Delete => self.gameboard.set(ind, 0),
                    _ => {}
                }
            }
        }
    }
}