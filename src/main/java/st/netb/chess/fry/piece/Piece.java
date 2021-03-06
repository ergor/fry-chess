package st.netb.chess.fry.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.ArrayList;
import java.util.List;

public abstract class Piece {

    private Kind kind;
    private Color color;
    private Point position;

    public Piece(Kind kind, Color color, Point position) {
        this.kind = kind;
        this.color = color;
        this.position = position;
    }

    public enum Kind {
        PAWN,
        BISHOP,
        KNIGHT,
        ROOK,
        QUEEN,
        KING
    }

    public enum Color {
        WHITE,
        BLACK
    }

    public static Color mapFromLibColor(st.netb.chess.lib.Piece.Color libColor) {
        switch (libColor) {
            case WHITE:
                return Color.WHITE;
            case BLACK:
                return Color.BLACK;
            default:
                throw new IllegalStateException("Somehow colors was exhausted");
        }
    }

    public static Piece mapFromLibPiece(st.netb.chess.lib.Piece libPiece) {
        Color color = mapFromLibColor(libPiece.getColor());
        Point position = libPiece.getPosition();
        switch (libPiece.getKind()) {
            case PAWN:
                return new Pawn(color, position);
            case BISHOP:
                return new Bishop(color, position);
            case KNIGHT:
                return new Knight(color, position);
            case ROOK:
                return new Rook(color, position);
            case QUEEN:
                return new Queen(color, position);
            case KING:
                return new King(color, position);
            default:
                throw new IllegalStateException("piece kind was exhausted");
        }
    }

    public Kind getKind() {
        return kind;
    }

    public Color getColor() {
        return color;
    }

    public Point getPosition() {
        return position;
    }

    public void setKind(Kind kind) {
        this.kind = kind;
    }

    public void setColor(Color color) {
        this.color = color;
    }

    public void setPosition(Point position) {
        this.position = position;
    }

    public abstract List<Point> allPossibleLandingSquares(Board board);

    public boolean isOutOfBoard(Point newPosition) {
        return (newPosition.getX() < 0 || newPosition.getX() > 7
                || newPosition.getY() < 0 || newPosition.getY() > 7);
    }

    public boolean isPositionFriendly(Point newPosition, Board board) {
        return board.getPiece(newPosition) != null && board.getPiece(newPosition).getColor().equals(color);
    }

    public boolean isPositionEnemy(Point newPosition, Board board) {
        return board.getPiece(newPosition) != null && !board.getPiece(newPosition).getColor().equals(color);
    }

    public boolean isPositionEmpty(Point newPosition, Board board) {
        return board.getPiece(newPosition) == null;
    }

    public Point getNewPositionAfterMovement(Point movement) {
        int newX = (int) (position.getX() + movement.getX());
        int newY = (int) ((int) position.getY() + movement.getY());
        return new Point(newX, newY);
    }

    public enum Direction{
        left(new Point(-1, 0)),
        right(new Point(1, 0)),
        up(new Point(0, 1)),
        down(new Point(0, -1)),
        upLeft(new Point(-1, 1)),
        upRight(new Point(1, 1)),
        downLeft(new Point(-1, -1)),
        downRight(new Point(1, -1));

        Point vector;

        Direction(Point direction) {
            this.vector = direction;
        }
    }

    public List<Point> getMovesInDirection(Board board, Direction direction){
       List<Point> allowedMovements = new ArrayList<>();

        for(int i = 1; i < 8; i++){
            Point point = new Point(i*direction.vector.x, i*direction.vector.y);
            Point newPosition = getNewPositionAfterMovement(point);

            if(isPositionFriendly(newPosition, board)){
              return allowedMovements;
           }else if(isPositionEnemy(newPosition, board)){
               allowedMovements.add(point);
               return allowedMovements;
           }
           allowedMovements.add(point);
       }
       return allowedMovements;
    }

    @Override
    public Piece clone() {
        Point newPoint = new Point(this.position.x, this.position.y);
        switch (this.kind) {
            case PAWN:
                return new Pawn(this.color, newPoint);
            case BISHOP:
                return new Bishop(this.color, newPoint);
            case KNIGHT:
                return new Knight(this.color, newPoint);
            case ROOK:
                return new Rook(this.color, newPoint);
            case QUEEN:
                return new Queen(this.color, newPoint);
            case KING:
                return new King(this.color, newPoint);
            default:
                throw new IllegalStateException("Kinds was exhausted");
        }
    }
}
