import assessment.Grid;
import assessment.Path;
import assessment.BestPathFinder;
import org.junit.jupiter.api.Test;

public class PathFinderTest {

    @Test
    public void testBestPath() {
        Grid grid = Grid.fromFile("grids/20.txt");
        Path path = BestPathFinder.findMaxValPath(grid, 20, 10, 1000, 9, 9);
        System.out.println(path);
    }
}
