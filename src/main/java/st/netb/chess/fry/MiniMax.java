package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;

import java.util.List;

class MiniMax{

    private int depth = 5;


    public Board getGoodMove(Board board){
        List<Board> boards = BoardGenerator.getBoards(board);
        Board bestMove = boards.get(0);
        int valueBestBoard = (board.getTurn() == Piece.Color.WHITE) ? Integer.MIN_VALUE : Integer.MAX_VALUE;

        for(Board b: boards) {
            int val = minimax(b, this.depth, Integer.MIN_VALUE, Integer.MIN_VALUE, b.getTurn() == Piece.Color.WHITE);
            if(isBest(val, valueBestBoard, b.getTurn() == Piece.Color.WHITE)){
                valueBestBoard = val;
                bestMove = b;
            };
        }

        return bestMove;
    }
    private boolean isBest(int value, int compareTo, boolean isWhite){
        return (isWhite) ? value > compareTo : value < compareTo;
    }


    private int minimax(Board board, int depth, int alpha, int beta, boolean isMax){
        int value;
        if(depth == 0 || BoardGenerator.getBoards(board).size() == 0){//or node is terminal
            return Evaluator.evaluateBoard(board);
        }
        if(isMax){
            value = Integer.MIN_VALUE;
            for(Board childBoard: BoardGenerator.getBoards(board)){
                value = Math.max(minimax(childBoard, depth -1, alpha, beta, false), value);
                alpha = Math.max(value, alpha);
                if(alpha >= beta){
                    break;
                }
            }
        }
        else{
            value = Integer.MAX_VALUE;
            for(Board childBoard: BoardGenerator.getBoards(board)){
                value = Math.min(minimax(childBoard, depth -1, alpha, beta, true), value);
                alpha = Math.min(value, alpha);
                if(alpha >= beta){//Should this be opposite
                    break;
                }
            }
        }

        return value;

    }
}
