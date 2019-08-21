package st.netb.chess.lib;

public class FenException extends Exception {

    public FenException(String message) {
        super(message);
    }

    public FenException(String message, Throwable throwable) {
        super(message, throwable);
    }
}
