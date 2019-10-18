package main.java.st.netb.chess.lib;

import java.util.ArrayList;

class MiniMax{

    public MiniMax(){

    }
    private int depth = 5;
    private Evaluator eval;
    private Generator generator;


    public Move getGoodMove(Board board){
        ArrayList<Board> boards = generator.getBoards(board);
        Board bestMove = boards.get(0);
        int valueBestBoard = (board.isWhiteMove()) ? Integer.MIN_VALUE : Integer.MAX_VALUE;

        for(Board b: boards) {
            int val = minimax(b, this.depth, b.isWhiteMove());
            if(isBest(val, valueBestBoard, b.isWiteMove())){
                valueBestBoard = val;
                bestMove = b;
            };
        }

        return generator.getMoveFromBoard(bestMove);
    }
    private boolean isBest(int value, int compareTo, boolean isWhite){
        return (isWhite) ? value > compareTo : value < compareTo;
    }


    private int minimax(Board board, int depth, boolean isMax){
        int value = 0;
        if(depth == 0 || generator.getBoards(board).length == 0){//or node is terminal
            return eval.eval(board);
        }
        if(isMax){
            value = Integer.MIN_VALUE;
            for(Board childBoard: generator.getBoards(board)){
                value = Math.max(minimax(childBoard, depth -1, false), value);
            }
        }
        else{
            value = Integer.MAX_VALUE;
            for(Board childBoard: generator.getBoards(board)){
                value = Math.min(minimax(childBoard, depth -1, true), value);
            }
        }

        return value;

    }
}
