package st.netb.chess.lib.piece;

import st.netb.chess.lib.Board;

import java.awt.*;
import java.util.List;

public class Pawn extends Piece{

	public Pawn(Piece.Kind kind, Piece.Color color, Point position) {
		super(kind, color, position);
	}

	public Pawn(Piece.Color color, Point position) {
		super(Kind.PAWN, color, position);
	}

	@Override
	public List<Point> allPossibleMoves(Board board) {
		return null;
	}
}
