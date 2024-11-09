package assessment.algorithm;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.stream.Collectors;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class CircleTest {

    @Test
    void test_return_to_previous_is_not_worth_it() {
        Grid grid = Grid.fromFile("grids/20.txt");
        Path path = Path.newPath(grid, Point.create(grid, 0, 0)); // 0.0
        path.add(grid, Point.create(grid, path, 0, 1)); // 1.0
        path.add(grid, Point.create(grid, path, 0, 2)); // 3.0
        path.add(grid, Point.create(grid, path, 0, 1)); // 3.0
        Assertions.assertEquals(3.0, path.value());
    }

    @Test
    void test_return_to_previous_is_worth_something() {
        Grid grid = Grid.fromFile("grids/20.txt");
        Path path = Path.newPath(grid, Point.create(grid, 0, 0)); // 0.0
        path.add(grid, Point.create(grid, path, 0, 1)); // 1.0
        path.add(grid, Point.create(grid, path, 0, 2)); // 3.0
        path.add(grid, Point.create(grid, path, 1, 1)); // 3.0
        path.add(grid, Point.create(grid, path, 0, 1)); // 3.2 distance 1, factor .2
        Assertions.assertEquals(3.2, path.value());
    }
}

