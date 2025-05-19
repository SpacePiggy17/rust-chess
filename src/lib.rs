use pyo3::prelude::*;

#[pymodule]
mod rust_chess {
    use std::str::FromStr;

    use chess::Board;
    use pyo3::{exceptions::PyValueError, prelude::*};

    /// Board class
    #[pyclass(name = "Board")]
    struct PyBoard {
        board: Board,
        halfmove_clock: u8,  // Halfmoves since last pawn move or capture
        fullmove_number: u8, // Fullmove number (increments after black moves)
    }

    #[pymethods]
    impl PyBoard {
        // Constructor
        #[new]
        fn new() -> Self {
            PyBoard {
                board: Board::default(),
                halfmove_clock: 0,
                fullmove_number: 1,
                // TODO: Incremental Zobrist hash
            }
        }

        // TODO: Add python doc hints

        /// Get the FEN string representation of the board
        fn get_fen(&self) -> PyResult<String> {
            let base_fen = self.board.to_string();

            // The chess crate does not track the halfmove clock and fullmove number,
            // so we need to add them manually.
            // 0: board, 1: player, 2: castling, 3: en passant, 4: halfmove clock, 5: fullmove number
            let mut parts: Vec<&str> = base_fen.split_whitespace().collect();

            let halfmove_clock_str: String = self.halfmove_clock.to_string();
            let fullmove_number_str: String = self.fullmove_number.to_string();
            parts[4] = halfmove_clock_str.as_str();
            parts[5] = fullmove_number_str.as_str();

            Ok(parts.join(" "))
        }

        /// Set the board state from a FEN string
        fn set_fen(&mut self, fen: &str) -> PyResult<()> {
            // Extract the halfmove clock and fullmove number from the FEN string
            let parts: Vec<&str> = fen.split_whitespace().collect();
            if parts.len() != 6 {
                return Err(PyValueError::new_err(
                    "FEN string must have exactly 6 parts",
                ));
            }

            // Parse the halfmove clock and fullmove number
            self.halfmove_clock = parts[4]
                .parse::<u8>()
                .map_err(|_| PyValueError::new_err("Invalid halfmove clock"))?;
            self.fullmove_number = parts[5]
                .parse::<u8>()
                .map_err(|_| PyValueError::new_err("Invalid fullmove number"))?;

            // Parse the board using the chess crate
            self.board = Board::from_str(fen)
                .map_err(|e| PyValueError::new_err(format!("Invalid FEN: {e}")))?;

            Ok(())
        }

        fn get_halfmove_clock(&self) -> PyResult<u8> {
            Ok(self.halfmove_clock)
        }

        fn get_fullmove_number(&self) -> PyResult<u8> {
            Ok(self.fullmove_number)
        }

        // Checks if the halfmoves since the last pawn move or capture is >= 100
        // and the game is ongoing (not checkmate or stalemate)
        fn is_fifty_moves(&self) -> PyResult<bool> {
            Ok(self.halfmove_clock >= 100 && self.board.status() == chess::BoardStatus::Ongoing)
        }

        // fn is_check(&self) -> PyResult<bool> {
        //     Ok(*self.board.checkers() != chess::EMPTY && )
        // }

        // fn make_move
    }
}
