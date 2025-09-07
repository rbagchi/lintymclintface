import java.util.List;
import java.util.ArrayList;

public class GenericsWildcard {
    public static void printNumbers(List<? extends Number> list) {
        for (Number n : list) {
            System.out.print(n + " ");
        }
        System.out.println();
    }
}
