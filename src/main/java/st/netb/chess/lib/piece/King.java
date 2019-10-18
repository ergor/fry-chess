package st.netb.chess.lib.piece;

import st.netb.chess.lib.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public class King extends Piece {

	public King(Kind kind, Color color, Point position) {
		super(kind, color, position);
	}

	public King(Color color, Point position) {
		super(Kind.KING, color, position);
	}

	@Override
	public List<Point> allPossibleMoves(Board board) {

		List<Point> allowedMovements = new ArrayList<>();
		allowedMovements.add(new Point(1,0));
		allowedMovements.add(new Point(0,1));
		allowedMovements.add(new Point(-1,0));
		allowedMovements.add(new Point(0,-1));
		allowedMovements.add(new Point(1,1));
		allowedMovements.add(new Point(-1,1));
		allowedMovements.add(new Point(-1,-1));
		allowedMovements.add(new Point(1,-1));
		List<Point> possiblePositions = new ArrayList<>();
		allowedMovements
				.stream()
				.forEach(pm -> {
					if (!isOutOfBoard(pm) && !isNewPositionOccupiedSameColor(pm, board))
						possiblePositions.add(getNewPositionAfterMovement(pm));
				});
		return allowedMovements;
	}
}
