package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public class Bishop extends Piece {

	public Bishop(Kind kind, Color color, Point position) {
		super(kind, color, position);
	}

	public Bishop(Color color, Point position) {
		super(Kind.BISHOP, color, position);
	}

	@Override
	 public List<Point> allPossibleLandingSquares(Board board) {

		List<Point> allowedMovements = getMovesInDirection(board, Direction.upLeft);
		allowedMovements.addAll(getMovesInDirection(board, Direction.upRight));
		allowedMovements.addAll(getMovesInDirection(board, Direction.downLeft));
		allowedMovements.addAll(getMovesInDirection(board, Direction.downRight));

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
