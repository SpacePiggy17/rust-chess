// PyO3 does not support "self" input parameters, only "&self"
#![allow(clippy::trivially_copy_pass_by_ref)]

use std::str::FromStr;

use pyo3::{exceptions::PyValueError, prelude::*, types::PyAny};
use pyo3_stub_gen::{define_stub_info_gatherer, derive::{gen_stub_pyclass, gen_stub_pymethods}};

// Color constants
const WHITE: PyColor = PyColor(chess::Color::White);
const BLACK: PyColor = PyColor(chess::Color::Black);
const COLORS: [PyColor; 2] = [WHITE, BLACK];

// Piece constants
const PAWN: PyPiece = PyPiece(chess::Piece::Pawn);
const KNIGHT: PyPiece = PyPiece(chess::Piece::Knight);
const BISHOP: PyPiece = PyPiece(chess::Piece::Bishop);
const ROOK: PyPiece = PyPiece(chess::Piece::Rook);
const QUEEN: PyPiece = PyPiece(chess::Piece::Queen);
const KING: PyPiece = PyPiece(chess::Piece::King);
const PIECES: [PyPiece; 6] = [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING];

/// Color enum
#[gen_stub_pyclass]
#[pyclass(name = "Color")]
#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Hash)]
struct PyColor(chess::Color);

#[gen_stub_pymethods]
#[pymethods]
impl PyColor {
    /// Get the color as a string
    #[inline]
    fn __str__(&self) -> String {
        if *self == WHITE {
            "WHITE".to_string()
        } else {
            "BLACK".to_string()
        }
    }

    /// Get the color as a bool string
    #[inline]
    fn __repr__(&self) -> String {
        if *self == WHITE {
            "True".to_string()
        } else {
            "False".to_string()
        }
    }
}

/// Piece enum
#[gen_stub_pyclass]
#[pyclass(name = "Piece")]
#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
struct PyPiece(chess::Piece);

#[gen_stub_pymethods]
#[pymethods]
impl PyPiece {
    /// Get the index of the piece (0-5)
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn get_index(&self) -> u8 {
        self.0.to_index() as u8
    }

    // Convert the piece to a string
    #[inline]
    #[allow(clippy::wrong_self_convention, clippy::inherent_to_string)]
    fn to_string(&self) -> String {
        match *self {
            PAWN => "PAWN".to_string(),
            KNIGHT => "KNIGHT".to_string(),
            BISHOP => "BISHOP".to_string(),
            ROOK => "ROOK".to_string(),
            QUEEN => "QUEEN".to_string(),
            KING => "KING".to_string(),
        }
    }

    /// Convert the piece to a string
    #[inline]
    fn __str__(&self) -> String {
        self.to_string()
    }

    /// Convert the piece to a string
    #[inline]
    fn __repr__(&self) -> String {
        self.to_string()
    }
}

/// Square class
#[gen_stub_pyclass]
#[pyclass(name = "Square")]
#[derive(PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Default, Hash)]
struct PySquare(chess::Square);

#[gen_stub_pymethods]
#[pymethods]
impl PySquare {
    /// Creates a new square from an integer (0-63) or a string (e.g. "e4")
    #[new]
    fn new(square: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Check if the input is an integer
        if let Ok(index) = square.extract::<u8>() {
            return PySquare::from_index(index);
        }
        // Try to extract the square as a string (e.g. "e4")
        else if let Ok(square_name) = square.extract::<&str>() {
            return PySquare::from_name(square_name);
        }
        // If the input is neither an integer nor a string, return an error
        Err(PyValueError::new_err(
            "Square must be an integer (0-63) or a string (e.g. \"e4\")",
        ))
    }

    /// Get the index of the square (0-63)
    #[inline]
    fn get_index(&self) -> u8 {
        self.0.to_int()
    }

    /// Create a new square from an index
    #[inline]
    #[staticmethod]
    fn from_index(index: u8) -> PyResult<Self> {
        if index > 63 {
            return Err(PyValueError::new_err(
                "Square index must be between 0 and 63",
            ));
        }
        Ok(PySquare(unsafe { chess::Square::new(index) }))
    }

    // TODO: from_rank_file
    /// Create a new square from a rank and file
    #[inline]
    #[staticmethod]
    fn from_rank_file(rank: u8, file: u8) -> PyResult<Self> {
        if rank > 7 || file > 7 {
            return Err(PyValueError::new_err(
                "Rank and file must be between 0 and 7",
            ));
        }
        Ok(PySquare(chess::Square::make_square(
            chess::Rank::from_index(rank as usize),
            chess::File::from_index(file as usize),
        )))
    }

    /// Get the name of the square (e.g. "e4")
    #[inline]
    fn get_name(&self) -> String {
        // Convert the square to a string using the chess crate
        self.0.to_string()
    }

    /// Get the name of the square (e.g. "e4")
    #[inline]
    fn __str__(&self) -> String {
        self.get_name()
    }

    /// Get the name of the square (e.g. "e4")
    #[inline]
    fn __repr__(&self) -> String {
        self.get_name()
    }

    /// Create a new square from the name (e.g. "e4")
    #[inline]
    #[staticmethod]
    fn from_name(square_name: &str) -> PyResult<Self> {
        // Parse the square using the chess crate
        let square_name = square_name.to_lowercase();
        chess::Square::from_str(&square_name)
            .map(PySquare)
            .map_err(|_| PyValueError::new_err("Invalid square"))
    }

    /// Get the rank of the square (0-7)
    #[inline]
    fn get_rank(&self) -> u8 {
        self.0.get_rank() as u8
    }

    /// Get the file of the square (0-7)
    #[inline]
    fn get_file(&self) -> u8 {
        self.0.get_file() as u8
    }

    /// Returns the square above, otherwise None
    #[inline]
    fn up(&self) -> Option<Self> {
        self.0.up().map(PySquare)
    }

    /// Returns the square below, otherwise None
    #[inline]
    fn down(&self) -> Option<Self> {
        self.0.down().map(PySquare)
    }

    /// Returns the square to the left, otherwise None
    #[inline]
    fn left(&self) -> Option<Self> {
        self.0.left().map(PySquare)
    }

    /// Returns the square to the right, otherwise None
    #[inline]
    fn right(&self) -> Option<Self> {
        self.0.right().map(PySquare)
    }
}

/// Move class
#[gen_stub_pyclass]
#[pyclass(name = "Move")]
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Default, Hash)]
struct PyMove(chess::ChessMove);

#[gen_stub_pymethods]
#[pymethods]
impl PyMove {
    /// Create a new move from a source square, destination square, and optional promotion piece
    #[new]
    #[pyo3(signature = (source, dest, promotion = None))] // Default promotion to None
    fn new(source: PySquare, dest: PySquare, promotion: Option<PyPiece>) -> Self {
        // Parse the move using the chess crate
        PyMove(chess::ChessMove::new(
            source.0,
            dest.0,
            promotion.map(|p| p.0),
        ))
    }

    // TODO: from_san

    /// Create a new move from a UCI string (e.g. "e2e4")
    #[inline]
    #[staticmethod]
    fn from_uci(uci: &str) -> PyResult<Self> {
        // Parse the move using the chess crate
        let uci = uci.to_lowercase();
        chess::ChessMove::from_str(&uci)
            .map(PyMove)
            .map_err(|_| PyValueError::new_err("Invalid UCI move"))
    }

    /// Get the UCI string representation of the move (e.g. "e2e4")
    #[inline]
    fn get_uci(&self) -> String {
        // Convert the move to a UCI string using the chess crate
        self.0.to_string()
    }

    /// Get the UCI string representation of the move (e.g. "e2e4")
    #[inline]
    fn __str__(&self) -> String {
        self.get_uci()
    }

    /// Get the debug representation of the move (e.g. "Move(e2, e4, None)")
    fn __repr__(&self) -> String {
        format!(
            "Move({}, {}, {:?})",
            self.0.get_source(),
            self.0.get_dest(),
            self.0.get_promotion()
        )
    }

    /// Get the source square of the move
    #[inline]
    #[getter]
    fn get_source(&self) -> PySquare {
        PySquare(self.0.get_source())
    }

    /// Get the destination square of the move
    #[inline]
    #[getter]
    fn get_dest(&self) -> PySquare {
        PySquare(self.0.get_dest())
    }

    /// Get the promotion piece of the move
    #[inline]
    #[getter]
    fn get_promotion(&self) -> Option<PyPiece> {
        self.0.get_promotion().map(PyPiece)
    }
}

/// Board class
#[gen_stub_pyclass]
#[pyclass(name = "Board")]
#[derive(Copy, Clone, PartialEq)]
struct PyBoard {
    board: chess::Board,
    #[pyo3(get)] // Get the halfmove clock
    halfmove_clock: u8, // Halfmoves since last pawn move or capture
    #[pyo3(get)] // Get the fullmove number
    fullmove_number: u8, // Fullmove number (increments after black moves)
                         // TODO: Incremental Zobrist hash
}

#[gen_stub_pymethods]
#[pymethods]
impl PyBoard {
    /// Create a new board from a FEN string, otherwise default to the starting position
    #[new]
    #[pyo3(signature = (fen = None))] // Default to None
    fn new(fen: Option<&str>) -> PyResult<Self> {
        match fen {
            // If no FEN string is provided, use the default starting position
            None => Ok(PyBoard {
                board: chess::Board::default(),
                halfmove_clock: 0,
                fullmove_number: 1,
            }),
            // Otherwise, parse the FEN string using the chess crate
            Some(fen_str) => PyBoard::from_fen(fen_str),
        }
    }

    // TODO: Add python doc hints

    /// Get the FEN string representation of the board
    fn get_fen(&self) -> String {
        let base_fen = self.board.to_string();

        // The chess crate does not track the halfmove clock and fullmove number, so we need to add them manually.
        // 0: board, 1: player, 2: castling, 3: en passant, 4: halfmove clock, 5: fullmove number
        let mut parts: Vec<&str> = base_fen.split_whitespace().collect();

        let halfmove_clock_str: String = self.halfmove_clock.to_string();
        let fullmove_number_str: String = self.fullmove_number.to_string();
        parts[4] = halfmove_clock_str.as_str();
        parts[5] = fullmove_number_str.as_str();

        parts.join(" ")
    }

    /// Get the FEN string representation of the board
    #[inline]
    fn __str__(&self) -> String {
        self.get_fen()
    }

    /// Create a new board from a FEN string
    #[staticmethod]
    fn from_fen(fen: &str) -> PyResult<Self> {
        // Extract the halfmove clock and fullmove number from the FEN string
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err(PyValueError::new_err(
                "FEN string must have exactly 6 parts",
            ));
        }

        // Parse the halfmove clock and fullmove number
        let halfmove_clock = parts[4]
            .parse::<u8>()
            .map_err(|_| PyValueError::new_err("Invalid halfmove clock"))?;
        let fullmove_number = parts[5]
            .parse::<u8>()
            .map_err(|_| PyValueError::new_err("Invalid fullmove number"))?;

        // Parse the board using the chess crate
        let board =
            chess::Board::from_str(fen).map_err(|e| PyValueError::new_err(format!("Invalid FEN: {e}")))?;

        Ok(PyBoard {
            board,
            halfmove_clock,
            fullmove_number,
        })
    }

    /// Checks if the halfmoves since the last pawn move or capture is >= 100
    /// and the game is ongoing (not checkmate or stalemate)
    #[inline]
    fn is_fifty_moves(&self) -> bool {
        self.halfmove_clock >= 100 && self.board.status() == chess::BoardStatus::Ongoing
    }

    /// Checks if the side to move is in check
    #[inline]
    fn is_check(&self) -> bool {
        *self.board.checkers() != chess::EMPTY
    }

    // fn make_move(&self)
}

// Define the Python module
#[pymodule]
fn rust_chess(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyColor>()?;
    module.add_class::<PyPiece>()?;
    module.add_class::<PySquare>()?;
    module.add_class::<PyMove>()?;
    module.add_class::<PyBoard>()?;

    // Add the constants to the module

    // Add the color constants
    module.add("WHITE", WHITE)?;
    module.add("BLACK", BLACK)?;
    module.add("COLORS", COLORS)?;

    // Add the piece constants
    module.add("PAWN", PAWN)?;
    module.add("KNIGHT", KNIGHT)?;
    module.add("BISHOP", BISHOP)?;
    module.add("ROOK", ROOK)?;
    module.add("QUEEN", QUEEN)?;
    module.add("KING", KING)?;
    module.add("PIECES", PIECES)?;

    // Define a macro to add square constants directly to the module (e.g. A1, A2, etc.)
    macro_rules! add_square_constants {
        ($module:expr, $($name:ident),*) => {
            $(
                $module.add(stringify!($name), PySquare(chess::Square::$name))?;
            )*
        }
    }

    // Add all square constants directly to the module
    #[rustfmt::skip]
    add_square_constants!(module,
        A1, A2, A3, A4, A5, A6, A7, A8,
        B1, B2, B3, B4, B5, B6, B7, B8,
        C1, C2, C3, C4, C5, C6, C7, C8,
        D1, D2, D3, D4, D5, D6, D7, D8,
        E1, E2, E3, E4, E5, E6, E7, E8,
        F1, F2, F3, F4, F5, F6, F7, F8,
        G1, G2, G3, G4, G5, G6, G7, G8,
        H1, H2, H3, H4, H5, H6, H7, H8
    );

    Ok(())
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
