package assessment;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.Objects;

public class Path implements Comparable<Path> {
    // beide bevatten de al afgelegde punten

    // points is bedoeld om door de punten te lopen
    private final ArrayList<Point> points = new ArrayList<>();

    // trodden is bedoeld om zo snel mogelijk vast te stellen of we al op het punt geweest zijn
    private final HashSet<Integer> trodden = new HashSet<>();

    private Path() {
    }

    // meh row/col vs x/y
    static Path newPath(Grid g, int x, int y) {
        Path p = new Path();
        p.add(g, new Point(x, y, g.get(y, x)));
        return p;
    }

    public void add(Grid g, Point point) {
        points.add(point);
        trodden.add(point.y * g.getWidth() + point.x);
    }

    public Double value() {
        return points.stream().mapToDouble(point -> point.value).sum();
    }

    // compare descending, highest value first
    @Override
    public int compareTo(Path o) {
        return -this.value().compareTo(o.value());
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


