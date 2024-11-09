package assessment.algorithm;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.stream.Collectors;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class GridReadTest {

    @Test
    void testGridReader() {
        Grid grid = Grid.fromFile("grids/20.txt");
        List<Integer> row10 = grid.grid().get(9);
        String row10line = row10.stream().map(i -> "" + i).collect(Collectors.joining(" "));
        assertEquals("1 1 1 2 0 0 1 1 2 2 0 2 2 2 2 1 1 2 0 2", row10line);
    }
}
