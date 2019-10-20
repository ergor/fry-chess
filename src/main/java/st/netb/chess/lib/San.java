package st.netb.chess.lib;

import java.awt.*;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.function.Function;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

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
	private boolean isEnPassant;
	private Piece.Kind piece;
	private CheckKind checkKind;
	private CastlingMove castling;
	private Annotation annotation;
	private Piece.Kind promotedPiece;

	private static Map<Character, Piece.Kind> pieceKindMap = new HashMap<>();
	static {
		pieceKindMap.put('B', Piece.Kind.BISHOP);
		pieceKindMap.put('N', Piece.Kind.KNIGHT);
		pieceKindMap.put('R', Piece.Kind.ROOK);
		pieceKindMap.put('Q', Piece.Kind.QUEEN);
		pieceKindMap.put('K', Piece.Kind.KING);
	}

	private static String fileNames = "abcdefgh";

	public enum CastlingMove {
		KINGSIDE("O-O"),
		QUEENSIDE("O-O-O");

		private String str;
		CastlingMove(String str) {
			this.str = str;
		}

		private static Map<String, CastlingMove> reverseMap = Arrays.stream(CastlingMove.values())
				.collect(Collectors.toMap(val -> val.str, Function.identity()));

		@Override
		public String toString() {
			return str;
		}

		public static CastlingMove fromString(String s) {
			return Optional.ofNullable(reverseMap.get(s)).orElseThrow(IllegalArgumentException::new);
		}
	}

	public enum CheckKind {
		CHECK("+"),
		MATE("#");

		private String str;
		CheckKind(String str) {
			this.str = str;
		}

		private static Map<String, CheckKind> reverseMap = Arrays.stream(CheckKind.values())
				.collect(Collectors.toMap(val -> val.str, Function.identity()));

		@Override
		public String toString() {
			return str;
		}

		public static CheckKind fromString(String s) {
			return Optional.ofNullable(reverseMap.get(s)).orElseThrow(IllegalArgumentException::new);
		}
	}

	public enum Annotation {
		BLUNDER("??"),
		MISTAKE("?"),
		INTERESTING("?!"),
		GOOD("!"),
		BRILLIANT("!!");

		private String str;
		Annotation(String str) {
			this.str = str;
		}

		private static Map<String, Annotation> reverseMap = Arrays.stream(Annotation.values())
				.collect(Collectors.toMap(val -> val.str, Function.identity()));

		@Override
		public String toString() {
			return str;
		}

		public static Annotation fromString(String s) {
			return Optional.ofNullable(reverseMap.get(s)).orElseThrow(IllegalArgumentException::new);
		}
	}

	private static String buildExpr(String... parts) {
		return "^" + String.join("", parts) + "(\\+|\\#)?(\\?\\?|\\?|\\?!|!|!!)?$";
	}

	private static boolean isNotEmpty(String s) {
		return s != null && s.length() > 0;
	}

	private static int getFile(String file) {
		return file.charAt(0) - 'a';
	}

	private static int getRank(String rank) {
		return rank.charAt(0) - '1';
	}

	private static final Pattern castlingPattern = Pattern.compile(buildExpr("(O-O|O-O-O)"));
	private static final Pattern pawnMovementPattern = Pattern.compile(buildExpr("(([a-h]){1}([1-8])?x)?([a-h])([1-8])(=?([KQBNR]))?(?:e\\.p\\.)?"));
	private static final Pattern pieceMovementPattern = Pattern.compile(buildExpr("([KQBNR])([a-h])?([0-9])?(x)?([a-h])([1-8])"));

	public static Optional<San> parse(String move) {
		San san = new San();
		Matcher matcher;

		matcher = castlingPattern.matcher(move);
		if (matcher.matches()) {
			String castlingMove = matcher.group(1);
			String checkKind = matcher.group(2);
			String annotation = matcher.group(3);

			san.castling = CastlingMove.fromString(castlingMove);
			san.checkKind = isNotEmpty(checkKind) ? CheckKind.fromString(checkKind) : null;
			san.annotation = isNotEmpty(annotation) ? Annotation.fromString(annotation) : null;
			return Optional.of(san);
		}
		matcher = pawnMovementPattern.matcher(move);
		if(matcher.matches()){
			String startFile = matcher.group(2);
			String startRank = matcher.group(3);
			String capture = matcher.group(1);
			String endFile = matcher.group(4);
			String endRank = matcher.group(5);
			String promotionPiece= matcher.group(7);
			String enPassant = matcher.group(8);
			String checkKind = matcher.group(9);
			//String annotation = matcher.group(10);

			san.piece = Piece.Kind.PAWN;
			san.startFile = isNotEmpty(startFile) ? getFile(startFile) : -1;
			san.startRank = isNotEmpty(startRank) ? getRank(startRank) : -1;
			san.isCapture = isNotEmpty(capture);
			san.endPos = new Point(getFile(endFile), getRank(endRank));
			san.promotedPiece = isNotEmpty(promotionPiece) ? pieceKindMap.get(promotionPiece.charAt(0)) : null;
			san.isEnPassant = isNotEmpty(enPassant);
			san.checkKind = isNotEmpty(checkKind) ? CheckKind.fromString(checkKind) : null;
			//san.annotation = isNotEmpty(annotation) ? Annotation.fromString(annotation) : null;
			return Optional.of(san);
		}

		matcher = pieceMovementPattern.matcher(move);
		if (matcher.matches()) {
			String pieceKind = matcher.group(1);
			String startFile = matcher.group(2);
			String startRank = matcher.group(3);
			String capture = matcher.group(4);
			String endFile = matcher.group(5);
			String endRank = matcher.group(6);
			String checkKind = matcher.group(7);
			String annotation = matcher.group(8);

			san.piece = pieceKindMap.get(pieceKind.charAt(0));
			san.startFile = isNotEmpty(startFile) ? getFile(startFile) : -1;
			san.startRank = isNotEmpty(startRank) ? getRank(startRank) : -1;
			san.isCapture = isNotEmpty(capture);
			san.endPos = new Point(getFile(endFile), getRank(endRank));
			san.checkKind = isNotEmpty(checkKind) ? CheckKind.fromString(checkKind) : null;
			san.annotation = isNotEmpty(annotation) ? Annotation.fromString(annotation) : null;
			return Optional.of(san);
		}

		return Optional.empty();
	}

	public int getStartRank() {
		return startRank;
	}

	public int getStartFile() {
		return startFile;
	}

	public Point getEndPos() {
		return endPos;
	}

	public boolean isCapture() {
		return isCapture;
	}

	public boolean isCheck() {
		return checkKind == CheckKind.CHECK;
	}

	public boolean isCheckmate() {
		return checkKind == CheckKind.MATE;
	}

	public boolean isEnPassant() {
		return isEnPassant;
	}

	public boolean isPromotion() {
		return promotedPiece != null;
	}

	public Piece.Kind getPiece() {
		return piece;
	}

	public Optional<CastlingMove> getCastling() {
		return Optional.ofNullable(castling);
	}

	public Optional<CheckKind> getCheckKind() {
		return Optional.ofNullable(checkKind);
	}

	public Optional<Annotation> getAnnotation() {
		return Optional.ofNullable(annotation);
	}

	public Optional<Piece.Kind> promotedPiece() {
		return Optional.ofNullable(promotedPiece);
	}

	public Piece.Kind getPromotedPiece() {
		return promotedPiece;
	}
}
