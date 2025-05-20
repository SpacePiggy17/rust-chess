// PyO3 does not support "self" input parameters, only "&self"
#![allow(clippy::trivially_copy_pass_by_ref)]

use std::str::FromStr;

use chess::Board;
use pyo3::{exceptions::PyValueError, prelude::*, types::PyAny};

/// Color enum
#[pyclass(name = "Color")]
#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Hash)]
struct PyColor(chess::Color);

#[pymethods]
impl PyColor {
    #[classattr]
    const WHITE: PyColor = PyColor(chess::Color::White);
    #[classattr]
    const BLACK: PyColor = PyColor(chess::Color::Black);

    #[classattr]
    const COLORS: [PyColor; 2] = [PyColor::WHITE, PyColor::BLACK];

    /// Get the color as a string
    #[inline]
    fn __str__(&self) -> String {
        if *self == PyColor::WHITE {
            "WHITE".to_string()
        } else {
            "BLACK".to_string()
        }
    }

    /// Get the color as a bool string
    #[inline]
    fn __repr__(&self) -> String {
        if *self == PyColor::WHITE {
            "True".to_string()
        } else {
            "False".to_string()
        }
    }
}

/// Piece enum
#[pyclass(name = "Piece")]
#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
struct PyPiece(chess::Piece);

#[pymethods]
impl PyPiece {
    #[classattr]
    const PAWN: PyPiece = PyPiece(chess::Piece::Pawn);
    #[classattr]
    const KNIGHT: PyPiece = PyPiece(chess::Piece::Knight);
    #[classattr]
    const BISHOP: PyPiece = PyPiece(chess::Piece::Bishop);
    #[classattr]
    const ROOK: PyPiece = PyPiece(chess::Piece::Rook);
    #[classattr]
    const QUEEN: PyPiece = PyPiece(chess::Piece::Queen);
    #[classattr]
    const KING: PyPiece = PyPiece(chess::Piece::King);

    #[classattr]
    const PIECES: [PyPiece; 6] = [
        PyPiece::PAWN,
        PyPiece::KNIGHT,
        PyPiece::BISHOP,
        PyPiece::ROOK,
        PyPiece::QUEEN,
        PyPiece::KING,
    ];

    /// Get the index of the piece (0-5)
    #[inline]
    fn get_index(&self) -> u8 {
        self.0 as u8
    }

    // Convert the piece to a string
    #[inline]
    #[allow(clippy::wrong_self_convention, clippy::inherent_to_string)]
    fn to_string(&self) -> String {
        match *self {
            PyPiece::PAWN => "PAWN".to_string(),
            PyPiece::KNIGHT => "KNIGHT".to_string(),
            PyPiece::BISHOP => "BISHOP".to_string(),
            PyPiece::ROOK => "ROOK".to_string(),
            PyPiece::QUEEN => "QUEEN".to_string(),
            PyPiece::KING => "KING".to_string(),
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
#[pyclass(name = "Square")]
#[derive(PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Default, Hash)]
struct PySquare(chess::Square);

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
#[pyclass(name = "Move")]
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Default, Hash)]
struct PyMove(chess::ChessMove);

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
#[pyclass(name = "Board")]
#[derive(Copy, Clone, PartialEq)]
struct PyBoard {
    board: Board,
    #[pyo3(get)] // Get the halfmove clock
    halfmove_clock: u8, // Halfmoves since last pawn move or capture
    #[pyo3(get)] // Get the fullmove number
    fullmove_number: u8, // Fullmove number (increments after black moves)
                         // TODO: Incremental Zobrist hash
}

#[pymethods]
impl PyBoard {
    /// Create a new board from a FEN string, otherwise default to the starting position
    #[new]
    #[pyo3(signature = (fen = None))] // Default to None
    fn new(fen: Option<&str>) -> PyResult<Self> {
        match fen {
            // If no FEN string is provided, use the default starting position
            None => Ok(PyBoard {
                board: Board::default(),
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
            Board::from_str(fen).map_err(|e| PyValueError::new_err(format!("Invalid FEN: {e}")))?;

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
fn rust_chess(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyColor>()?;
    m.add_class::<PyPiece>()?;
    m.add_class::<PySquare>()?;
    m.add_class::<PyMove>()?;
    m.add_class::<PyBoard>()?;

    Ok(())
}
