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
	 public List<Point> allPossibleMoves(Board board) {

		List<Point> allowedMovements = new ArrayList<>();
		for(int i = -7; i<=7; i++){
			Point point = new Point(i, i);
			if(!isBlocked(board, point)){
				allowedMovements.add(point);
			}
			point = new Point(i, -i);
			if(!isBlocked(board, point)){
				allowedMovements.add(point);
			}
		}

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
