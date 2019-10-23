
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
					String inputMove = scanner.nextLine();
					if(inputMove.toLowerCase().equals("exit")) break;
					try{
						board = doMove(inputMove, board, turn);
					}catch (IllegalArgumentException e){
						System.out.println("Illegal argument");
					}
				} else {
					System.out.println("Black to Move: ");
					board = MiniMax.getGoodMove(board);
				}
			}
		} catch (FenException e) {
			e.printStackTrace();
		}

	}

	private static Board doMove(String input, Board initialBoard, Piece.Color turn) {

		San san = San.parse(input).orElseThrow(IllegalArgumentException::new);
		List<Point> startPositions = applySan(san, initialBoard);
		if (startPositions.size() == 0) {
			System.out.println("Illegal move");
			return initialBoard;
		}
		else if (startPositions.size() > 1) {
			System.out.println("Ambigous move");
			return initialBoard;
		}

		Board newBoard = initialBoard.clone();
		Point startPosition = startPositions.get(0);
		Piece movedPiece = newBoard.yankPiece(startPosition);

		// TODO: initialize properly
		newBoard.putPieceSetPosition(san.getEndPos(), movedPiece);
		newBoard.setTurn(initialBoard.getNextTurn());

		return newBoard;
	}


	private static List<Point> applySan(San sanMove, Board initialBoard) {

		//TODO: castling moves, checks/mates, etc

		List<Piece> candidatePieces = initialBoard.getPieces().stream()
				.filter(piece -> sanMove.getStartFile() < 0 || piece.getPosition().x == sanMove.getStartFile())
				.filter(piece -> sanMove.getStartRank() < 0 || piece.getPosition().y == sanMove.getStartRank())
				.filter(piece -> piece.getKind() == mapFromLibKind(sanMove.getPiece()))
				.filter(piece -> piece.getColor() == initialBoard.getTurn())
				.collect(Collectors.toList());

		List<Point> startPositions = new ArrayList<>();
		for (Piece piece : candidatePieces) {
			if (piece.allPossibleLandingSquares(initialBoard).stream()
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
