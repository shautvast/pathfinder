package assessment;

import java.util.ArrayList;
import java.util.List;
import java.util.PriorityQueue;

public class BestPathFinder {
    private final static double timeDistanceFactor = .2;
    private static Path max;
    private final static PriorityQueue<Path> paths = new PriorityQueue<>();
    private static volatile boolean running = true;


    /**
     * @param g het Grid (vierkant)
     * @param N grootte van het Grid
     * @param t totaal aantal discrete tijdstappen
     * @param T maximale tijdsduur
     * @param x startpositie X
     * @param y startpositie Y
     * @return het meest waardevole pad
     */
    public static Path findMaxValPath(Grid g, int N, int t, int T, int x, int y) {
        Path path = Path.newPath(g, x, y);
        paths.add(path);
        max = path;

        // start looping until T is reached or all paths have been considered
        while (running && !paths.isEmpty()) {
            // take current highest ranking path
            path = paths.peek();
            assert path != null;
            while (path.length() > t) {
                // dit pad heeft lengte t bereikt
                paths.poll(); // geen nieuwe paden op basis van deze, we pakken de vorige in waardering
                // heeft dit pad meer waarde dan de huidige max ?
                if (path.value() > max.value()) {
                    max = path;
                }
            }

            // find best new directions
            List<Point> newPointsFromHere = new ArrayList<>();
            if (y > 0) {
                newPointsFromHere.add(new Point(x, y - 1, getValueFromGrid(g, path, x, y - 1)));
                if (x < N - 1)
                    newPointsFromHere.add(new Point(x + 1, y - 1, getValueFromGrid(g, path, x + 1, y - 1)));
            }
            if (x > 0) {
                newPointsFromHere.add(new Point(x - 1, y, getValueFromGrid(g, path, x - 1, y)));
                if (y > 0)
                    newPointsFromHere.add(new Point(x - 1, y - 1, getValueFromGrid(g, path, x - 1, y - 1)));
            }
            if (x < N - 1) {
                newPointsFromHere.add(new Point(x + 1, y, getValueFromGrid(g, path, x + 1, y)));
                if (y < N - 1)
                    newPointsFromHere.add(new Point(x + 1, y + 1, getValueFromGrid(g, path, x + 1, y + 1)));
            }
            if (y < N - 1) {
                newPointsFromHere.add(new Point(x, y + 1, getValueFromGrid(g, path, x, y + 1)));
                if (x > 0)
                    newPointsFromHere.add(new Point(x - 1, y + 1, getValueFromGrid(g, path, x - 1, y + 1)));
            }

            // sort the new points in descending order of value
            newPointsFromHere.sort(Point::compareTo);
            double max = newPointsFromHere.getFirst().value;
            for (Point p : newPointsFromHere) {
                // create a new Path based on the current
                Path newPath = path.copy();

                // add the new point
                newPath.add(g, p);

                // stop adding points once their value decreases
                if (p.value < max) {
                    break;
                }
            }
        }
        return max;
    }

    /**
     * de waarde van een punt x,y wordt bepaald door de beginwaarde
     * tenzij we er al geweest zijn
     * dan telt de tijd sinds we er geweest zijn
     * = afstand in sindsdien afgelegd pad
     * de waarde is rechtevenredig met de afstand
     */
    private static double getValueFromGrid(Grid grid, Path path, int x, int y) {
        int gridValue = grid.get(x, y);
        if (path.hasPoint(x, y)) {
            // been there
            int distanceInPath = path.getDistanceInPath(x, y);
            double increment = gridValue / timeDistanceFactor;

            return Math.max((distanceInPath - 1) * increment, gridValue);
        } else {
            return gridValue;
        }
    }
}
