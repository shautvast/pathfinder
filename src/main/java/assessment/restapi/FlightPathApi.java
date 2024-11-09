package assessment.restapi;

import assessment.algorithm.OptimalPathFinder;
import assessment.algorithm.Grid;
import assessment.algorithm.Path;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class FlightPathApi {

    @GetMapping(path = "/api/path/{gridname}/{pathlength}/{maxMillis}/{x}/{y}")
    public PathDto getOptimalPath(@PathVariable String gridname,
                                  @PathVariable int pathlength,
                                  @PathVariable long maxMillis,
                                  @PathVariable int x,
                                  @PathVariable int y) {
        Grid grid = Grid.fromFile("grids/" + gridname + ".txt");
        Path p = new OptimalPathFinder().findOptimalPath(grid, grid.getWidth(), pathlength, maxMillis, x, y);
        return new PathDto(p);
    }

    @GetMapping(path = "/api/grid/{gridname}")
    public GridDto getGrid(@PathVariable String gridname) {
        Grid grid = Grid.fromFile("grids/" + gridname + ".txt");
        return new GridDto(grid);
    }
}
