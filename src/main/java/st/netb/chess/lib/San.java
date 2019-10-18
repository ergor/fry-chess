package st.netb.chess.lib;

import st.netb.chess.fry.Board;

import java.awt.*;
import java.util.HashMap;
import java.util.Map;

/**
 * endPos
 * piece
 * isCapture
 * isCheck
 * isCheckmate
 * isStalemate
 * isEnPassant
 * isCastle
 * isPromotion
 *
 */
public class San {

	private int startRank = -1;
	private int startFile = -1;
	private Point endPos;
	private boolean isCapture;
	private boolean isCheck;
	private boolean isCheckmate;
	private boolean isStalemate;
	private boolean isEnPassant;
	private boolean isCastling;
	private boolean isPromotion;
	private Piece.Kind piece;

	private CastlingMoves castling;
	private Piece.Kind promotedPiece;

	private static Map<Character, Piece.Kind> pieceKindMap = new HashMap<>();
	static {
		pieceKindMap.put('b', Piece.Kind.BISHOP);
		pieceKindMap.put('n', Piece.Kind.KNIGHT);
		pieceKindMap.put('r', Piece.Kind.ROOK);
		pieceKindMap.put('q', Piece.Kind.QUEEN);
		pieceKindMap.put('k', Piece.Kind.KING);
	}

	private static String fileNames = "abcdefgh";

	public int getStartRank() {
		return startRank;
	}

	public void setStartRank(int startRank) {
		this.startRank = startRank;
	}

	public int getStartFile() {
		return startFile;
	}

	public void setStartFile(int startFile) {
		this.startFile = startFile;
	}

	public Point getEndPos() {
		return endPos;
	}

	public void setEndPos(Point endPos) {
		this.endPos = endPos;
	}


	public enum CastlingMoves {
		KINGSIDE,
		QUEENSIDE
	}

	public San(String sanMove) {
		sanMove = sanMove.trim();
		//Castling
		if(sanMove.equals("0-0-0")) {
			isCastling = true;
			castling = CastlingMoves.QUEENSIDE;
			return;
		} else if (sanMove.equals("0-0")) {
			isCastling = true;
			castling = CastlingMoves.KINGSIDE;
			return;
		}

		int startRank = -1;
		int startFile = -1;
		int endRank = -1;
		int endFile = -1;

		char[] sanMoveCharArray = sanMove.toCharArray();
		for(int i = 0; i < sanMoveCharArray.length; i++) {
			char c = sanMoveCharArray[i];

			// Find type of piece.
			if(piece == null) {
				if(Character.isUpperCase(c)) {
					piece = pieceKindMap.get(Character.toLowerCase(c));
				} else {
					piece = Piece.Kind.PAWN;
					i--;
				}
				continue;
			}

			if(c == 'x') {
				isCapture = true;
				continue;
			}

			// Find file.
			if(Character.isLowerCase(c)) {
				if(startFile == -1) {
					startFile = fileNames.indexOf(c);
				} else {
					endFile = fileNames.indexOf(c);
				}
				continue;
			}

			// Find Rank if exists
			if(Character.isDigit(c)) {
				if(startRank == -1 && endFile == -1) {
					startRank = Integer.parseInt(String.valueOf(c)) - 1;
				} else {
					 endRank = Integer.parseInt(String.valueOf(c)) - 1;
				}
				continue;
			}

			if(Character.isUpperCase(c) && piece == Piece.Kind.PAWN) {
				this.isPromotion = true;
				this.promotedPiece = pieceKindMap.get(Character.toLowerCase(c));
				continue;
			}

			if(c == '+' && isCheck) {
				isCheckmate = true;
				continue;
			}

			if(c == '+') {
				isCheck = true;
			}

		}

		// If no endrank/endfile was found, then start pos found is end pos.
		if(endRank == -1) {
			this.endPos = new Point(startFile, startRank);
		} else {
			this.startRank = startRank;
			this.startFile = startFile;
			this.endPos = new Point(endFile, endRank);
		}

	}

	public boolean isCapture() {
		return isCapture;
	}

	public void setCapture(boolean capture) {
		isCapture = capture;
	}

	public boolean isCheck() {
		return isCheck;
	}

	public void setCheck(boolean check) {
		isCheck = check;
	}

	public boolean isCheckmate() {
		return isCheckmate;
	}

	public void setCheckmate(boolean checkmate) {
		isCheckmate = checkmate;
	}

	public boolean isStalemate() {
		return isStalemate;
	}

	public void setStalemate(boolean stalemate) {
		isStalemate = stalemate;
	}

	public boolean isEnPassant() {
		return isEnPassant;
	}

	public void setEnPassant(boolean enPassant) {
		isEnPassant = enPassant;
	}

	public boolean isCastling() {
		return isCastling;
	}

	public void setCastling(boolean castling) {
		isCastling = castling;
	}

	public boolean isPromotion() {
		return isPromotion;
	}

	public void setPromotion(boolean promotion) {
		isPromotion = promotion;
	}

	public Piece.Kind getPiece() {
		return piece;
	}

	public void setPiece(Piece.Kind piece) {
		this.piece = piece;
	}

	public CastlingMoves getCastling() {
		return castling;
	}

	public void setCastling(CastlingMoves castling) {
		this.castling = castling;
	}

	public Piece.Kind getPromotedPiece() {
		return promotedPiece;
	}

	public void setPromotedPiece(Piece.Kind promotedPiece) {
		this.promotedPiece = promotedPiece;
	}
}
