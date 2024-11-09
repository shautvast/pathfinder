package assessment.algorithm;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.Objects;


public class Point implements Comparable<Point> {

    public final int x;
    public final int y;
    public double value;

    private Point(int x, int y, double initialValue) {
        this.x = x;
        this.y = y;
        this.value = initialValue;
    }

    public static Point create(Grid g, int x, int y) {
        return new Point(x, y, g.getInitialValue(x, y));
    }

    public static Point create(Grid g, Path p, int x, int y) {
        return new Point(x, y, g.getCurrentValue(p, x, y));
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
