package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public class King extends Piece {

	public King(Color color, Point position) {
		super(Kind.KING, color, position);
	}


	//castling not included
	@Override
	public List<Point> allPossibleLandingSquares(Board board) {

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
				.forEach(move -> {
					Point newPosition = getNewPositionAfterMovement(move);
					if (!isOutOfBoard(newPosition) && !isPositionFriendly(newPosition, board)){
						possiblePositions.add(newPosition);
					}
				});

		return possiblePositions;
	}
}
