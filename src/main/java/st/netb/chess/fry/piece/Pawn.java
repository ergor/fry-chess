package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public class Pawn extends Piece{

	public Pawn(Piece.Color color, Point position) {
		super(Kind.PAWN, color, position);
	}

	// promoting of pawn not included
	@Override
	public List<Point> allPossibleLandingSquares(Board board) {
		List<Point> allowedMovements = new ArrayList<>();

		if(this.getColor() == Color.WHITE){
			Point diagonalLeft = new Point(-1, 1);
			Point diagonalRight = new Point(1, 1);
			Point oneForward = new Point (0, 1);
			Point twoForward = new Point(0, 2);
			allowedMovements.add(oneForward); //go forward
			if(isPositionEnemy(diagonalLeft, board))
				allowedMovements.add(diagonalLeft); //go diagonal left
			if(isPositionEnemy(diagonalRight, board))
				allowedMovements.add(diagonalRight); //go diagonal right
			if(getPosition().getY() == 1) // startPosition, it is possible to go two fields forward
				allowedMovements.add(twoForward);
		}else{
			Point diagonalLeft = new Point(-1, -1);
			Point diagonalRight = new Point(1, -1);
			Point oneForward = new Point (0, -1);
			Point twoForward = new Point(0, -2);
			if(!isPositionEnemy(getNewPositionAfterMovement(oneForward), board))
				allowedMovements.add(oneForward); //go forward
			if(isPositionEnemy(getNewPositionAfterMovement(diagonalLeft), board))
				allowedMovements.add(diagonalLeft); //go diagonal left
			if(isPositionEnemy(getNewPositionAfterMovement(diagonalRight), board))
				allowedMovements.add(diagonalRight); //go diagonal right
			if(getPosition().getY() == 6) // startPosition, it is possible to go two fields forward
				allowedMovements.add(twoForward);
		}

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
