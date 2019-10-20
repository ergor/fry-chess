package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Queen extends Piece {

	public Queen(Kind kind, Color color, Point position) {
		super(kind, color, position);
	}

	public Queen(Color color, Point position) {
		super(Kind.QUEEN, color, position);
	}

	@Override
	public List<Point> allPossibleLandingSquares(Board board) {
		List<Point> allowedRookMovements = Rook.getAllowedMoves(board, this);
		List<Point> allowedBishopMovements = Bishop.getAllowedMoves(board, this);
		List<Point> allowedMovements = Stream.concat(allowedRookMovements.stream(), allowedBishopMovements.stream())
				.collect(Collectors.toList());

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
