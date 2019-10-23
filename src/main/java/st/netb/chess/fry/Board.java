package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;
import st.netb.chess.lib.Fen;
import st.netb.chess.lib.FenException;



import java.awt.Point;
import java.util.*;
import java.util.stream.Collectors;

public class Board extends PieceMap {

	private Check check;
	private Point enPassant;
	private int score;
	private List<Fen.CastlingMoves> castlingMoves;
	private Piece.Color turn;

	public Board(List<Piece> pieces, Check check, Point enPassant, int score, List<Fen.CastlingMoves> castlingMoves, Piece.Color turn) {
		pieces.forEach(this::putPiece);
		this.check = check;
		this.enPassant = enPassant;
		this.score = score;
		this.castlingMoves = castlingMoves;
		this.turn = turn;
		//findCheck();
	}

	public Piece.Color getTurn() {
		return turn;
	}

	public Piece.Color getNextTurn() {
		switch (turn) {
			case WHITE:
				return Piece.Color.BLACK;
			case BLACK:
				return Piece.Color.WHITE;
			default:
				throw new IllegalStateException("somehow a color other than black or white exist");
		}
	}

	public void setTurn(Piece.Color turn) {
		this.turn = turn;
	}

	public enum Check {
		WHITE_CHECK,
		BLACK_CHECK,
		NO_CHECK
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

	public List<Fen.CastlingMoves> getCastlingMoves() {
		return castlingMoves;
	}

	public void setCastlingMoves(List<Fen.CastlingMoves> castlingMoves) {
		this.castlingMoves = castlingMoves;
	}

	public static Board getStartingBoard() throws FenException {
		Fen fen = new Fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
		return new Board(
				fen.getPieces().stream().map(Piece::mapFromLibPiece).collect(Collectors.toList()),
				Check.NO_CHECK,
				fen.getEnPassant(),
				0,
				fen.getCastlingAvailability(),
				Piece.mapFromLibColor(fen.getActiveColor()));
	}

	private void findCheck(){
		Optional<Piece> king = getPieces().stream().filter(e -> e.getKind() == Piece.Kind.KING && e.getColor() !=  this.getTurn()).findAny();
		Optional<Piece> otherKing = getPieces().stream().filter(e -> e.getKind() == Piece.Kind.KING && e.getColor() ==  this.getTurn()).findAny();
		Point positionKing = null;
		Point positionOtherKing = null;
		if(king.isPresent()){
			positionKing = king.get().getPosition();
		}
		if(otherKing.isPresent()){
			positionOtherKing = otherKing.get().getPosition();
		}

		for (Piece piece: getPieces()) {
			for(Point point: piece.allPossibleLandingSquares(this)){
				if(point.equals(positionKing)){
					if(this.turn == Piece.Color.BLACK){
						this.check = Check.WHITE_CHECK;
					}else{
						this.check = Check.BLACK_CHECK;
					}
					return;
				}else if(point.equals(positionOtherKing)){
					if(this.turn == Piece.Color.BLACK){
						this.check = Check.BLACK_CHECK;
					}else{
						this.check = Check.WHITE_CHECK;
					}
					return;
				}
			}
		}
	}

	@Override
	public String toString() {

		StringBuilder sb = new StringBuilder();

		for (int rank = 7; rank >= 0; rank--) {
			sb.append(String.format(" %d | ", rank+1));
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

		sb.append("   +  -  -  -  -  -  -  -  -\n");
		sb.append("     ");
		String files = "abcdefgh";
		for (int file = 0; file < 8; file++) {
			sb.append(String.format(" %c ", files.toCharArray()[file]));
		}
		sb.append("\n");

		return sb.toString();
	}
	@Override
	public Board clone(){
		Check check = this.check;
		Point enPassant = this.enPassant == null ? null : this.enPassant.getLocation(); // getLocation == clone
		int score = this.score;
		List<Fen.CastlingMoves> castlingMoves = new ArrayList<>(this.castlingMoves);
		Piece.Color turn = this.turn;
		List<Piece> clonedPieces = this.getPieces().stream()
				.map(Piece::clone)
				.collect(Collectors.toList());

		return new Board(clonedPieces, check, enPassant, score, castlingMoves, turn);
	}
}

abstract class PieceMap {
	private Map<Point, Piece> pieceMap = new HashMap<>();

	public Piece getPiece(Point position) {
		return pieceMap.get(position);
	}

	public Piece yankPiece(Point position) {
		return pieceMap.remove(position);
	}

	public Collection<Piece> getPieces() {
		return pieceMap.values();
	}

	public Set<Point> getPositions() {
		return pieceMap.keySet();
	}

	public void putPieceSetPosition(Point position, Piece piece) {
		piece.setPosition(position);
		pieceMap.put(position, piece);
	}

	public void putPiece(Piece piece) {
		pieceMap.put(piece.getPosition(), piece);
	}
}
