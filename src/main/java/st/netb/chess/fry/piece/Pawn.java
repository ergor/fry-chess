package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public class Pawn extends Piece{

	public Pawn(Piece.Kind kind, Piece.Color color, Point position) {
		super(kind, color, position);
	}

	public Pawn(Piece.Color color, Point position) {
		super(Kind.PAWN, color, position);
	}

	// promoting of pawn not included
	@Override
	public List<Point> allPossibleMoves(Board board) {
		Point diagonalLeft = new Point(-1, 1);
		Point diagonalRight = new Point(1, 1);
		Point oneForward = new Point (0, 1);
		Point twoForward = new Point(0, 2);
		List<Point> allowedMovements = new ArrayList<>();
		allowedMovements.add(oneForward); //go forward
		if(isPositionEnemy(diagonalLeft, board))
			allowedMovements.add(diagonalLeft); //go diagonal left
		if(isPositionEnemy(diagonalRight, board))
			allowedMovements.add(diagonalRight); //go diagonal right
		if(getPosition().getY() == 1) // startPosition, it is possible to go two fields forward
			allowedMovements.add(twoForward);

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
