package st.netb.chess.fry;


import st.netb.chess.fry.piece.Piece;

public class Evaluator {
	// https://github.com/alexandersoto/chess-bot/blob/master/chess/evaluation/SimpleEvaluator.java
	private static final int kingval      = 350;
	private static final int queenval     = 900;
	private static final int rookval      = 500;
	private static final int bishopval    = 300;
	private static final int knightval    = 300;
	private static final int pawnval      = 100;


	private static int bishoppos[][] =
			{
					{-5, -5, -5, -5, -5, -5, -5, -5},
					{-5, 10,  5,  8,  8,  5, 10, -5},
					{-5,  5,  3,  8,  8,  3,  5, -5},
					{-5,  3, 10,  3,  3, 10,  3, -5},
					{-5,  3, 10,  3,  3, 10,  3, -5},
					{-5,  5,  3,  8,  8,  3,  5, -5},
					{-5, 10,  5,  8,  8,  5, 10, -5},
					{-5, -5, -5, -5, -5, -5, -5, -5}
			};
	private static int knightpos[][] =
			{
					{-10, -5, -5, -5, -5, -5, -5,-10},
					{ -8,  0,  0,  3,  3,  0,  0, -8},
					{ -8,  0, 10,  8,  8, 10,  0, -8},
					{ -8,  0,  8, 10, 10,  8,  0, -8},
					{ -8,  0,  8, 10, 10,  8,  0, -8},
					{ -8,  0, 10,  8,  8, 10,  0, -8},
					{ -8,  0,  0,  3,  3,  0,  0, -8},
					{-10, -5, -5, -5, -5, -5, -5,-10}
			};

	private static int pawnposWhite[][] =
			{
					{0,  0,  0,  0,  0,  0,  0,  0},
					{0,  0,  0, -5, -5,  0,  0,  0},
					{0,  2,  3,  4,  4,  3,  2,  0},
					{0,  4,  6, 10, 10,  6,  4,  0},
					{0,  6,  9, 10, 10,  9,  6,  0},
					{4,  8, 12, 16, 16, 12,  8,  4},
					{5, 10, 15, 20, 20, 15, 10,  5},
					{0,  0,  0,  0,  0,  0,  0,  0}
			};

	// I created this, should be just a flipped
	// version of the white array
	private static int pawnposBlack[][] =
			{
					{0,  0,  0,  0,  0,  0,  0,  0},
					{5, 10, 15, 20, 20, 15, 10,  5},
					{4,  8, 12, 16, 16, 12,  8,  4},
					{0,  6,  9, 10, 10,  9,  6,  0},
					{0,  4,  6, 10, 10,  6,  4,  0},
					{0,  2,  3,  4,  4,  3,  2,  0},
					{0,  0,  0, -5, -5,  0,  0,  0},
					{0,  0,  0,  0,  0,  0,  0,  0}
			};

	public static int evaluateBoard(Board board) {
		Piece.Color colorToMove = board.getTurn();
		int playerScore = getPiecesScoreOfColor(board, colorToMove);
		int opponentScore = getPiecesScoreOfColor(board, colorToMove == Piece.Color.WHITE ? Piece.Color.BLACK : Piece.Color.WHITE);

		return playerScore - opponentScore;
	}

	public static int getPiecesScoreOfColor(Board board, Piece.Color color) {
		int[][] pawnPos = color == Piece.Color.WHITE ? pawnposWhite : pawnposBlack;
		return board.getPieces().values().stream()
				.filter(piece -> piece.getColor() == color)
				.mapToInt(piece -> {
					switch(piece.getKind()) {
						case PAWN:
							return pawnval + pawnPos[piece.getPosition().x][piece.getPosition().y];
						case BISHOP:
							return bishopval + bishoppos[piece.getPosition().x][piece.getPosition().y];
						case KNIGHT:
							return knightval + knightpos[piece.getPosition().x][piece.getPosition().y];
						case ROOK:
							return rookval;
						case QUEEN:
							return queenval;
						case KING:
							return kingval;
						default:
							return 0;
					}
				}).sum();
	}


}
