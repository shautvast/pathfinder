package assessment.restapi;

import assessment.algorithm.Point;

public class PointDto {
    private final int x;
    private final int y;

    public PointDto(Point p) {
        this.x = p.x;
        this.y = p.y;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

}
