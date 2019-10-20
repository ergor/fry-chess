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
	public List<Point> allPossibleMoves(Board board) {

		List<Point> allowedMovements = new ArrayList<>();
		for(int i = -7; i<=7; i++){
			Point point = new Point(i, 0);
			if(!isBlocked(board, point)){
				allowedMovements.add(point);
			}
			point = new Point(0, i);
			if(!isBlocked(board, point)){
				allowedMovements.add(point);
			}

			allowedMovements.add(new Point(0, i));
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
