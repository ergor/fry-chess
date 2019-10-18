package st.netb.chess.lib.piece;

import st.netb.chess.lib.Board;

import java.awt.*;
import java.util.List;

public class Queen extends Piece {

	public Queen(Kind kind, Color color, Point position) {
		super(kind, color, position);
	}

	public Queen(Color color, Point position) {
		super(Kind.QUEEN, color, position);
	}

	@Override
	public List<Point> allPossibleMoves(Board board) {
		return null;
	}
}
