package st.netb.chess.lib;

import java.awt.Point;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

/**
 * FEN rank definition:
 *  - Leftmost rank in string is rank 8, ie top of board.
 *  - Ranks separated by '/'
 */
public class Fen {

    private List<Piece> pieces;
    private Piece.Color activeColor;
    private List<CastlingMoves> castlingAvailability;
    private Point enPassant;
    private int halfmoveClock;
    private int fullmoveClock;


    private static Map<Character, Piece.Kind> pieceKindMap = new HashMap<>();
    static {
        pieceKindMap.put('p', Piece.Kind.PAWN);
        pieceKindMap.put('b', Piece.Kind.BISHOP);
        pieceKindMap.put('n', Piece.Kind.KNIGHT);
        pieceKindMap.put('r', Piece.Kind.ROOK);
        pieceKindMap.put('q', Piece.Kind.QUEEN);
        pieceKindMap.put('k', Piece.Kind.KING);
    }

    private static Map<String, Piece.Color> colorMap = new HashMap<>();
    static {
        colorMap.put("w", Piece.Color.WHITE);
        colorMap.put("b", Piece.Color.BLACK);
    }

    public enum CastlingMoves {
        KINGSIDE_WHITE,
        KINGSIDE_BLACK,
        QUEENSIDE_WHITE,
        QUEENSIDE_BLACK
    }

    /**
     */
    public Fen(String fenString) throws FenException {

        Pattern pattern = Pattern.compile(
                "^([rnbqkp1-8]{1,8})\\/([rnbqkp1-8]{1,8})\\/([rnbqkp1-8]{1,8})\\/([rnbqkp1-8]{1,8})\\/([rnbqkp1-8]{1,8})\\/([rnbqkp1-8]{1,8})\\/([rnbqkp1-8]{1,8})\\/([rnbqkp1-8]{1,8}) ([wb]) ([kq]{1,4}|-) ([a-h][1-8]|-) (\\d) (\\d)$",
                Pattern.CASE_INSENSITIVE);
        Matcher matcher = pattern.matcher(fenString);

        if (matcher.matches()) {

        }
        else {

        }
        List<String> parts = Arrays.stream(fenString.split(" "))
                .map(String::trim)
                .collect(Collectors.toList());

        try {
            pieces = parsePieces(parts.get(0));
            activeColor = parseActiveColor(parts.get(1));
            castlingAvailability = parseCastlingAvailability(parts.get(2));
        }
        catch (Exception e) {
            throw new FenException("failed to parse fen", e);
        }
    }

    private List<CastlingMoves> parseCastlingAvailability(String castlingString) {
        return null;
    }

    private Piece.Color parseActiveColor(String colorString) throws RuntimeException {
        return colorMap.computeIfAbsent(
                colorString,
                key -> {
                    throw new RuntimeException("illegal active color (expected \'w\' or \'b\')");
                });
    }


    private List<Piece> parsePieces(String input) throws RuntimeException {
        List<String> ranks = Arrays.asList(input.split("/"));

        if (ranks.size() != 8) {
            throw new RuntimeException("Wrong number of ranks found (expected 8)");
        }

        return IntStream.range(0, ranks.size())
                .mapToObj(i -> parseRank(i, ranks.get(i)))
                .flatMap(Collection::stream)
                .collect(Collectors.toList());
    }

    private List<Piece> parseRank(int rank, String rankString) throws RuntimeException {

        List<Piece> pieces = new ArrayList<>();
        char[] chars = rankString.toCharArray();

        int file;
        for (file = 0; file < chars.length; file++) {

            char pieceChar = chars[file];

            if (Character.isDigit(pieceChar)) {
                int skip = Integer.parseInt(String.valueOf(pieceChar)) - 1;
                file += skip;
                continue;
            }

            Piece.Color color = Character.isUpperCase(pieceChar)
                    ? Piece.Color.WHITE
                    : Piece.Color.BLACK;

            Piece.Kind kind = pieceKindMap.computeIfAbsent(
                    pieceChar,
                    key -> {
                        throw new RuntimeException("illegal character");
                    });

            pieces.add(new Piece(kind, color, new Point(file, rank)));
        }

        if (file > 7) {
            throw new RuntimeException("more than 8 files were processed");
        }

        return pieces;
    }
}
