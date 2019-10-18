
package st.netb.chess.fry;

import st.netb.chess.lib.FenException;
import st.netb.chess.lib.Piece;

import java.util.Scanner;

public class Fry {
	public static Board board;


	public static void main(String[] args) {
		Scanner scanner = new Scanner(System. in);
		try {
			board = Board.getStartingBoard();
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
				doMove(inputMove);
			}
		} catch (FenException e) {
			e.printStackTrace();
		}

	}

	private static void doMove(String input) {
		// stuff;
		board = board;
	}

}
