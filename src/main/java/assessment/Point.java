package assessment;

import java.util.Objects;

public class Point implements Comparable<Point> {

    final int x;
    final int y;
    final double value;

    public Point(int x, int y, double value) {
        this.x = x;
        this.y = y;
        this.value = value;
    }

    @Override
    public int compareTo(Point that) {
        return -Double.compare(this.value, that.value);
    }

    @Override
    public String toString() {
//        return "(" + x + "," + y + ")";
        return "(" + x + "," + y + ":" + value + ")";
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Point point = (Point) o;
        return x == point.x && y == point.y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }
}
