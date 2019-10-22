package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public class Knight extends Piece {

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
				.forEach(move -> {
					Point newPosition = getNewPositionAfterMovement(move);
					if (!isOutOfBoard(newPosition) && !isPositionFriendly(newPosition, board)){
						possiblePositions.add(newPosition);
					}
				});

		return possiblePositions;
	}
}
