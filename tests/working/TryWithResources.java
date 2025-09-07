import java.io.*;

public class TryWithResources {
    public void readFile(String path) throws IOException {
        try (BufferedReader br = new BufferedReader(new FileReader(path))) {
            System.out.println(br.readLine());
        } catch (Exception e) {
            //TODO: handle exception
        }
    }
}
