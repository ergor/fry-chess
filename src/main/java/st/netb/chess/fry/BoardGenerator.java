package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;
import st.netb.chess.lib.Fen;

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
		for(Piece piece: pieces.values()){
			for(Point point: pieces.keySet()){
				Map<Point, Piece> p = new HashMap<>(pieces);
				p.remove(point);
				p = copyMap(p);
				p.put(point, piece);

				boards.add(getBoardFromPoints(p));
			}
		}
		return boards;
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
