package assessment.algorithm;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.Objects;

/**
 * Represents a path in a grid, consisting of a sequence of points.
 */
public class Path implements Comparable<Path> {
    //both these collections contain the current points in the path

    // points is meant to traverse the points
    public final ArrayList<Point> points = new ArrayList<>();

    // the purpose of trodden is to quickly determine if a point is part of this path
    private final HashSet<Integer> trodden = new HashSet<>();

    private Path() {
    }

    // meh row/col vs x/y
    public static Path newPath(Grid g, Point start) {
        Path p = new Path();
        p.add(g, start);
        return p;
    }

    public void add(Grid g, Point point) {
        points.add(point);
        trodden.add(point.y * g.getWidth() + point.x);
    }

    public double value() {
        return points.stream().mapToDouble(point -> point.value).sum();
    }

    // compare descending, highest value first
    @Override
    public int compareTo(Path o) {
        return -Double.compare(this.value(), o.value());
    }

    public int length() {
        return points.size();
    }

    public Point getHead() {
        return points.get(points.size() - 1);
    }

    boolean hasPoint(Grid g, int x, int y) {
        return trodden.contains(y * g.getWidth() + x);
    }

    int getDistanceInPath(int x, int y) {
        if (points.isEmpty()) {
            return -1;
        }
        for (int i = points.size() - 1; i >= 0; i--) {
            Point p = points.get(i);
            if (p.x == x && p.y == y) {
                return points.size() - i;
            }
        }
        return -1;
    }

    public Path copy() {
        Path p = new Path();
        p.points.addAll(points);
        p.trodden.addAll(trodden);
        return p;
    }

    @Override
    public String toString() {
        return "P:" + points + ":" + value();
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Path path = (Path) o;
        return Objects.equals(points, path.points);
    }

    @Override
    public int hashCode() {
        return Objects.hashCode(points);
    }
}


