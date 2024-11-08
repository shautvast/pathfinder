import assessment.Grid;
import assessment.Path;
import assessment.BestPathFinder;
import org.junit.jupiter.api.Test;

public class PathFinderTest {

    @Test
    public void testBestPath20() {
        Grid grid = Grid.fromFile("grids/20.txt");
        Path path = new BestPathFinder().findMaxValPath(grid, 20, 8, 1000, 9, 9);
        System.out.println(path);
    }

    @Test
    public void testBestPath100() {
        Grid grid = Grid.fromFile("grids/100.txt");
        Path path =  new BestPathFinder().findMaxValPath(grid, 100, 8, 10000, 50, 50);
        System.out.println(path);
    }

    @Test
    public void testBestPath1000() {
        Grid grid = Grid.fromFile("grids/1000.txt");
        Path path = new BestPathFinder().findMaxValPath(grid, 1000, 18, 100000, 500, 500);
        System.out.println(path);
    }
}
