package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;

import java.awt.*;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class BoardGenerator {

	public static List<Board> getBoards(Board board){
		board = board.clone();
		List<Board> boards = new ArrayList<>();
		Map<Point, Piece> pieces = board.getPieces();
		for(Point piecePosition: pieces.keySet()){
			Piece piece = pieces.get(piecePosition);
			if(piece.getColor() != board.getTurn()){
				continue;
			}
			if((piece.getKind() == Piece.Kind.BISHOP ||piece.getKind() == Piece.Kind.QUEEN )){
				continue;
			}
			List<Point> moves = piece.allPossibleLandingSquares(board);
			for(Point move: moves){
				Map<Point, Piece> mapNewBoard = new HashMap<>(pieces);
				move = (Point) move.clone();
				mapNewBoard = copyMap(mapNewBoard);
				mapNewBoard.remove(piecePosition);
				piece.setPosition(move);
				mapNewBoard.put(move, piece);

				Board newBoard = getBoardFromPoints(mapNewBoard, board);
				if(isValidBoard(newBoard))
				boards.add(newBoard);
			}
		}
		return boards;
	}

	public static boolean isValidBoard(Board board){
		return true; //TODO implement
	}

	private static Board getBoardFromPoints(Map<Point, Piece> pieces, Board board){
		Piece.Color turn;
		if(board.getTurn() == Piece.Color.WHITE){
			turn = Piece.Color.BLACK;
		}else {
			turn = Piece.Color.WHITE;
		}

		return new Board(pieces, null, null, new ArrayList<>(), turn);
	}

	public static Map<Point, Piece> copyMap(Map<Point, Piece> p){
		return p.entrySet()
				.stream()
				.collect(Collectors.toMap((f -> new Point(f.getKey().x, f.getKey().y)), e -> e.getValue().clone()));
	}


}
