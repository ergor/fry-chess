package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public class Rook extends Piece {

	public Rook(Kind kind, Color color, Point position) {
		super(kind, color, position);
	}

	public Rook(Color color, Point position) {
		super(Kind.ROOK, color, position);
	}

	@Override
	public List<Point> allPossibleLandingSquares(Board board) {

		List<Point> allowedMovements = getMovesInDirection(board, Direction.left);
		allowedMovements.addAll(getMovesInDirection(board, Direction.right));
		allowedMovements.addAll(getMovesInDirection(board, Direction.up));
		allowedMovements.addAll(getMovesInDirection(board, Direction.down));

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
