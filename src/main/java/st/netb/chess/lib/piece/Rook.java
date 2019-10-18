package st.netb.chess.lib.piece;

import st.netb.chess.lib.Board;

import java.awt.*;
import java.util.List;

public class Rook extends Piece {

	public Rook(Kind kind, Color color, Point position) {
		super(kind, color, position);
	}

	public Rook(Color color, Point position) {
		super(Kind.ROOK, color, position);
	}

	@Override
	public List<Point> allPossibleMoves(Board board) {
		return null;
	}
}
