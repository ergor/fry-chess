package st.netb.chess.fry;

import st.netb.chess.fry.piece.Piece;

import java.awt.*;
import java.util.Iterator;
import java.util.NoSuchElementException;

public class BoardIterator implements Iterator<Board> {

    private Board basisBoard;
    private Piece currentPiece;
    private Iterator<Point> landingSquareIterator;
    private Iterator<Point> nextLandingSquareIterator;

    private int pieceIndex = 0;
    private Piece[] pieces;

    public BoardIterator(Board basisBoard) {
        this.basisBoard = basisBoard;
        this.pieces = basisBoard.getPieces().toArray(new Piece[0]);

        while (landingSquareIterator == null || !landingSquareIterator.hasNext()) {
            if (pieceIndex < pieces.length) {
                currentPiece = pieces[pieceIndex++];
                landingSquareIterator = currentPiece.allPossibleLandingSquares(basisBoard).iterator();
            }
        }
    }

    /**
     * Returns {@code true} if the iteration has more elements.
     * (In other words, returns {@code true} if {@link #next} would
     * return an element rather than throwing an exception.)
     *
     * @return {@code true} if the iteration has more elements
     */
    @Override
    public boolean hasNext() {
        if (landingSquareIterator.hasNext()) {
            return true;
        } else {
            for (int i = pieceIndex+1; i < pieces.length; i++) {
                nextLandingSquareIterator = pieces[i].allPossibleLandingSquares(basisBoard).iterator();
                if (nextLandingSquareIterator.hasNext()) {
                    return true;
                }
            }
            return false;
        }
    }

    /**
     * Returns the next element in the iteration.
     *
     * @return the next element in the iteration
     * @throws NoSuchElementException if the iteration has no more elements
     */
    @Override
    public Board next() {

        while (!landingSquareIterator.hasNext()) {
            if (nextLandingSquareIterator != null) {
                landingSquareIterator = nextLandingSquareIterator;
                nextLandingSquareIterator = null;
                break;
            }

            if (pieceIndex < pieces.length) {
                currentPiece = pieces[pieceIndex++];
                landingSquareIterator = currentPiece.allPossibleLandingSquares(basisBoard).iterator();
            } else {
                throw new NoSuchElementException("Generator: ran out of pieces");
            }
        }

        Board generated = basisBoard.clone();
        Piece movingPiece = generated.yankPiece(currentPiece.getPosition());
        generated.putPieceSetPosition(landingSquareIterator.next(), movingPiece);

        return generated;
    }
}
