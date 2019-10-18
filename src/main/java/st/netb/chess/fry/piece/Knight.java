package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.List;

public class Knight extends Piece {

	public Knight(Kind kind, Color color, Point position) {
		super(kind, color, position);
	}

	public Knight(Color color, Point position) {
		super(Kind.KNIGHT, color, position);
	}

	@Override
	public List<Point> allPossibleMoves(Board board) {
		return null;
	}
}
