package st.netb.chess.lib.piece;

import st.netb.chess.fry.Board;

import java.awt.*;
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

    public abstract List<Point> allPossibleMoves(Board board);

    public boolean isOutOfBoard(Point movement) {
        Point newPosition = getNewPositionAfterMovement(movement);
        if (newPosition.getX() < 0 || newPosition.getY() > 7
                || newPosition.getY() < 0 || newPosition.getY() > 7)
            return false;
        return true;
    }

    public boolean isNewPositionOccupiedSameColor(Point movement, Board board) {
        Point newPosition = getNewPositionAfterMovement(movement);
        return board.getPiece(newPosition) != null && board.getPiece(newPosition).getColor().equals(color);
    }

    public Point getNewPositionAfterMovement(Point movement) {
        int newX = (int) (position.getX() + movement.getX());
        int newY = (int) ((int) position.getY() + movement.getY());
        return new Point(newX, newY);
    }
}
