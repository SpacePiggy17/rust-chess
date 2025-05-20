import rust_chess as ch

n = 1
# n = 100_000

for i in range(n):
    color = ch.WHITE
    color2 = ch.COLORS[1]
    print(color)
    print(color2)
    print(not color2)
    print()

    pawn = ch.PAWN
    print(pawn)
    print(pawn.to_string())
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
    move2 = ch.Move.from_uci("E2E4")
    print(move2)
    print(move2.get_uci())
    print(move2.source)
    print(move2.dest)
    print(move2.promotion)
    print()

    board = ch.Board()
    board = ch.Board("rnbqkbnr/ppp1p1pp/5p2/3p4/4P3/3P4/PPP1KPPP/RNBQ1BNR b kq - 1 3")
    print(board)
    print(board.get_fen())
    print(board.halfmove_clock)
    print(board.fullmove_number)
    print(board.is_fifty_moves())
    print(board.is_check())

print("---------------------------------------")

import chess
for i in range(n):
    pawn = chess.PAWN
    print(pawn)
    print()

    square = chess.Square(12)
    square2 = chess.parse_square("e2")
    print(square2)
    print(chess.square_name(square2))
    print(chess.square_file(square2))
    print(chess.square_rank(square2))
    print()

    move = chess.Move(chess.Square(12), chess.Square(28))
    move2 = chess.Move.from_uci("e2e4")
    print(move2)
    print(move2.uci())
    print(move2.from_square)
    print(move2.to_square)
    print(move2.promotion)
    print()

    board = chess.Board()
    board = chess.Board("rnbqkbnr/ppp1p1pp/5p2/3p4/4P3/3P4/PPP1KPPP/RNBQ1BNR b kq - 1 3")
    print(board)
    print(board.fen())
    print(board.halfmove_clock)
    print(board.fullmove_number)
    print(board.is_fifty_moves())
    print(board.is_check())
