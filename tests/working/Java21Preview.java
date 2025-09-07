// Using a preview feature from Java 21 that might not be enabled by default
public class Java21Preview {
    record Point(int x, int y) {}
    static void printObject(Object obj) {
        switch (obj) {
            case Point(int x, _) -> System.out.println("Point with x=" + x);
            default -> System.out.println("Unknown object");
        }
    }
}
