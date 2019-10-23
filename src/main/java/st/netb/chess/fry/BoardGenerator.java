package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;

import java.awt.*;
import java.util.*;
import java.util.List;
import java.util.stream.Collectors;

public class BoardGenerator {

	public static List<Board> getBoards(Board board){

		List<Board> boards = new ArrayList<>();
		Collection<Piece> pieces = board.getPieces();

		for(Piece piece : pieces){

			if(piece.getColor() != board.getTurn()){
				continue;
			}

			List<Point> landingSquares = piece.allPossibleLandingSquares(board);
			for(Point landingSquare: landingSquares){
				Board newBoard = board.clone();
				Piece movedPiece = newBoard.yankPiece(piece.getPosition());

				newBoard.putPieceSetPosition(landingSquare, movedPiece);
				newBoard.setTurn(board.getNextTurn());

				if(isValidBoard(newBoard))
				boards.add(newBoard);
			}
		}
		return boards;
	}

	public static boolean isValidBoard(Board board){
		return true; //TODO implement
	}
}
