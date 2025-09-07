// Using records and pattern matching for switch (Java 21)
public class Java21Features {

    record Point(int x, int y) {}

    static void printObject(Object obj) {
        switch (obj) {
            case Point(int x, int y) -> System.out.println("Point with x=" + x + " and y=" + y);
            case String s -> System.out.println("String: " + s);
            default -> System.out.println("Unknown object");
        }
    }

    public static void main(String[] args) {
        printObject(new Point(1, 2));
        printObject("Hello from Java 21");
    }
}
