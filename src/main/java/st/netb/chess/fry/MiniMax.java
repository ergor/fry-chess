package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;

import java.rmi.UnexpectedException;
import java.util.List;

class MiniMax{

    private static int depth = 5;


     public static Board getGoodMove(Board board){
        List<Board> boardsNextTurn = BoardGenerator.getBoards(board);

        boolean isWhite = board.getTurn() == Piece.Color.WHITE;
        boolean isNextWhite = board.getNextTurn() == Piece.Color.WHITE;

        for (Board b: boardsNextTurn) {
            int val = minimax(b, depth, Integer.MIN_VALUE, Integer.MAX_VALUE, isNextWhite);
            b.setScore(val);
        }

        return boardsNextTurn.stream()
                .reduce((res, elem) ->
                        isBest(elem.getScore(), res.getScore(), isWhite) ? elem : res)
                .orElseThrow(() -> new RuntimeException("a best board was not found"));
    }
    private static boolean isBest(int value, int compareTo, boolean isWhite){
        return (isWhite) ? value > compareTo : value < compareTo;
    }


    private static int minimax(Board board, int depth, int alpha, int beta, boolean isMax){
        int value;
        List<Board> boards = BoardGenerator.getBoards(board);
        if(depth == 0 || boards.size() == 0){//or node is terminal
            return Evaluator.evaluateBoard(board);
        }
        if(isMax){
            value = Integer.MIN_VALUE;
            for(Board childBoard: boards){
                int newValue = minimax(childBoard, depth -1, alpha, beta, false);
                value = Math.max(newValue, value);
                if(newValue >= beta){
                    return value;
                }
                alpha = Math.max(value, alpha);
            }
        }
        else{
            value = Integer.MAX_VALUE;
            for(Board childBoard: boards){
                int newValue = minimax(childBoard, depth -1, alpha, beta, true);
                value = Math.min(newValue, value);
                if(newValue >= alpha){//Should this be opposite
                    return value;
                }
                alpha = Math.min(value, alpha);
            }
        }

        return value;

    }
}
