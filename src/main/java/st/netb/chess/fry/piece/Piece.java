package st.netb.chess.fry.piece;

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

    public static Color mapLibColor(st.netb.chess.lib.Piece.Color libColor) {
        switch (libColor) {
            case WHITE:
                return Color.WHITE;
            case BLACK:
                return Color.BLACK;
            default:
                throw new IllegalStateException("Somehow colors was exhausted");
        }
    }

    public static Piece fromLibPiece(st.netb.chess.lib.Piece libPiece) {
        Color color = mapLibColor(libPiece.getColor());
        Point position = libPiece.getPosition();
        switch (libPiece.getKind()) {
            case PAWN:
                return new Pawn(Kind.PAWN, color, position);
            case BISHOP:
                return new Bishop(Kind.PAWN, color, position);
            case KNIGHT:
                return new Knight(Kind.PAWN, color, position);
            case ROOK:
                return new Rook(Kind.PAWN, color, position);
            case QUEEN:
                return new Queen(Kind.PAWN, color, position);
            case KING:
                return new King(Kind.PAWN, color, position);
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
