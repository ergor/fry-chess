
package st.netb.chess.fry;

import java.util.Scanner;

public class Fry {

	public static void main(String[] args) {
		Scanner scanner = new Scanner(System. in);
		while(true) {
			//Board board = new Board();
			System.out.println("X to Move: ");
			String inputMove = scanner.nextLine();
			if(inputMove.toLowerCase().equals("exit")) break;
			doMove(inputMove);
		}


	}

	private static void doMove(String input) {

	}

}
