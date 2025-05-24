// PyO3 does not support "self" input parameters, only "&self"
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::unused_self)]

use std::str::FromStr;

use pyo3::{basic::CompareOp, exceptions::PyValueError, prelude::*, types::PyAny};
use pyo3_stub_gen::{
    define_stub_info_gatherer,
    derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods},
    module_variable,
};

// TODO: Remove inline for Python-called only?

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
/// White is True, Black is False.
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
#[pyclass(name = "Color", frozen)]
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
    fn get_string(&self) -> &str {
        if *self == WHITE {
            "WHITE"
        } else {
            "BLACK"
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
    fn __str__(&self) -> &str {
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

    /// Get the color as a boolean string.
    ///
    /// ```python
    /// >>> rust_chess.WHITE
    /// True
    /// >>> rust_chess.BLACK
    /// False
    /// ```
    #[inline]
    fn __repr__(&self) -> &str {
        if self.__bool__() {
            "True"
        } else {
            "False"
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
        if let Ok(other_color) = other.extract::<PyColor>() {
            self.__bool__() == other_color.__bool__()
        } else if let Ok(other_bool) = other.extract::<bool>() {
            self.__bool__() == other_bool
        } else {
            false
        }
    }
}

/// Piece type enum class.
/// Represents the different types of chess pieces.
/// Indexing starts at 0 (PAWN) and ends at 5 (KING).
/// Supports comparison and equality.
/// Does not include color.
///
/// `rust_chess` has constants for each piece type (e.g. PAWN, KNIGHT, etc.).
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
#[pyclass(name = "PieceType", frozen, eq, ord)]
#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
struct PyPieceType(chess::Piece);

#[gen_stub_pymethods]
#[pymethods]
impl PyPieceType {
    /// Get the index of the piece.
    /// Ranges from 0 (PAWN) to 5 (KING).
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
    /// Returns the capital piece type letter.
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
    /// Returns the capital piece type letter.
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
    /// Returns the capital piece type letter.
    ///
    /// ```python
    /// >>> rust_chess.PAWN
    /// P
    /// ```
    #[inline]
    fn __repr__(&self) -> String {
        self.get_string(WHITE)
    }
}

/// Piece class.
/// Represents a chess piece with a type and color.
/// Uses the PieceType and Color classes.
/// Supports comparison and equality.
/// A white piece is considered less than a black piece of the same type.
///
/// ```python
/// TODO
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "Piece", frozen, eq, ord)]
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
    #[inline]
    fn new(piece_type: PyPieceType, color: PyColor) -> Self {
        PyPiece { piece_type, color }
    }

    /// Get the index of the piece (0-5)
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
}

/// Bitboard class.
/// Represents a 64-bit unsigned integer.
/// Each bit represents a square on the chessboard.
/// The least-significant bit represents a1, and the most-significant bit represents h8.
/// Supports bitwise operations and iteration.
/// Also supports comparison and equality.
///
#[gen_stub_pyclass]
#[pyclass(name = "Bitboard", eq, ord)]
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Default, Hash)]
struct PyBitboard(chess::BitBoard);

#[gen_stub_pymethods]
#[pymethods]
impl PyBitboard {
    /// Create a new Bitboard from a 64-bit integer or a square
    #[new]
    #[inline]
    fn new(bitboard_or_square: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(square) = bitboard_or_square.extract::<PySquare>() {
            Ok(PyBitboard::from_square(square))
        } else if let Ok(bitboard) = bitboard_or_square.extract::<u64>() {
            Ok(PyBitboard::from_uint(bitboard))
        } else {
            Err(PyValueError::new_err(
                "Bitboard must be a 64-bit integer or a square",
            ))
        }
    }

    /// Create a new Bitboard from a square
    #[staticmethod]
    #[inline]
    fn from_square(square: PySquare) -> Self {
        PyBitboard(chess::BitBoard::from_square(square.0))
    }

    /// Create a new Bitboard from an unsigned 64-bit integer
    #[staticmethod]
    #[inline]
    fn from_uint(bitboard: u64) -> Self {
        PyBitboard(chess::BitBoard(bitboard))
    }

    /// Convert the Bitboard to a square.
    /// This grabs the least-significant square.
    ///
    #[inline]
    fn to_square(&self) -> PySquare {
        PySquare(self.0.to_square())
    }

    /// Convert the Bitboard to an unsigned 64-bit integer
    #[inline]
    fn to_uint(&self) -> u64 {
        self.0 .0
    }

    /// Convert the Bitboard to a string.
    /// Displays the bitboard in an 8x8 grid.
    /// a1 is the top-left corner, h8 is the bottom-right corner.
    /// To make a1 the bottom-left corner and h8 the top-right corner, call `flip_vertical()` on the bitboard.
    /// Very useful for debugging purposes.
    ///
    #[inline]
    fn get_string(&self) -> String {
        self.0.to_string()
    }

    /// Convert the Bitboard to a string.
    /// Displays the bitboard in an 8x8 grid.
    /// a1 is the top-left corner, h8 is the bottom-right corner.
    /// To make a1 the bottom-left corner and h8 the top-right corner, call `flip_vertical()` on the bitboard.
    /// Very useful for debugging purposes.
    ///
    #[inline]
    fn __str__(&self) -> String {
        self.get_string()
    }

    /// Convert the Bitboard to a string.
    /// Displays the bitboard in an 8x8 grid.
    /// a1 is the top-left corner, h8 is the bottom-right corner.
    /// To make a1 the bottom-left corner and h8 the top-right corner, call `flip_vertical()` on the bitboard.
    /// Very useful for debugging purposes.
    ///
    #[inline]
    fn __repr__(&self) -> String {
        self.get_string()
    }

    /// Count the number of squares in the Bitboard
    #[inline]
    fn popcnt(&self) -> u32 {
        self.0.popcnt()
    }

    /// Flip a bitboard vertically.
    /// View it from the opponent's perspective.
    /// Useful for operations that rely on symmetry, like piece-square tables.
    ///
    #[inline]
    fn flip_vertical(&self) -> Self {
        PyBitboard(self.0.reverse_colors())
    }

    /// Return an iterator of the bitboard
    #[inline]
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    /// Get the next square in the Bitboard.
    /// Removes the square from the Bitboard.
    ///
    #[inline]
    fn __next__(&mut self) -> Option<PySquare> {
        self.0.next().map(PySquare)
    }

    // Bitwise operations

    /// Bitwise NOT operation
    #[inline]
    fn __invert__(&self) -> Self {
        PyBitboard(!self.0)
    }

    /// Bitwise AND operation (self & other).
    #[inline]
    fn __and__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(other_bitboard) = other.extract::<PyBitboard>() {
            Ok(PyBitboard(self.0 & other_bitboard.0))
        } else if let Ok(other_u64) = other.extract::<u64>() {
            Ok(PyBitboard::from_uint(self.0 .0 & other_u64))
        } else {
            Err(PyValueError::new_err(
                "Operand must be a Bitboard or an integer",
            ))
        }
    }

    /// Reflected bitwise AND operation (other & self).
    #[inline]
    fn __rand__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__and__(other)
    }

    /// In-place bitwise AND operation (self &= other).
    #[inline]
    fn __iand__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(other_bitboard) = other.extract::<PyBitboard>() {
            self.0 &= other_bitboard.0;
            Ok(())
        } else if let Ok(other_u64) = other.extract::<u64>() {
            self.0 .0 &= other_u64;
            Ok(())
        } else {
            Err(PyValueError::new_err(
                "Operand must be a Bitboard or an integer",
            ))
        }
    }

    /// Bitwise OR operation (self | other).
    #[inline]
    fn __or__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(other_bitboard) = other.extract::<PyBitboard>() {
            Ok(PyBitboard(self.0 | other_bitboard.0))
        } else if let Ok(other_u64) = other.extract::<u64>() {
            Ok(PyBitboard::from_uint(self.0 .0 | other_u64))
        } else {
            Err(PyValueError::new_err(
                "Operand must be a Bitboard or an integer",
            ))
        }
    }

    /// Reflected bitwise OR operation (other | self).
    #[inline]
    fn __ror__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__or__(other)
    }

    /// In-place bitwise OR operation (self |= other).
    #[inline]
    fn __ior__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(other_bitboard) = other.extract::<PyBitboard>() {
            self.0 |= other_bitboard.0;
            Ok(())
        } else if let Ok(other_u64) = other.extract::<u64>() {
            self.0 .0 |= other_u64;
            Ok(())
        } else {
            Err(PyValueError::new_err(
                "Operand must be a Bitboard or an integer",
            ))
        }
    }

    /// Bitwise XOR operation (self ^ other).
    #[inline]
    fn __xor__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(other_bitboard) = other.extract::<PyBitboard>() {
            Ok(PyBitboard(self.0 ^ other_bitboard.0))
        } else if let Ok(other_u64) = other.extract::<u64>() {
            Ok(PyBitboard::from_uint(self.0 .0 ^ other_u64))
        } else {
            Err(PyValueError::new_err(
                "Operand must be a Bitboard or an integer",
            ))
        }
    }

    /// Reflected bitwise XOR operation (other ^ self).
    #[inline]
    fn __rxor__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__xor__(other)
    }

    /// In-place bitwise XOR operation (self ^= other).
    #[inline]
    fn __ixor__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(other_bitboard) = other.extract::<PyBitboard>() {
            self.0 ^= other_bitboard.0;
            Ok(())
        } else if let Ok(other_u64) = other.extract::<u64>() {
            self.0 .0 ^= other_u64;
            Ok(())
        } else {
            Err(PyValueError::new_err(
                "Operand must be a Bitboard or an integer",
            ))
        }
    }

    /// Multiplication operation (self * other).
    #[inline]
    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(other_bitboard) = other.extract::<PyBitboard>() {
            Ok(PyBitboard(self.0 * other_bitboard.0))
        } else if let Ok(other_u64) = other.extract::<u64>() {
            Ok(PyBitboard::from_uint(self.0 .0 * other_u64))
        } else {
            Err(PyValueError::new_err(
                "Operand must be a Bitboard or an integer",
            ))
        }
    }

    /// Reflected multiplication operation (other * self).
    #[inline]
    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__mul__(other)
    }

    /// In-place multiplication operation (self *= other).
    #[inline]
    fn __imul__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(other_bitboard) = other.extract::<PyBitboard>() {
            self.0 = self.0 * other_bitboard.0;
            Ok(())
        } else if let Ok(other_u64) = other.extract::<u64>() {
            self.0 .0 *= other_u64;
            Ok(())
        } else {
            Err(PyValueError::new_err(
                "Operand must be a Bitboard or an integer",
            ))
        }
    }

    /// Left shift operation (self << shift).
    #[inline]
    fn __lshift__(&self, shift: u32) -> Self {
        PyBitboard::from_uint(self.0 .0 << shift)
    }

    /// Reflected left shift operation (not typically used)
    #[inline]
    fn __rlshift__(&self, _other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Err(PyValueError::new_err(
            "Cannot perform shift with Bitboard on right",
        ))
    }

    /// In-place left shift operation (self <<= shift).
    #[inline]
    fn __ilshift__(&mut self, shift: u32) {
        self.0 .0 <<= shift;
    }

    /// Right shift operation (self >> shift).
    #[inline]
    fn __rshift__(&self, shift: u32) -> Self {
        PyBitboard::from_uint(self.0 .0 >> shift)
    }

    /// Reflected right shift operation (not typically used)
    #[inline]
    fn __rrshift__(&self, _other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Err(PyValueError::new_err(
            "Cannot perform shift with Bitboard on right",
        ))
    }

    /// In-place right shift operation (self >>= shift).
    #[inline]
    fn __irshift__(&mut self, shift: u32) {
        self.0 .0 >>= shift;
    }
}

/// Square class.
/// Represents a square on the chessboard.
/// The square is represented as an integer (0-63) or a string (e.g. "e4").
/// Supports comparison and equality.
///
/// `rust_chess` has constants for each square (e.g. A1, B2, etc.).
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
/// TODO
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "Square", frozen)]
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
    #[inline]
    fn new(square: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(index) = square.extract::<u8>() {
            return PySquare::from_index(index);
        } else if let Ok(square_name) = square.extract::<&str>() {
            return PySquare::from_name(square_name);
        }
        Err(PyValueError::new_err(
            "Square must be an integer (0-63) or a string (e.g. \"e4\")",
        ))
    }

    /// Get the index of the square (0-63).
    /// Indexing starts at 0 (a1) and ends at 63 (h8).
    ///
    /// ```python
    /// >>> rust_chess.Square("e4").get_index()
    /// 28
    /// ```
    #[inline]
    fn get_index(&self) -> u8 {
        self.0.to_int()
    }

    /// Convert a square to a bitboard
    #[inline]
    fn to_bitboard(&self) -> PyBitboard {
        PyBitboard::from_square(*self)
    }

    /// Create a new square from an index.
    /// Indexing starts at 0 (a1) and ends at 63 (h8).
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

    /// Create a new square from rank and file.
    /// Rank and file are 0-indexed (0-7).
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

    /// Get the color of the square on the chessboard
    #[inline]
    fn get_color(&self) -> PyColor {
        // column % 2 == row % 2
        if self.get_file() % 2 == self.get_rank() % 2 {
            WHITE
        } else {
            BLACK
        }
    }

    /// Create a new square from a name (e.g. "e4").
    /// Not really needed since you can use the square constants.
    /// Could also just call the constructor with the name string.
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
    /// >>> rust_chess.G6 > rust_chess.D3
    /// True
    /// >>> rust_chess.G6 <= 56
    /// True
    /// ```
    #[inline]
    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<bool> {
        // Convert self to index
        let self_index = self.get_index();

        // Convert other to index
        let other_index = if let Ok(other_square) = other.extract::<PySquare>() {
            other_square.get_index()
        } else if let Ok(other_index) = other.extract::<u8>() {
            other_index
        } else {
            return Err(PyValueError::new_err(
                "Square must be an integer (0-63) or a Square",
            ));
        };

        Ok(match op {
            CompareOp::Eq => self_index == other_index,
            CompareOp::Ne => self_index != other_index,
            CompareOp::Lt => self_index < other_index,
            CompareOp::Le => self_index <= other_index,
            CompareOp::Gt => self_index > other_index,
            CompareOp::Ge => self_index >= other_index,
        })
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
/// Represents a chess move.
/// The move is represented as a source square, destination square, and optional promotion piece.
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
#[pyclass(name = "Move", frozen, eq)]
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Default, Hash)]
struct PyMove(chess::ChessMove);

#[gen_stub_pymethods]
#[pymethods]
impl PyMove {
    /// Create a new move from a source, destination, and optional promotion piece or UCI string.
    ///
    /// ```python
    /// >>> rust_chess.Move(rust_chess.A2, rust_chess.A4)
    /// (a2, a4, None)
    /// >>> rust_chess.Move("g2g1q")
    /// (g2, g1, QUEEN)
    /// ```
    #[new]
    #[pyo3(signature = (source_or_uci, dest = None, promotion = None))] // Default dest (enable UCI option) and promotion to None
    fn new(
        source_or_uci: &Bound<'_, PyAny>,
        dest: Option<PySquare>,
        promotion: Option<PyPieceType>,
    ) -> PyResult<Self> {
        // Expect source and destination squares
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
        // Otherwise, try treating the first argument as a UCI string
        if let Ok(uci) = source_or_uci.extract::<&str>() {
            return PyMove::from_uci(uci);
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

    /// Get the internal representation of the move (e.g. "Move(e2, e4, None)").
    ///
    /// ```python
    /// >>> move = rust_chess.Move(rust_chess.A2, rust_chess.A4)
    /// >>> move
    /// Move(e2, e4, None)
    /// ```
    #[inline]
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

    /// Get the promotion piece of the move, otherwise None.
    ///
    /// ```python
    /// >>> move = rust_chess.Move(rust_chess.A2, rust_chess.A4)
    /// >>> move.promotion
    ///
    /// >>> move.promotion == None
    /// True
    /// >>> move = rust_chess.Move("g2g1q")
    /// >>> move.promotion
    /// QUEEN
    /// ```
    #[getter]
    #[inline]
    fn get_promotion(&self) -> Option<PyPieceType> {
        self.0.get_promotion().map(PyPieceType)
    }
}

/// Move iterator class for generating legal moves.
/// Not intended for direct use.
/// Use the `Board` class methods for generating moves.
#[gen_stub_pyclass]
#[pyclass(name = "MoveGenerator")]
struct PyMoveGenerator(chess::MoveGen);

#[gen_stub_pymethods]
#[pymethods]
impl PyMoveGenerator {
    /// Return an iterator of the generator
    #[inline]
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    /// Get the next move in the generator
    #[inline]
    fn __next__(&mut self) -> Option<PyMove> {
        self.0.next().map(PyMove)
    }

    /// Get the type of the move generator
    #[inline]
    fn __repr__(&self) -> String {
        "MoveGenerator()".to_string()
    }
}

/// Board status enum class.
/// Represents the status of a chess board.
/// The status can be one of the following:
///     Ongoing, five-fold repetition, seventy-five moves, insufficient material, stalemate, or checkmate.
/// Supports comparison and equality.
///
#[gen_stub_pyclass_enum]
#[pyclass(name = "BoardStatus", frozen, eq, ord)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum PyBoardStatus {
    #[pyo3(name = "ONGOING")]
    Ongoing,
    #[pyo3(name = "FIVE_FOLD_REPETITION")]
    FiveFoldRepetition,
    #[pyo3(name = "SEVENTY_FIVE_MOVES")]
    SeventyFiveMoves,
    #[pyo3(name = "INSUFFICIENT_MATERIAL")]
    InsufficientMaterial,
    #[pyo3(name = "STALEMATE")]
    Stalemate,
    #[pyo3(name = "CHECKMATE")]
    Checkmate,
}

/// Board class.
/// Represents the state of a chess board.
///
#[gen_stub_pyclass]
#[pyclass(name = "Board")]
struct PyBoard {
    board: chess::Board,
    // move_gen: chess::MoveGen,
    move_gen: Py<PyMoveGenerator>, // Use a Py to be able to share between Python and Rust

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
    /// Create a new board from a FEN string, otherwise default to the starting position.
    ///
    /// ```python
    /// >>> rust_chess.Board()
    /// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    /// >>> rust_chess.Board("rnbqkbnr/ppp1pppp/8/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 2")
    /// rnbqkbnr/ppp1pppp/8/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 2
    /// ```
    #[new]
    #[pyo3(signature = (fen = None))] // Default to None
    fn new(fen: Option<&str>) -> PyResult<Self> {
        match fen {
            // If no FEN string is provided, use the default starting position
            None => {
                let board = chess::Board::default();

                // We can assume the GIL is acquired, since this function is only called from Python
                let py = unsafe { Python::assume_gil_acquired() };

                // Create a new move generator using the chess crate
                let move_gen = Py::new(py, PyMoveGenerator(chess::MoveGen::new_legal(&board)))?;

                Ok(PyBoard {
                    board,
                    move_gen,
                    halfmove_clock: 0,
                    fullmove_number: 1,
                })
            }
            // Otherwise, parse the FEN string using the chess crate
            Some(fen_str) => PyBoard::from_fen(fen_str),
        }
    }

    /// Get the FEN string representation of the board.
    ///
    /// ```python
    /// >>> rust_chess.Board().get_fen()
    /// 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
    /// ```
    #[inline]
    fn get_fen(&self) -> String {
        let base_fen = self.board.to_string();

        // 0: board, 1: player, 2: castling, 3: en passant, 4: halfmove clock, 5: fullmove number
        let mut parts: Vec<&str> = base_fen.split_whitespace().collect();

        // The chess crate does not track the halfmove clock and fullmove number correctly, so we need to add them manually.
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

    /// Create a new board from a FEN string.
    ///
    /// ```python
    /// >>> rust_chess.Board.from_fen("rnbqkbnr/ppp1pppp/8/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 2")
    /// rnbqkbnr/ppp1pppp/8/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 2
    /// ```
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

        // We can assume the GIL is acquired, since this function is only called from Python
        let py = unsafe { Python::assume_gil_acquired() };

        // Create a new move generator using the chess crate
        let move_gen = Py::new(py, PyMoveGenerator(chess::MoveGen::new_legal(&board)))?;

        Ok(PyBoard {
            board,
            move_gen,
            halfmove_clock,
            fullmove_number,
        })
    }

    /// Get the current player to move.
    ///
    /// ```python
    /// >>> board = rust_chess.Board()
    /// >>> board.turn
    /// True
    /// >>> print(board.turn)
    /// WHITE
    /// ```
    #[getter]
    #[inline]
    fn get_turn(&self) -> PyColor {
        PyColor(self.board.side_to_move())
    }

    /// Get the en passant square, otherwise None.
    ///
    /// ```python
    /// >>> rust_chess.Board().en_passant
    ///
    /// >>> rust_chess.Board().en_passant == None
    /// True
    /// >>> rust_chess.Board("rnbqkbnr/pp2p1pp/2p5/3pPp2/5P2/8/PPPP2PP/RNBQKBNR w KQkq f6 0 4").en_passant
    /// f5
    /// ```
    #[getter]
    #[inline]
    fn get_en_passant(&self) -> Option<PySquare> {
        self.board.en_passant().map(PySquare)
    }

    /// Get the piece type on a square, otherwise None.
    /// Different than `get_piece_on` because it returns the piece type, which does not include color.
    ///
    /// ```python
    /// >>> rust_chess.Board().get_piece_type_on(rust_chess.A1)
    /// R
    /// >>> rust_chess.Board().get_piece_type_on(rust_chess.E8)
    /// K
    /// ```
    #[inline]
    fn get_piece_type_on(&self, square: PySquare) -> Option<PyPieceType> {
        // Get the piece on the square using the chess crate
        self.board.piece_on(square.0).map(PyPieceType)
    }

    /// Get the color of the piece on a square, otherwise None.
    ///
    /// ```python
    /// >>> rust_chess.Board().get_color_on(rust_chess.A1)
    /// True
    /// >>> print(rust_chess.Board().get_color_on(rust_chess.A1))
    /// WHITE
    /// >>> rust_chess.Board().get_color_on(rust_chess.E8)
    /// False
    #[inline]
    fn get_color_on(&self, square: PySquare) -> Option<PyColor> {
        // Get the color of the piece on the square using the chess crate
        self.board.color_on(square.0).map(PyColor)
    }

    /// Get the piece on a square (color-inclusive), otherwise None.
    /// Different than `get_piece_on` because it returns the piece, which includes color.
    ///
    /// ```python
    /// >>> rust_chess.Board().get_piece_on(rust_chess.A1)
    /// R
    /// >>> rust_chess.Board().get_piece_on(rust_chess.E8)
    /// k
    /// ```
    #[inline]
    fn get_piece_on(&self, square: PySquare) -> Option<PyPiece> {
        self.get_color_on(square).and_then(|color| {
            self.get_piece_type_on(square)
                .map(|piece_type| PyPiece { piece_type, color })
        })
    }

    /// Check if a move is a capture or a pawn move.
    /// Doesn't check legality.
    ///
    #[inline]
    fn is_zeroing(&self, chess_move: PyMove) -> bool {
        self.get_piece_type_on(chess_move.get_source()) == Some(PAWN) // Pawn move
        || self.get_piece_type_on(chess_move.get_dest()).is_some() // Capture (moving piece onto other piece)
    }

    /// Check if the move is legal (supposedly very slow according to the chess crate).
    /// Use this function for moves not generated by the move generator.
    /// `is_legal_quick` is faster for moves generated by the move generator.
    ///
    /// ```python
    /// >>> move = rust_chess.Move("e2e4")
    /// >>> rust_chess.Board().is_legal_move(move)
    /// True
    /// >>> move2 = rust_chess.Move("e2e5")
    /// >>> rust_chess.Board().is_legal_move(move2)
    /// False
    /// ```
    #[inline]
    fn is_legal_move(&self, chess_move: PyMove) -> bool {
        // Check if the move is legal using the chess crate
        chess::Board::legal(&self.board, chess_move.0)
    }

    // TODO: is_legal_quick

    // TODO: make_null_move_new, make_null_move

    /// Make a move onto a new board
    ///
    #[pyo3(signature = (chess_move, check_legality = false))]
    fn make_move_new(&self, chess_move: PyMove, check_legality: bool) -> PyResult<Self> {
        // If we are checking legality, check if the move is legal
        if check_legality && !self.is_legal_move(chess_move) {
            return Err(PyValueError::new_err("Illegal move"));
        }

        // Make the move onto a new board using the chess crate
        let new_board: chess::Board = self.board.make_move_new(chess_move.0);

        // Reset the halfmove clock if the move zeroes (is a capture or pawn move and therefore "zeroes" the halfmove clock)
        let halfmove_clock: u8 = if self.is_zeroing(chess_move) {
            0
        } else {
            self.halfmove_clock + 1
        };

        // Increment fullmove number if black moves
        let fullmove_number: u8 = if self.board.side_to_move() == chess::Color::Black {
            self.fullmove_number + 1
        } else {
            self.fullmove_number
        };

        // We can assume the GIL is acquired, since this function is only called from Python
        let py = unsafe { Python::assume_gil_acquired() };

        // Create a new move generator using the chess crate
        let move_gen = Py::new(py, PyMoveGenerator(chess::MoveGen::new_legal(&new_board)))?;

        Ok(PyBoard {
            board: new_board,
            move_gen,
            halfmove_clock,
            fullmove_number,
        })
    }

    /// Make a move on the current board
    ///
    #[pyo3(signature = (chess_move, check_legality = false))]
    fn make_move(&mut self, chess_move: PyMove, check_legality: bool) -> PyResult<()> {
        // If we are checking legality, check if the move is legal
        if check_legality && !self.is_legal_move(chess_move) {
            return Err(PyValueError::new_err("Illegal move"));
        }

        // Make the move onto a new board using the chess crate
        let temp_board: chess::Board = self.board.make_move_new(chess_move.0);

        // Reset the halfmove clock if the move zeroes (is a capture or pawn move and therefore "zeroes" the halfmove clock)
        self.halfmove_clock = if self.is_zeroing(chess_move) {
            0
        } else {
            self.halfmove_clock + 1
        };

        // Increment fullmove number if black moves
        if self.board.side_to_move() == chess::Color::Black {
            self.fullmove_number += 1;
        }

        // Update the current board
        self.board = temp_board;

        // We can assume the GIL is acquired, since this function is only called from Python
        let py = unsafe { Python::assume_gil_acquired() };

        // Create a new move generator using the chess crate
        self.move_gen = Py::new(py, PyMoveGenerator(chess::MoveGen::new_legal(&temp_board)))?;

        Ok(())
    }

    // TODO: set_iterator_mask, will have to implement PyBitboard
    // TODO: remove_mask

    // Fixme
    // /// Get the number of moves remaining in the move generator.
    // /// This is the number of remaining moves that can be generated.
    // /// The default mask is all legal moves.
    // ///
    // #[inline]
    // fn get_moves_remaining(&self) -> usize {
    //     // We can assume the GIL is acquired, since this function is only called from Python
    //     let py = unsafe { Python::assume_gil_acquired() };
    //
    //     // Get the length of the move generator
    //     self.move_gen.borrow(py).0.len()
    // }

    /// Remove a move from the move generator.
    /// Prevents the move from being generated.
    /// Useful if you already have a certain move and don't need to generate it again.
    ///
    #[inline]
    fn remove_move(&mut self, chess_move: PyMove) {
        // We can assume the GIL is acquired, since this function is only called from Python
        let py = unsafe { Python::assume_gil_acquired() };

        // Remove the move from the generator
        self.move_gen.borrow_mut(py).0.remove_move(chess_move.0);
    }

    /// Reset the move generator for the current board
    #[inline]
    fn reset_move_generator(&mut self) -> PyResult<()> {
        // We can assume the GIL is acquired, since this function is only called from Python
        let py = unsafe { Python::assume_gil_acquired() };

        // Create a new move generator using the chess crate
        self.move_gen = Py::new(py, PyMoveGenerator(chess::MoveGen::new_legal(&self.board)))?;

        Ok(())
    }

    /// Get the next remaining move of the generator.
    /// Updates the move generator to the next move.
    /// Unless the mask is set, this will return the next legal move by default.
    ///
    #[inline]
    fn next_move(&mut self) -> Option<PyMove> {
        // We can assume the GIL is acquired, since this function is only called from Python
        let py = unsafe { Python::assume_gil_acquired() };

        // Get the next move from the generator
        self.move_gen.borrow_mut(py).__next__()
    }

    /// Generate the next remaining legal moves for the current board.
    /// Exhausts the move generator if fully iterated over.
    /// Updates the move generator.
    ///
    #[inline]
    fn generate_legal_moves(&mut self) -> Py<PyMoveGenerator> {
        // We can assume the GIL is acquired, since this function is only called from Python
        let py = unsafe { Python::assume_gil_acquired() };

        // Set the iterator mask to everything (check all legal moves)
        self.move_gen
            .borrow_mut(py)
            .0
            .set_iterator_mask(!chess::EMPTY);

        // Share ownership with Python
        self.move_gen.clone_ref(py)
    }

    #[inline]
    /// Generate the next remaining legal captures for the current board.
    /// Exhausts the move generator if fully iterated over.
    /// Updates the move generator.
    ///
    fn generate_legal_captures(&mut self) -> Py<PyMoveGenerator> {
        // Get the mask of enemy‐occupied squares
        let targets_mask = self.board.color_combined(!self.board.side_to_move());

        // We can assume the GIL is acquired, since this function is only called from Python
        let py = unsafe { Python::assume_gil_acquired() };

        // Set the iterator mask to the targets mask (check all legal captures [moves onto enemy pieces])
        self.move_gen
            .borrow_mut(py)
            .0
            .set_iterator_mask(*targets_mask);

        // Share ownership with Python
        self.move_gen.clone_ref(py)
    }

    /// Checks if the side to move has insufficient material to checkmate the opponent.
    /// The cases where this is true are:
    ///     1. K vs K
    ///     2. K vs K + N
    ///     3. K vs K + B
    ///     4. K + B vs K + B with the bishops on the same color.
    #[inline]
    fn is_insufficient_material(&self) -> bool {
        let kings = self.board.pieces(chess::Piece::King);

        // Get the bitboards of the white and black pieces without the kings
        let white_bb = self.board.color_combined(chess::Color::White) & !kings;
        let black_bb = self.board.color_combined(chess::Color::Black) & !kings;
        let combined_bb = white_bb | black_bb;

        // King vs King: Combined bitboard minus kings is empty
        if combined_bb == chess::EMPTY {
            return true;
        }

        let remaining_num_pieces = combined_bb.popcnt();

        if remaining_num_pieces <= 2 {
            let knights = self.board.pieces(chess::Piece::Knight);
            let bishops = self.board.pieces(chess::Piece::Bishop);

            // King vs King + Knight/Bishop: Combined bitboard minus kings and knight/bishop is empty
            if remaining_num_pieces == 1 && combined_bb & !(knights | bishops) == chess::EMPTY {
                return true;
            } else if *knights == chess::EMPTY {
                // Only bishops left
                let white_bishops = bishops & white_bb;
                let black_bishops = bishops & black_bb;

                if white_bishops != chess::EMPTY && black_bishops != chess::EMPTY // Both sides have a bishop
                    // King + Bishop vs King + Bishop same color: White and black bishops are on the same color square
                    && PySquare(white_bishops.to_square()).get_color() == PySquare(black_bishops.to_square()).get_color()
                {
                    return true;
                }
            }
        }
        false
    }

    /// Checks if the halfmoves since the last pawn move or capture is >= 100
    /// and the game is ongoing (not checkmate or stalemate).
    ///
    /// ```python
    /// >>> rust_chess.Board().is_fifty_moves
    /// False
    /// >>> rust_chess.Board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 50 1").is_fifty_moves()
    /// True
    /// ```
    #[inline]
    fn is_fifty_moves(&self) -> bool {
        self.halfmove_clock >= 100 && self.board.status() == chess::BoardStatus::Ongoing
    }

    /// Checks if the halfmoves since the last pawn move or capture is >= 150
    /// and the game is ongoing (not checkmate or stalemate).
    ///
    #[inline]
    fn is_seventy_five_moves(&self) -> bool {
        self.halfmove_clock >= 150 && self.board.status() == chess::BoardStatus::Ongoing
    }

    // TODO: Check threefold and fivefold repetition
    fn is_fivefold_repetition(&self) -> bool {
        false
    }

    /// Checks if the side to move is in check.
    ///
    /// ```python
    /// >>> rust_chess.Board().is_check
    /// False
    /// >>> rust_chess.Board("rnb1kbnr/pppp1ppp/4p3/8/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3").is_check()
    /// True
    /// ```
    #[inline]
    fn is_check(&self) -> bool {
        *self.board.checkers() != chess::EMPTY
    }

    /// Checks if the side to move is in stalemate
    #[inline]
    fn is_stalemate(&self) -> bool {
        self.board.status() == chess::BoardStatus::Stalemate
    }

    /// Checks if the side to move is in checkmate
    #[inline]
    fn is_checkmate(&self) -> bool {
        self.board.status() == chess::BoardStatus::Checkmate
    }

    /// Get the status of the board
    #[inline]
    fn get_status(&self) -> PyBoardStatus {
        let status = self.board.status();
        match status {
            chess::BoardStatus::Checkmate => PyBoardStatus::Checkmate,
            chess::BoardStatus::Stalemate => PyBoardStatus::Stalemate,
            chess::BoardStatus::Ongoing => {
                if self.is_insufficient_material() {
                    PyBoardStatus::InsufficientMaterial
                } else if self.is_seventy_five_moves() {
                    PyBoardStatus::SeventyFiveMoves
                } else if self.is_fivefold_repetition() {
                    PyBoardStatus::FiveFoldRepetition
                } else {
                    PyBoardStatus::Ongoing
                }
            }
        }
    }
}

// Define the Python module
#[pymodule]
fn rust_chess(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyColor>()?;
    module.add_class::<PyPieceType>()?;
    module.add_class::<PyPiece>()?;
    module.add_class::<PyBitboard>()?;
    module.add_class::<PySquare>()?;
    module.add_class::<PyMove>()?;
    module.add_class::<PyMoveGenerator>()?;
    module.add_class::<PyBoardStatus>()?;
    module.add_class::<PyBoard>()?;

    // Add the constants and stubs to the module

    // Add the color constants and their stubs
    module.add("WHITE", WHITE)?;
    module_variable!("rust_chess", "WHITE", PyColor);
    module.add("BLACK", BLACK)?;
    module_variable!("rust_chess", "BLACK", PyColor);
    module.add("COLORS", COLORS)?;
    module_variable!("rust_chess", "COLORS", Vec<PyColor>);

    // Add the piece constants and their stubs
    module.add("PAWN", PAWN)?;
    module_variable!("rust_chess", "PAWN", PyPieceType);
    module.add("KNIGHT", KNIGHT)?;
    module_variable!("rust_chess", "KNIGHT", PyPieceType);
    module.add("BISHOP", BISHOP)?;
    module_variable!("rust_chess", "BISHOP", PyPieceType);
    module.add("ROOK", ROOK)?;
    module_variable!("rust_chess", "ROOK", PyPieceType);
    module.add("QUEEN", QUEEN)?;
    module_variable!("rust_chess", "QUEEN", PyPieceType);
    module.add("KING", KING)?;
    module_variable!("rust_chess", "KING", PyPieceType);
    module.add("PIECES", PIECES)?;
    module_variable!("rust_chess", "PIECES", Vec<PyPieceType>);

    // Define a macro to add square constants and stubs directly to the module (e.g. A1, A2, etc.)
    macro_rules! add_square_constants {
        ($module:expr, $($name:ident),*) => {
            $(
                $module.add(stringify!($name), PySquare(chess::Square::$name))?;
                module_variable!("rust_chess", stringify!($name), PySquare);
            )*
        }
    }

    // Add all square constants and stubs directly to the module
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
