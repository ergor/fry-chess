
package st.netb.chess.fry;

import st.netb.chess.lib.Board;

import java.util.Scanner;

public class Fry {
	public static Board board;


	public static void main(String[] args) {
		Scanner scanner = new Scanner(System. in);
		board = Board.getStartingBoard();
		while(true) {
			board.toString();
			Board.Turn turn = board.getTurn();
			if(turn == Board.Turn.WHITE) {
				System.out.println("White to Move: ");
			} else {
				System.out.println("Black to Move: ");
			}
			String inputMove = scanner.nextLine();
			if(inputMove.toLowerCase().equals("exit")) break;
			doMove(inputMove);
		}


	}

	private static void doMove(String input) {
		// stuff;
		board = board;
	}

}
