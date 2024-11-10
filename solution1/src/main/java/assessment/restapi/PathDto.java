package assessment.restapi;

import assessment.algorithm.Path;

import java.util.List;
import java.util.stream.Collectors;

public class PathDto {
    private final List<PointDto> points;
    private final double value;

    public PathDto(Path p){
        this.points = p.points.stream().map(PointDto::new).collect(Collectors.toList());
        this.value = p.value();
    }

    public List<PointDto> getPoints() {
        return points;
    }

    public double getValue() {
        return value;
    }

}
