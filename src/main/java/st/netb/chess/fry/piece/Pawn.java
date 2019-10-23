package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class Pawn extends Piece{

	public Pawn(Piece.Color color, Point position) {
		super(Kind.PAWN, color, position);
	}

	private static final Point WHITE_ONE_FORWARD = new Point (0, 1);
	private static final Point WHITE_TWO_FORWARD = new Point(0, 2);
	private static final Point WHITE_DIAGONAL_LEFT = new Point(-1, 1);
	private static final Point WHITE_DIAGONAL_RIGHT = new Point(1, 1);

	private static final Point BLACK_ONE_FORWARD = new Point (0, -1);
	private static final Point BLACK_TWO_FORWARD = new Point(0, -2);
	private static final Point BLACK_DIAGONAL_LEFT = new Point(-1, -1);
	private static final Point BLACK_DIAGONAL_RIGHT = new Point(1, -1);

	// promoting of pawn not included
	@Override
	public List<Point> allPossibleLandingSquares(Board board) {

		List<Point> allowedLandingSquares;

		if (this.getColor() == Color.WHITE) {
			allowedLandingSquares = pawnLandingSquares(board, 1,
					WHITE_ONE_FORWARD, WHITE_TWO_FORWARD, WHITE_DIAGONAL_LEFT, WHITE_DIAGONAL_RIGHT);
		}
		else {
			allowedLandingSquares = pawnLandingSquares(board, 6,
					BLACK_ONE_FORWARD, BLACK_TWO_FORWARD, BLACK_DIAGONAL_LEFT, BLACK_DIAGONAL_RIGHT);
		}

		return allowedLandingSquares.stream()
				.filter(pos -> !isOutOfBoard(pos))
				.collect(Collectors.toList());
	}

	private List<Point> pawnLandingSquares(Board board,
										   int startRank,
										   Point forwardVector,
										   Point twoForwardVector,
										   Point diagonalLeftVector,
										   Point diagonalRightVector) {

		List<Point> allowedMovements = new ArrayList<>();

		Point forward = getNewPositionAfterMovement(forwardVector);
		Point twoForward = getNewPositionAfterMovement(twoForwardVector);
		Point diagonalLeft = getNewPositionAfterMovement(diagonalLeftVector);
		Point diagonalRight = getNewPositionAfterMovement(diagonalRightVector);

		if (isPositionEmpty(forward, board)) {
			allowedMovements.add(forward);
		}
		if (getPosition().y == startRank
				&& isPositionEmpty(twoForward, board)) {
			allowedMovements.add(twoForward);
		}
		if (isPositionEnemy(diagonalLeft, board)) {
			allowedMovements.add(diagonalLeft);
		}
		if (isPositionEnemy(diagonalRight, board)) {
			allowedMovements.add(diagonalRight);
		}

		return allowedMovements;
	}
}
