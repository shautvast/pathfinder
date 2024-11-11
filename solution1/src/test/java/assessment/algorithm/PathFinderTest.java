package assessment.algorithm;

import org.junit.jupiter.api.Test;

public class PathFinderTest {

    @Test
    public void testBestPath20() {
        Grid grid = Grid.fromFile("grids/20.txt");
        Path path = new OptimalPathFinder().findOptimalPath(grid, 20, 10, 1000, 9, 9);
        System.out.println(path);
    }

    @Test
    public void testBestPath100() {
        Grid grid = Grid.fromFile("grids/100.txt");
        Path path = new OptimalPathFinder().findOptimalPath(grid, 100, 10, 100, 9, 9);
        System.out.println(path);
    }

    @Test
    public void testBestPath1000() {
        Grid grid = Grid.fromFile("grids/1000.txt");
        Path path = new OptimalPathFinder().findOptimalPath(grid, 1000, 18, 10000, 500, 500);
        System.out.println(path);
    }
}
