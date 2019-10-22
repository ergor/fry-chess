package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;

import java.util.List;

class MiniMax{

    private static int depth = 5;


     public static Board getGoodMove(Board board){
        List<Board> boards = BoardGenerator.getBoards(board);
        Board bestMove = boards.get(0);
        int scoreBestBoard = (board.getTurn() == Piece.Color.WHITE) ? Integer.MIN_VALUE : Integer.MAX_VALUE;

        for(Board b: boards) {
            int val = minimax(b, depth, Integer.MIN_VALUE, Integer.MIN_VALUE, b.getTurn() == Piece.Color.WHITE);
            if(isBest(val, scoreBestBoard, b.getTurn() == Piece.Color.WHITE)){
                scoreBestBoard = val;
                bestMove = b;
                bestMove.setScore(scoreBestBoard);
            };
        }
        return bestMove;
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
