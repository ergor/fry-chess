package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public class Knight extends Piece {

	public Knight(Kind kind, Color color, Point position) {
		super(kind, color, position);
	}

	public Knight(Color color, Point position) {
		super(Kind.KNIGHT, color, position);
	}

	@Override
	public List<Point> allPossibleLandingSquares(Board board) {
		List<Point> allowedMovements = new ArrayList<>();
		allowedMovements.add(new Point(1, 2));
		allowedMovements.add(new Point(1, -2));
		allowedMovements.add(new Point(-1, 2));
		allowedMovements.add(new Point(1, -2));
		allowedMovements.add(new Point(2, 1));
		allowedMovements.add(new Point(2, -1));
		allowedMovements.add(new Point(-2, 1));
		allowedMovements.add(new Point(-2, -1));

		List<Point> possiblePositions = new ArrayList<>();
		allowedMovements
				.stream()
				.forEach(pm -> {
					if (!isOutOfBoard(pm) && !isPositionFriendly(pm, board))
						possiblePositions.add(getNewPositionAfterMovement(pm));
				});
		return possiblePositions;
	}
}
