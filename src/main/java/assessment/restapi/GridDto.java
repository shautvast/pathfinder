package assessment.restapi;

import assessment.algorithm.Grid;

import java.util.List;

public class GridDto {
    private List<List<Integer>> grid;
    private int size;

    public GridDto(Grid grid) {
        this.grid = grid.grid();
        this.size = grid.getWidth();
    }

    public List<List<Integer>> getGrid() {
        return grid;
    }

    public int getSize() {
        return size;
    }
}
