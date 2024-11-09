package assessment.algorithm;

import java.util.*;

/**
 * Finds most valuable path given distance/time covered (t) and elapsed compute time (T)
 * for a drone moving through a square grid of size N that has cells that have varying value
 * <p/>
 * outstanding questions:
 * * is a cell of (initial) value 0 equally valuable as a cell just occupied?
 * -> probably, what is the chance that you find a 'treasure' behind it, that you would have not found via an alternative path?
 */
public class OptimalPathFinder {

    // paths to be considered
    private final PriorityQueue<Path> paths = new PriorityQueue<>();

    private final Set<Path> takenPaths = new HashSet<>();

    /**
     * @param g het Grid (vierkant)
     * @param N grootte van het Grid
     * @param t totaal aantal discrete tijdstappen
     * @param T maximale tijdsduur
     * @param x startpositie X
     * @param y startpositie Y
     * @return het meest waardevolle pad
     */
    public Path findOptimalPath(Grid g, int N, int t, long T, int x, int y) {
        Path path = Path.newPath(g, Point.create(g, x, y));
        paths.add(path);
        // overall best path
        Path max = path;

        long t0 = System.currentTimeMillis();
        long t1 = t0 + T;

        // start looping until max duration T is reached or all paths have been evaluated
        while (System.currentTimeMillis() <= t1 && !paths.isEmpty()) {

            // take current highest ranking path
            path = paths.peek();
            assert path != null;
            while (path.length() >= t) {
                // dit pad heeft lengte t bereikt, we kunnen niet verder
                paths.poll();
                // meer waarde dan de huidige max ?
                if (path.value() > max.value()) {
                    max = path;
                }

                // pak de volgende
                path = paths.peek();
            }

//            System.out.println("paths:" + paths.size());
//            System.out.println("CUR: " + path);
            Point currentPos = path.getHead();
            x = currentPos.x;
            y = currentPos.y;

            // find best new directions
            List<Point> newDirections = new ArrayList<>();
            if (y > 0) {
                newDirections.add(Point.create(g, path, x, y - 1));
                if (x < N - 1) {
                    newDirections.add(Point.create(g, path, x + 1, y - 1));
                }
            }
            if (x > 0) {
                newDirections.add(Point.create(g, path, x - 1, y));
                if (y > 0) {
                    newDirections.add(Point.create(g, path, x - 1, y - 1));
                }
            }
            if (x < N - 1) {
                newDirections.add(Point.create(g, path, x + 1, y));
                if (y < N - 1) {
                    newDirections.add(Point.create(g, path, x + 1, y + 1));
                }
            }
            if (y < N - 1) {
                newDirections.add(Point.create(g, path, x, y + 1));
                if (x > 0)
                    newDirections.add(Point.create(g, path, x - 1, y + 1));
            }

            if (!newDirections.isEmpty()) {
                boolean pointsAdded = false;
                for (Point p : newDirections) {
                    // is it worthwile going there?
                    if (p.value > 0) { // use (higher) cutoff point?
                        // create a new Path based on the current
                        Path newPath = path.copy();

                        // add the new point
                        newPath.add(g, p);
                        if (!takenPaths.contains(newPath)) {
                            paths.add(newPath);
                            takenPaths.add(newPath);
                            pointsAdded = true;
                        }
                    }
                }
                if (!pointsAdded) {
                    // dead end, evict
                    Path ended = paths.poll();
                    if (ended != null && ended.value() > max.value()) {
                        max = ended;
                    }
                }
            }
        }
        return max;
    }
}
