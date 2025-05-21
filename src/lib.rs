// PyO3 does not support "self" input parameters, only "&self"
#![allow(clippy::trivially_copy_pass_by_ref)]

use std::str::FromStr;

use pyo3::{exceptions::PyValueError, prelude::*, types::PyAny};
use pyo3_stub_gen::{
    define_stub_info_gatherer,
    derive::{gen_stub_pyclass, gen_stub_pymethods},
};

// TODO: Figure out auto stub for constants

// Color constants
const WHITE: PyColor = PyColor(chess::Color::White);
const BLACK: PyColor = PyColor(chess::Color::Black);
const COLORS: [PyColor; 2] = [WHITE, BLACK];

// Piece constants
const PAWN: PyPieceType = PyPieceType(chess::Piece::Pawn);
const KNIGHT: PyPieceType = PyPieceType(chess::Piece::Knight);
const BISHOP: PyPieceType = PyPieceType(chess::Piece::Bishop);
const ROOK: PyPieceType = PyPieceType(chess::Piece::Rook);
const QUEEN: PyPieceType = PyPieceType(chess::Piece::Queen);
const KING: PyPieceType = PyPieceType(chess::Piece::King);
const PIECES: [PyPieceType; 6] = [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING];

/// Color enum class.
///
/// ```python
/// >>> color = rust_chess.WHITE
///
/// >>> color
/// True
/// >>> print(color)
/// WHITE
/// >>> color == rust_chess.BLACK
/// False
/// >>> color == (not rust_chess.BLACK)
/// True
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "Color")]
#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Hash)]
struct PyColor(chess::Color);

#[gen_stub_pymethods]
#[pymethods]
impl PyColor {
    /// Get the color as a string.
    ///
    /// ```python
    /// >>> rust_chess.WHITE.get_string()
    /// 'WHITE'
    /// >>> rust_chess.BLACK.get_string()
    /// 'BLACK'
    /// ```
    #[inline]
    fn get_string(&self) -> String {
        if *self == WHITE {
            "WHITE".to_string()
        } else {
            "BLACK".to_string()
        }
    }

    /// Get the color as a string.
    ///
    /// ```python
    /// >>> print(rust_chess.WHITE)
    /// WHITE
    /// >>> print(rust_chess.BLACK)
    /// BLACK
    /// ```
    #[inline]
    fn __str__(&self) -> String {
        self.get_string()
    }

    /// Get the color as a boolean.
    ///
    /// ```python
    /// >>> bool(rust_chess.WHITE)
    /// True
    /// >>> bool(rust_chess.BLACK)
    /// False
    /// ```
    #[inline]
    fn __bool__(&self) -> bool {
        *self == WHITE
    }

    /// Get the color as a bool string.
    ///
    /// ```python
    /// >>> rust_chess.WHITE
    /// True
    /// >>> rust_chess.BLACK
    /// False
    /// ```
    #[inline]
    fn __repr__(&self) -> String {
        if self.__bool__() {
            "True".to_string()
        } else {
            "False".to_string()
        }
    }

    /// Compare the color to another color or boolean.
    ///
    /// ```python
    /// >>> rust_chess.WHITE == rust_chess.BLACK
    /// False
    /// >>> rust_chess.WHITE == True
    /// True
    /// ```
    #[inline]
    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        if let Ok(other_bool) = other.extract::<bool>() {
            self.__bool__() == other_bool
        } else if let Ok(other_color) = other.extract::<PyColor>() {
            self.__bool__() == other_color.__bool__()
        } else {
            false
        }
    }
}

/// Piece type enum class.
///
/// ```python
/// >>> piece = rust_chess.PAWN
///
/// >>> print(piece)
/// P
/// >>> piece == rust_chess.PAWN
/// True
/// >>> piece == rust_chess.KNIGHT
/// False
/// >>> piece.get_index()
/// 0
/// >>> piece < rust_chess.KNIGHT
/// True
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "PieceType")]
#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
struct PyPieceType(chess::Piece);

#[gen_stub_pymethods]
#[pymethods]
impl PyPieceType {
    /// Get the index of the piece (0-5).
    ///
    /// ```python
    /// >>> rust_chess.BISHOP.get_index()
    /// 2
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn get_index(&self) -> u8 {
        self.0.to_index() as u8
    }

    /// Convert the piece to a string.
    ///
    /// ```python
    /// >>> rust_chess.PAWN.get_string()
    /// P
    /// ```
    #[inline]
    #[pyo3(signature = (color = WHITE))]
    fn get_string(&self, color: PyColor) -> String {
        self.0.to_string(color.0)
    }

    /// Convert the piece to a string.
    ///
    /// ```python
    /// >>> print(rust_chess.PAWN)
    /// P
    /// ```
    #[inline]
    fn __str__(&self) -> String {
        self.get_string(WHITE)
    }

    /// Convert the piece to a string.
    ///
    /// ```python
    /// >>> rust_chess.PAWN
    /// P
    /// ```
    #[inline]
    fn __repr__(&self) -> String {
        self.get_string(WHITE)
    }

    // TODO: Implement __lt__, __le__, __gt__, __ge__ for comparison with mega compare
}

/// Piece class
#[gen_stub_pyclass]
#[pyclass(name = "Piece")]
#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Hash)]
struct PyPiece {
    piece_type: PyPieceType,
    color: PyColor,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyPiece {
    /// Create a new piece from a piece type and color
    #[new]
    fn new(piece_type: PyPieceType, color: PyColor) -> Self {
        PyPiece { piece_type, color }
    }

    ///  Get the index of the piece (0-5)
    #[inline]
    fn get_index(&self) -> u8 {
        self.piece_type.get_index()
    }

    /// Convert the piece to a string
    #[inline]
    fn get_string(&self) -> String {
        self.piece_type.get_string(self.color)
    }

    /// Convert the piece to a string
    #[inline]
    fn __str__(&self) -> String {
        self.get_string()
    }

    /// Convert the piece to a string
    #[inline]
    fn __repr__(&self) -> String {
        self.get_string()
    }

    /// Get the piece type of the piece
    #[getter]
    #[inline]
    fn get_piece_type(&self) -> PyPieceType {
        self.piece_type
    }

    /// Get the color of the piece
    #[getter]
    #[inline]
    fn get_color(&self) -> PyColor {
        self.color
    }

    // TODO: Implement __lt__, __le__, __gt__, __ge__ for comparison with mega compare
}

/// Square class.
///
/// ```python
/// >>> square = rust_chess.Square(0)
/// >>> square
/// a1
/// >>> print(square)
/// a1
/// >>> square == rust_chess.Square("a1")
/// True
/// >>> square == rust_chess.A1
/// True
/// >>> square.get_index()
/// 0
/// >>> rust_chess.A4 == 24
/// True
/// >>> rust_chess.G4.get_rank()
/// 3
/// >>> rust_chess.G4.get_file()
/// 6
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "Square")]
#[derive(PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Default, Hash)]
struct PySquare(chess::Square);

#[gen_stub_pymethods]
#[pymethods]
impl PySquare {
    /// Creates a new square from an integer (0-63) or a string (e.g. "e4").
    ///
    /// ```python
    /// >>> rust_chess.Square(0)
    /// a1
    /// >>> rust_chess.Square("e4")
    /// e4
    /// ```
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

    /// Get the index of the square (0-63).
    ///
    /// ```python
    /// >>> rust_chess.Square("e4").get_index()
    /// 28
    /// ```
    #[inline]
    fn get_index(&self) -> u8 {
        self.0.to_int()
    }

    /// Create a new square from an index.
    ///
    /// ```python
    /// >>> rust_chess.Square.from_index(0)
    /// a1
    /// ```
    #[staticmethod]
    #[inline]
    fn from_index(index: u8) -> PyResult<Self> {
        if index > 63 {
            return Err(PyValueError::new_err(
                "Square index must be between 0 and 63",
            ));
        }
        Ok(PySquare(unsafe { chess::Square::new(index) }))
    }

    /// Create a new square from a rank and file.
    ///
    /// ```python
    /// >>> rust_chess.Square.from_rank_file(0, 3)
    /// d1
    /// ```
    #[staticmethod]
    #[inline]
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

    /// Get the name of the square (e.g. "e4").
    ///
    /// ```python
    /// >>> rust_chess.E4.get_name()
    /// 'e4'
    /// ```
    #[inline]
    fn get_name(&self) -> String {
        // Convert the square to a string using the chess crate
        self.0.to_string()
    }

    /// Get the name of the square (e.g. "e4"),
    ///
    /// ```python
    /// >>> print(rust_chess.E4)
    /// e4
    /// ```
    #[inline]
    fn __str__(&self) -> String {
        self.get_name()
    }

    /// Get the name of the square (e.g. "e4").
    ///
    /// ```python
    /// >>> rust_chess.E4
    /// e4
    /// ```
    #[inline]
    fn __repr__(&self) -> String {
        self.get_name()
    }

    /// Create a new square from a name (e.g. "e4").
    ///
    /// ```python
    /// >>> rust_chess.Square.from_name("d2")
    /// d2
    /// ```
    #[staticmethod]
    #[inline]
    fn from_name(square_name: &str) -> PyResult<Self> {
        // Parse the square using the chess crate
        let square_name = square_name.to_lowercase();
        chess::Square::from_str(&square_name)
            .map(PySquare)
            .map_err(|_| PyValueError::new_err("Invalid square"))
    }

    /// Compare the square to another square or integer.
    ///
    /// ```python
    /// >>> rust_chess.Square("d2") == rust_chess.D2
    /// True
    /// >>> rust_chess.Square("d2") == 11
    /// True
    /// ```
    #[inline]
    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        if let Ok(other_index) = other.extract::<u8>() {
            self.get_index() == other_index
        } else if let Ok(other_square) = other.extract::<PySquare>() {
            self.0 == other_square.0
        } else {
            false
        }
    }

    /// Get the rank of the square as an integer (0-7).
    ///
    /// ```python
    /// >>> rust_chess.E4.get_rank()
    /// 3
    /// ```
    #[inline]
    fn get_rank(&self) -> u8 {
        self.0.get_rank() as u8
    }

    /// Get the file of the square as an integer (0-7).
    ///
    /// ```python
    /// >>> rust_chess.E4.get_file()
    /// 4
    /// ```
    #[inline]
    fn get_file(&self) -> u8 {
        self.0.get_file() as u8
    }

    /// Returns the square above, otherwise None.
    ///
    /// ```python
    /// >>> rust_chess.H5.up()
    /// h6
    /// ```
    #[inline]
    fn up(&self) -> Option<Self> {
        self.0.up().map(PySquare)
    }

    /// Returns the square below, otherwise None.
    ///
    /// ```python
    /// >>> rust_chess.H5.down()
    /// h4
    /// ```
    #[inline]
    fn down(&self) -> Option<Self> {
        self.0.down().map(PySquare)
    }

    /// Returns the square to the left, otherwise None.
    ///
    /// ```python
    /// >>> rust_chess.H5.left()
    /// g5
    /// ```
    #[inline]
    fn left(&self) -> Option<Self> {
        self.0.left().map(PySquare)
    }

    /// Returns the square to the right, otherwise None
    ///
    /// ```python
    /// >>> rust_chess.H5.right()
    ///
    /// >>> rust_chess.H5.right() == None
    /// True
    /// ```
    #[inline]
    fn right(&self) -> Option<Self> {
        self.0.right().map(PySquare)
    }
}

/// Move class.
///
/// ```python
/// >>> move = rust_chess.Move(rust_chess.A4, rust_chess.B1)
/// >>> move
/// Move(a4, b1, None)
/// >>> print(move)
/// a4b1
/// >>> rust_chess.Move("a2a1q")
/// Move(a2, a1, QUEEN)
/// >>> move.get_uci() == rust_chess.Move.from_uci("a4b1") // FIXME
/// True
/// >>> move.source
/// a2
/// >>> move.dest
/// a4
/// >>> move.promotion
///
/// >>> move.promotion == None
/// True
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "Move")]
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Default, Hash)]
struct PyMove(chess::ChessMove);

#[gen_stub_pymethods]
#[pymethods]
impl PyMove {
    /// Create a new move from a source, destination, and optional promotion piece or UCI string.
    #[new]
    #[pyo3(signature = (source_or_uci, dest = None, promotion = None))] // Default dest (enable UCI option) and promotion to None
    fn new(
        source_or_uci: &Bound<'_, PyAny>,
        dest: Option<PySquare>,
        promotion: Option<PyPieceType>,
    ) -> PyResult<Self> {
        // If source_or_uci is a string, treat it as a UCI string
        if let Ok(uci) = source_or_uci.extract::<&str>() {
            return PyMove::from_uci(uci);
        }
        // Otherwise, expect source and destination squares
        if let Ok(source) = source_or_uci.extract::<PySquare>() {
            if let Some(dest) = dest {
                // Create a new move using the chess crate
                return Ok(PyMove(chess::ChessMove::new(
                    source.0,
                    dest.0,
                    promotion.map(|p| p.0),
                )));
            }
        }
        // If we reach here, the input was invalid
        Err(PyValueError::new_err("Move must be a UCI string or a source and destination square with optional promotion piece type"))
    }

    // TODO: from_san

    /// Create a new move from a UCI string (e.g. "e2e4").
    ///
    /// ```python
    /// >>> rust_chess.Move.from_uci("e2e4")
    /// Move(e2, e4, None)
    /// ```
    #[staticmethod]
    #[inline]
    fn from_uci(uci: &str) -> PyResult<Self> {
        // Parse the move using the chess crate
        let uci = uci.to_lowercase();
        chess::ChessMove::from_str(&uci)
            .map(PyMove)
            .map_err(|_| PyValueError::new_err("Invalid UCI move"))
    }

    /// Get the UCI string representation of the move (e.g. "e2e4").
    ///
    /// ```python
    /// >>> move = rust_chess.Move(rust_chess.A2, rust_chess.A4)
    /// >>> move.get_uci()
    /// 'a2a4'
    /// ```
    #[inline]
    fn get_uci(&self) -> String {
        // Convert the move to a UCI string using the chess crate
        self.0.to_string()
    }

    /// Get the UCI string representation of the move (e.g. "e2e4").
    ///
    /// ```python
    /// >>> move = rust_chess.Move(rust_chess.A2, rust_chess.A4)
    /// >>> print(move)
    /// a2a4
    /// ```
    #[inline]
    fn __str__(&self) -> String {
        self.get_uci()
    }

    /// Get the debug representation of the move (e.g. "Move(e2, e4, None)").
    ///
    /// ```python
    /// >>> move = rust_chess.Move(rust_chess.A2, rust_chess.A4)
    /// >>> move
    /// Move(e2, e4, None)
    /// ```
    fn __repr__(&self) -> String {
        format!(
            "Move({}, {}, {:?})",
            self.0.get_source(),
            self.0.get_dest(),
            self.0.get_promotion() // FIXME: Don't output Some(<PyPiece>)
        )
    }

    /// Get the source square of the move.
    ///
    /// ```python
    /// >>> move = rust_chess.Move(rust_chess.A2, rust_chess.A4)
    /// >>> move.source
    /// a2
    /// ```
    #[getter]
    #[inline]
    fn get_source(&self) -> PySquare {
        PySquare(self.0.get_source())
    }

    /// Get the destination square of the move.
    ///
    ///
    /// ```python
    /// >>> move = rust_chess.Move(rust_chess.A2, rust_chess.A4)
    /// >>> move.dest
    /// a4
    /// ```
    #[getter]
    #[inline]
    fn get_dest(&self) -> PySquare {
        PySquare(self.0.get_dest())
    }

    /// Get the promotion piece of the move
    ///
    /// ```python
    /// >>> move = rust_chess.Move(rust_chess.A2, rust_chess.A4)
    /// >>> move.promotion
    ///
    /// >>> move.promotion == None
    /// True
    /// ```
    #[getter]
    #[inline]
    fn get_promotion(&self) -> Option<PyPieceType> {
        self.0.get_promotion().map(PyPieceType)
    }

    // Fixme
    // TODO: Don't use get_uci
    // /// Compare the move to another move.
    // ///
    // /// ```python
    // /// >>> move = rust_chess.Move(rust_chess.A2, rust_chess.A4)
    // /// >>> move == rust_chess.Move.from_uci("a2b4")
    // /// True
    // /// ```
    // #[inline]
    // fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
    //     if let Ok(other_move) = other.extract::<PyMove>() {
    //         self.get_uci() == other_move.get_uci()
    //     } else {
    //         false
    //     }
    // }
}

// TODO: Bitboards

/// Board class
#[gen_stub_pyclass]
#[pyclass(name = "Board")]
struct PyBoard {
    board: chess::Board,
    move_gen: chess::MoveGen,

    /// Get the halfmove clock.
    ///
    /// ```python
    /// >>> rust_chess.Board().halfmove_clock
    /// 0
    /// ```
    #[pyo3(get)]
    halfmove_clock: u8, // Halfmoves since last pawn move or capture

    /// Get the fullmove number.
    ///
    /// ```python
    /// >>> rust_chess.Board().fullmove_number
    /// 1
    /// ```
    #[pyo3(get)]
    fullmove_number: u8, // Fullmove number (increments after black moves)
}
// TODO: Incremental Zobrist hash

#[gen_stub_pymethods]
#[pymethods]
impl PyBoard {
    /// Create a new board from a FEN string, otherwise default to the starting position
    #[new]
    #[pyo3(signature = (fen = None))] // Default to None
    fn new(fen: Option<&str>) -> PyResult<Self> {
        match fen {
            // If no FEN string is provided, use the default starting position
            None => {
                let board = chess::Board::default();
                Ok(PyBoard {
                    board,
                    move_gen: chess::MoveGen::new_legal(&board),
                    halfmove_clock: 0,
                    fullmove_number: 1,
                })
            }
            // Otherwise, parse the FEN string using the chess crate
            Some(fen_str) => PyBoard::from_fen(fen_str),
        }
    }

    // TODO: Add python doc hints

    /// Get the FEN string representation of the board.
    ///
    /// ```python
    /// >>> rust_chess.Board().get_fen()
    /// 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
    /// ```
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

    /// Get the FEN string representation of the board.
    ///
    /// ```python
    /// >>> print(rust_chess.Board())
    /// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    /// ```
    #[inline]
    fn __str__(&self) -> String {
        self.get_fen()
    }

    /// Get the FEN string representation of the board.
    ///
    /// ```python
    /// >>> print(rust_chess.Board())
    /// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    /// ```
    #[inline]
    fn __repr__(&self) -> String {
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
        let board = chess::Board::from_str(fen)
            .map_err(|e| PyValueError::new_err(format!("Invalid FEN: {e}")))?;

        Ok(PyBoard {
            board,
            move_gen: chess::MoveGen::new_legal(&board),
            halfmove_clock,
            fullmove_number,
        })
    }

    /// Get the current player to move
    #[getter]
    #[inline]
    fn get_turn(&self) -> PyColor {
        PyColor(self.board.side_to_move())
    }

    /// Get the en passant square, otherwise None
    #[getter]
    #[inline]
    fn get_en_passant(&self) -> Option<PySquare> {
        self.board.en_passant().map(PySquare)
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

    /// Get the piece type on a square, otherwise None
    #[inline]
    fn get_piece_type_on(&self, square: PySquare) -> Option<PyPieceType> {
        // Get the piece on the square using the chess crate
        self.board.piece_on(square.0).map(PyPieceType)
    }

    /// Get the color of the piece on a square, otherwise None
    #[inline]
    fn get_color_on(&self, square: PySquare) -> Option<PyColor> {
        // Get the color of the piece on the square using the chess crate
        self.board.color_on(square.0).map(PyColor)
    }

    /// Get the piece on a square, otherwise None
    #[inline]
    fn get_piece_on(&self, square: PySquare) -> Option<PyPiece> {
        self.get_color_on(square).and_then(|color| {
            self.get_piece_type_on(square)
                .map(|piece_type| PyPiece { piece_type, color })
        })
    }

    /// Check if the move is legal (supposedly slow according to the chess crate)
    #[inline]
    fn is_legal_move(&self, chess_move: PyMove) -> bool {
        // Check if the move is legal using the chess crate
        chess::Board::legal(&self.board, chess_move.0)
    }

    /// Check if a move is a capture or a pawn move (doesn't check for legality)
    #[inline]
    fn is_zeroing(&self, chess_move: PyMove) -> bool {
        self.get_piece_type_on(chess_move.get_source()) == Some(PAWN) // Pawn move
            || self.get_piece_type_on(chess_move.get_dest()).is_some() // Capture (moving piece onto other piece)
    }

    /// Makes a move onto a new board
    #[pyo3(signature = (chess_move, check_legality = false))]
    fn make_move_new(&self, chess_move: PyMove, check_legality: bool) -> PyResult<PyBoard> {
        // If we are checking legality, check if the move is legal
        if check_legality && !self.is_legal_move(chess_move) {
            return Err(PyValueError::new_err("Illegal move"));
        }

        // Make the move onto a new board using the chess crate
        let new_board: chess::Board = self.board.make_move_new(chess_move.0);

        // Update the halfmove clock and fullmove number
        let mut halfmove_clock: u8 = self.halfmove_clock + 1;
        let fullmove_number: u8 = if self.board.side_to_move() == chess::Color::Black {
            self.fullmove_number + 1 // Increment fullmove number if black moves
        } else {
            self.fullmove_number
        };

        // Reset the halfmove clock if the move zeroes (is a capture or pawn move and therefore "zeroes" the halfmove clock)
        if self.is_zeroing(chess_move) {
            halfmove_clock = 0;
        }

        Ok(PyBoard {
            board: new_board,
            move_gen: chess::MoveGen::new_legal(&new_board),
            halfmove_clock,
            fullmove_number,
        })
    }

    /// Makes a move on the current board
    #[pyo3(signature = (chess_move, check_legality = false))]
    fn make_move(&mut self, chess_move: PyMove, check_legality: bool) -> PyResult<()> {
        // Make the move onto a new board
        let board = self.make_move_new(chess_move, check_legality)?;

        // Update the current board
        self.board = board.board;
        self.move_gen = board.move_gen;
        self.halfmove_clock = board.halfmove_clock;
        self.fullmove_number = board.fullmove_number;

        Ok(())
    }
}

// Define the Python module
#[pymodule]
fn rust_chess(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyColor>()?;
    module.add_class::<PyPieceType>()?;
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
