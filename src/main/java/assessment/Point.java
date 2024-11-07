package assessment;

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
}
