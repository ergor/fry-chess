package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public class Rook extends Piece {

	public Rook(Color color, Point position) {
		super(Kind.ROOK, color, position);
	}

	@Override
	public List<Point> allPossibleLandingSquares(Board board) {

		List<Point> allowedMovements = getAllowedMoves(board, this);

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

	public static List<Point> getAllowedMoves(Board board, Piece piece){

		List<Point> allowedMovements = new ArrayList<>();
		for(int i = -7; i<=7; i++){
			if(i == 0 ){
				continue;
			}
			Point point = new Point(i, 0);
			if(!piece.isBlocked(board, point)){
				allowedMovements.add(point);
			}
			point = new Point(0, i);
			if(!piece.isBlocked(board, point)){
				allowedMovements.add(point);
			}
		}
		return allowedMovements;
	}
}
