package st.netb.chess.lib;

import java.awt.*;

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

    @Override
    public String toString() {
        return "" + getColor() + "|" + getKind() + " " + "(" + getPosition().x + ", " + getPosition().y + ")";
    }
}
