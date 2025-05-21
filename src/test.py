import rust_chess as ch
import chess

def test_rust_chess():
    color = ch.WHITE
    color2 = ch.COLORS[1]
    print(color)
    print(color2)
    print(not color2)
    print()

    pawn = ch.PAWN
    print(pawn)
    print(pawn.get_string())
    print(pawn.get_index())
    print()

    square = ch.Square(12)
    square2 = ch.Square("E2")
    square3 = ch.A3
    print(square2)
    print(square2.get_name())
    print(square2.get_index())
    print(square2.get_rank())
    print(square2.get_file())
    print(square2.up())
    print(square2.down())
    print(square2.left())
    print(square2.right())
    print()

    move = ch.Move(ch.Square(12), ch.Square(28))
    move2 = ch.Move.from_uci("E2e4")
    print(move2)
    print(move2.get_uci())
    print(move2.source)
    print(move2.dest)
    print(move2.promotion)
    print()

    board = ch.Board()
    board2 = ch.Board("rnbqkbnr/ppp1p1pp/5p2/3p4/4P3/3P4/PPP1KPPP/RNBQ1BNR b kq - 1 3")
    print(board2)
    print(board2.get_fen())
    print(board2.halfmove_clock)
    print(board2.fullmove_number)
    print(board2.turn)
    print(board2.is_fifty_moves())
    print(board2.is_check())
    print(board.is_legal_move(move))
    print(board2.is_legal_move(move2))

    print(board.is_zeroing(move)) # Pawn move
    print(board2.is_zeroing(ch.Move.from_uci("e2e3")))

    print(board.get_piece_type_on(ch.E2))
    print(board.get_color_on(ch.E2))
    print(board.get_piece_on(ch.E4))
    print(board2.get_piece_on(ch.E2))

    board3 = board.make_move_new(move) # Pawn move
    print(board3)
    board.make_move(ch.Move.from_uci("g1f3"), check_legality=True) # Horse move
    print(board)
    # board4 = board2.make_move_new(move2, check_legality=True) # Will panic
    # print(board4)

    print(board.next_legal_capture())
    print(board.next_legal_move())

def test_chess():
    color = chess.WHITE
    color2 = chess.COLORS[1]
    print(color)
    print(color2)
    print(not color2)
    print()

    pawn = chess.PAWN
    print(pawn)
    print()

    square = chess.Square(12)
    square2 = chess.parse_square("e2")
    square3 = chess.A3
    print(square2)
    print(chess.square_name(square2))
    print(chess.square_file(square2))
    print(chess.square_rank(square2))
    print()

    move = chess.Move(chess.Square(12), chess.Square(28))
    move2 = chess.Move.from_uci("e8d7") # King move
    print(move2)
    print(move2.uci())
    print(move2.from_square)
    print(move2.to_square)
    print(move2.promotion)
    print()

    board = chess.Board()
    board2 = chess.Board("rnbqkbnr/ppp1p1pp/5p2/3p4/4P3/3P4/PPP1KPPP/RNBQ1BNR b kq - 1 3")
    print(board2)
    print(board2.fen())
    print(board2.halfmove_clock)
    print(board2.fullmove_number)
    print(board2.turn)
    print(board2.is_fifty_moves())
    print(board2.is_check())
    print(board.is_legal(move))
    print(board2.is_legal(move2))

    print(board.is_zeroing(move)) # Pawn move
    print(board2.is_zeroing(chess.Move.from_uci("e8d7"))) # King move

    print(board.piece_type_at(chess.E2))
    print(board.color_at(chess.E2))
    print(board.piece_at(chess.E4))
    print(board2.piece_at(chess.E2))

    board.push(move) # Pawn move
    print(board)
    board.pop()
    board.push(chess.Move.from_uci("g1f3")) # Horse move
    print(board)

if __name__ == "__main__":
    n = 1
    n = 100_000

    for i in range(n):
        test_rust_chess() # Around 2.3 times faster python chess :)

    print("---------------------------------------")

    # for i in range(n):
    #     test_chess() # Biggest slow down is creating with fen, displaying fen, legality, pushing moves
