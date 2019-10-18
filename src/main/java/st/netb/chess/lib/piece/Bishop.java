package st.netb.chess.lib.piece;

import st.netb.chess.lib.Board;

import java.awt.*;
import java.util.List;

public class Bishop extends Piece {

	public Bishop(Kind kind, Color color, Point position) {
		super(kind, color, position);
	}

	public Bishop(Color color, Point position) {
		super(Kind.BISHOP, color, position);
	}

	@Override
	public List<Point> allPossibleMoves(Board board) {
		return null;
	}
}
