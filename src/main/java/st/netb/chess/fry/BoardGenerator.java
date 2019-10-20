package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;

import java.awt.*;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class BoardGenerator {

	public List<Board> getBoards(Board board){
		List<Board> boards = new ArrayList<>();
		Map<Point, Piece> pieces = board.getPieces();
		for(Point piece: pieces.keySet()){
			List<Point> moves = pieces.get(piece).allPossibleMoves(board);
			for(Point move: moves){
				Map<Point, Piece> mapNewBoard = new HashMap<>(pieces);
				mapNewBoard = copyMap(mapNewBoard);
				mapNewBoard.remove(piece);
				mapNewBoard.put(move, pieces.get(piece));

				Board newBoard = getBoardFromPoints(mapNewBoard);
				if(isValidBoard(newBoard))
				boards.add(newBoard);
			}
		}
		return boards;
	}

	public boolean isValidBoard(Board board){
		return true; //TODO implement
	}

	public Board getBoardFromPoints(Map<Point, Piece> pieces){
		return new Board(pieces, null, null, new ArrayList<>(), Piece.Color.BLACK);
	}

	public Map<Point, Piece> copyMap(Map<Point, Piece> p){
		return p.entrySet()
				.stream()
				.collect(Collectors.toMap((f -> new Point(f.getKey().x, f.getKey().y)), e -> e.getValue().getClone()));
	}

}
