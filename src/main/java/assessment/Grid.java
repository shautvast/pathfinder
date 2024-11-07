package assessment;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;

public record Grid(List<List<Integer>> grid) {

    public static Grid fromFile(String resource)  {
        try {
            BufferedReader reader = new BufferedReader(new InputStreamReader(Grid.class.getClassLoader().getResourceAsStream(resource)));
            String line;
            List<List<Integer>> rows = new ArrayList<>();
            while ((line=reader.readLine())!=null){
                String[] values = line.split(" ");
                List<Integer> row = new ArrayList<>(values.length);
                for (int i = 0; i < values.length; i++) {
                    row.add(Integer.parseInt(values[i]));
                }
                rows.add(row);
            }
            return new Grid(rows);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public int get(int r, int c){
        return grid.get(r).get(c);
    }

    //assumes square
    public int getWidth(){
        return grid.size();
    }
}
