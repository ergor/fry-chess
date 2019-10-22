package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Queen extends Piece {

	public Queen(Color color, Point position) {
		super(Kind.QUEEN, color, position);
	}

	@Override
	public List<Point> allPossibleLandingSquares(Board board) {

		List<Point> allowedMovements = getMovesInDirection(board, Direction.left);
		allowedMovements.addAll(getMovesInDirection(board, Direction.right));
		allowedMovements.addAll(getMovesInDirection(board, Direction.up));
		allowedMovements.addAll(getMovesInDirection(board, Direction.down));
		allowedMovements.addAll(getMovesInDirection(board, Direction.upLeft));
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
