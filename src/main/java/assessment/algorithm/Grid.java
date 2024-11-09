package assessment.algorithm;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;

public record Grid(List<List<Integer>> grid) {
    private static final double timeValueFactor = .2;

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

    public int getInitialValue(int x, int y){
        return grid.get(y).get(x);
    }

    //assumes square
    public int getWidth(){
        return grid.size();
    }

    /**
     * de waarde van een punt x,y wordt bepaald door de beginwaarde, tenzij we er al geweest zijn
     * dan telt de tijd sinds we er geweest zijn = afstand in sindsdien afgelegd pad
     * de waarde is rechtevenredig met de afstand
     */
    public double getCurrentValue(Path path, int x, int y) {
        int gridValue = getInitialValue(x, y);
        if (path.hasPoint(this, x, y)) {
            // been there
            int distanceInPath = path.getDistanceInPath(x, y) - 1;
            double increment = gridValue * timeValueFactor;

            return Math.min((distanceInPath - 1) * increment, gridValue);
        } else {
            return gridValue;
        }
    }
}
