package st.netb.chess.fry;

import st.netb.chess.lib.Fen;
import st.netb.chess.lib.Piece;

import java.awt.*;
import java.util.Map;

public class Board {

	private Map<Point, Piece> pieces;
	private Check check;
	private Point enPassant;
	private int score;
	private Fen.CastlingMoves[] castlingMoves;
	private Turn turn;

	public Board(Map<Point, Piece> pieces, Check check, Point enPassant, int score, Fen.CastlingMoves[] castlingMoves, Turn turn) {
		this.pieces = pieces;
		this.check = check;
		this.enPassant = enPassant;
		this.score = score;
		this.castlingMoves = castlingMoves;
		this.turn = turn;
	}

	public Turn getTurn() {
		return turn;
	}

	public void setTurn(Turn turn) {
		this.turn = turn;
	}

	public enum Check {
		WHITE_CHECK,
		BLACK_CHECK,
		NO_CHECK
	}

	public enum Turn {
		WHITE,
		BLACK
	}

	public Piece getPiece(Point position) {
		return pieces.get(position);
	}

	public Map<Point, Piece> getPieces() {
		return pieces;
	}

	public void setPieces(Map<Point, Piece> pieces) {
		this.pieces = pieces;
	}

	public Check getCheck() {
		return check;
	}

	public void setCheck(Check check) {
		this.check = check;
	}

	public Point getEnPassant() {
		return enPassant;
	}

	public void setEnPassant(Point enPassant) {
		this.enPassant = enPassant;
	}

	public int getScore() {
		return score;
	}

	public void setScore(int score) {
		this.score = score;
	}

	public Fen.CastlingMoves[] getCastlingMoves() {
		return castlingMoves;
	}

	public void setCastlingMoves(Fen.CastlingMoves[] castlingMoves) {
		this.castlingMoves = castlingMoves;
	}

	public static Board getStartingBoard() throws FenException {
		return Fen.getBoard("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	}

	@Override
	public String toString() {

		StringBuilder sb = new StringBuilder();

		for (int rank = 0; rank < 8; rank++) {
			sb.append(String.format(" %d | ", rank));
			for (int file = 0; file < 8; file++) {
				Point p = new Point(file, rank);
				Piece piece = getPiece(p);

				char symbol = '.';
				if (piece != null) {
					switch (piece.getKind()) {
						case PAWN:
							symbol = 'p'; break;
						case BISHOP:
							symbol = 'b'; break;
						case KNIGHT:
							symbol = 'n'; break;
						case ROOK:
							symbol = 'r'; break;
						case QUEEN:
							symbol = 'q'; break;
						case KING:
							symbol = 'k'; break;
					}
					if (piece.getColor() == Piece.Color.WHITE) {
						symbol = Character.toUpperCase(symbol);
					}
				}
				sb.append(String.format(" %c ", symbol));
			}
			sb.append("\n");
		}

		sb.append("      ");
		for (int file = 0; file < 8; file++) {
			sb.append(String.format(" %d ", file));
		}
		sb.append("\n");

		return sb.toString();
	}
}
