import java.util.function.Function;

public class LambdaExpression {
    public static void main(String[] args) {
        Function<Integer, String> toString = (Integer i) -> "Number: " + i;
        System.out.println(toString.apply(5));
    }
}
