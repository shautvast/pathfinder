package assessment;

import java.util.ArrayList;
import java.util.HashSet;

public class Path implements Comparable<Path> {
    // beide bevatten de al afgelegde punten

    // points is bedoeld om door de punten te lopen
    private final ArrayList<Point> points = new ArrayList<>();

    // trodden is bedoeld om zo snel mogelijk vast te stellen of we al op het punt geweest zijn
    private final HashSet<Integer> trodden = new HashSet<>();

    private Path() {
    }

    static Path newPath(Grid g, int x, int y) {
        Path p = new Path();
        newPath(g, x, y);
        return p;
    }

    public void add(Grid g, Point point) {
        points.add(point);
        trodden.add(point.y * g.getWidth() + point.x);
    }

    public Double value() {
        return points.stream().mapToDouble(point -> point.value).sum();
    }

    @Override
    public int compareTo(Path o) {
        return this.value().compareTo(o.value());
    }

    public int length() {
        return points.size();
    }

    boolean hasPoint(int x, int y) {
        return trodden.contains(x * y);
    }

    int getDistanceInPath(int x, int y) {
        for (int i = points.size() - 1; i >= 0; i++) {
            if (points.get(i).x == x && points.get(i).y == y) {
                return points.size() - i;
            }
        }
        return -1;
    }

    public Path copy(){
        Path p = new Path();
        p.points.addAll(points);
        p.trodden.addAll(trodden);
        return p;
    }
}


