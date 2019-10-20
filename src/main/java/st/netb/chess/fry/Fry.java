
package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;
import st.netb.chess.lib.FenException;
import st.netb.chess.lib.San;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Scanner;
import java.util.function.Function;
import java.util.stream.Collectors;

public class Fry {

	public static void main(String[] args) {
		Scanner scanner = new Scanner(System.in);
		try {
			Board board = Board.getStartingBoard();
			while(true) {
				System.out.println(board.toString());
				Piece.Color turn = board.getTurn();
				if(turn == Piece.Color.WHITE) {
					System.out.println("White to Move: ");
				} else {
					System.out.println("Black to Move: ");
				}
				String inputMove = scanner.nextLine();
				if(inputMove.toLowerCase().equals("exit")) break;
				board = doMove(inputMove, board, turn);
			}
		} catch (FenException e) {
			e.printStackTrace();
		}

	}

	private static Board doMove(String input, Board initialBoard, Piece.Color turn) {
		// stuff;
		San san = new San(input);
		List<Point> startPositions = applySan(san, initialBoard);
		if (startPositions.size() == 0) {
			System.out.println("Illegal move");
		}
		else if (startPositions.size() > 1) {
			System.out.println("Ambigous move");
		}
		Point startPosition = startPositions.get(0);

		Map<Point, Piece> pieces = initialBoard.getPieces().keySet().stream()
				.filter(pos -> !pos.equals(startPositions))
				.collect(Collectors.toMap(Function.identity(), pos -> initialBoard.getPieces().get(pos).clone()));
		pieces.put(san.getEndPos(), initialBoard.getPiece(startPosition));

		Piece.Color nextPlayer = turn == Piece.Color.WHITE ? Piece.Color.BLACK : Piece.Color.WHITE;

		// TODO: initialize properly
		return new Board(pieces, Board.Check.NO_CHECK, null, initialBoard.getCastlingMoves(), nextPlayer);
	}


	private static List<Point> applySan(San sanMove, Board initialBoard) {

		//TODO: castling moves, checks/mates, etc

		Map<Point, Piece> candidatePieces = initialBoard.getPieces().entrySet().stream()
				.filter(e -> sanMove.getStartFile() < 0 || e.getKey().getX() == sanMove.getStartFile())
				.filter(e -> sanMove.getStartRank() < 0 || e.getKey().getY() == sanMove.getStartRank())
				.filter(e -> e.getValue().getKind() == mapFromLibKind(sanMove.getPiece()))
				.collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));

		List<Point> startPositions = new ArrayList<>();
		for (Piece piece : candidatePieces.values()) {
			if (piece.allPossibleMoves(initialBoard).stream()
					.anyMatch(pos -> pos.equals(sanMove.getEndPos()))) {
				startPositions.add(piece.getPosition());
			}
		}

		return startPositions;
	}


	private static Piece.Kind mapFromLibKind(st.netb.chess.lib.Piece.Kind libKind) {
		switch (libKind) {
			case PAWN:
				return Piece.Kind.PAWN;
			case BISHOP:
				return Piece.Kind.BISHOP;
			case KNIGHT:
				return Piece.Kind.KNIGHT;
			case ROOK:
				return Piece.Kind.ROOK;
			case QUEEN:
				return Piece.Kind.QUEEN;
			case KING:
				return Piece.Kind.KING;
			default:
				throw new IllegalStateException("enum was exhausted");
		}
	}
}
