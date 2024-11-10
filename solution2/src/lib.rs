use grid::{Grid, Path, Point};
use std::collections::BTreeSet;
use std::time::{Duration, SystemTime};

pub mod grid;

pub fn find_optimal_path(grid: &Grid, t: usize, T: u128, x: u16, y: u16) {
    let mut paths_to_consider: BTreeSet<Path> = BTreeSet::new();
    let taken_paths: Vec<u64> = vec![];

    let path = Path::new(grid, x, y);

    let mut max: Path = path.clone(); // sorry

    paths_to_consider.insert(path);
    let t0 = SystemTime::now();
    let mut running = true;
    let mut discrete_elapsed = 0;

    while running {
        let N = grid.size();
        let mut current_path = paths_to_consider.pop_last().unwrap(); // assert Some

        if current_path.value > max.value {
            max = current_path.clone(); // sorry
        }
        while current_path.length() >= t {
            current_path = paths_to_consider.pop_last().unwrap(); // highest
            if current_path.value > max.value {
                max = current_path.clone(); // sorry
            }
        }

        let head = current_path.head();
        let x = head.x;
        let y = head.y;
        let mut new_directions = vec![];

        if y > 0 {
            new_directions.push(Point::new(
                x,
                y - 1,
                grid.get_value(x, y - 1, discrete_elapsed),
            ));
            if x < N - 1 {
                new_directions.push(Point::new(
                    x + 1,
                    y - 1,
                    grid.get_value(x + 1, y - 1, discrete_elapsed),
                ));
            }
        }

        if x > 0 {
            new_directions.push(Point::new(
                x - 1,
                y,
                grid.get_value(x - 1, y, discrete_elapsed),
            ));
            if y > 0 {
                new_directions.push(Point::new(
                    x - 1,
                    y - 1,
                    grid.get_value(x - 1, y - 1, discrete_elapsed),
                ));
            }
        }

        if x < N - 1 {
            new_directions.push(Point::new(
                x + 1,
                y,
                grid.get_value(x + 1, y, discrete_elapsed),
            ));
            if y < N - 1 {
                new_directions.push(Point::new(
                    x + 1,
                    y + 1,
                    grid.get_value(x + 1, y + 1, discrete_elapsed),
                ));
            }
        }

        if y < N - 1 {
            new_directions.push(Point::new(
                x,
                y + 1,
                grid.get_value(x, y + 1, discrete_elapsed),
            ));
            if x > 0 {
                new_directions.push(Point::new(
                    x - 1,
                    y + 1,
                    grid.get_value(x - 1, y + 1, discrete_elapsed),
                ));
            }
        }

        // continue?
        let t1 = SystemTime::now();
        if let Ok(elapsed) = t1.duration_since(t0) {
            if elapsed.as_millis() > T {
                running = false;
            }
        }
        discrete_elapsed += 1;
    }
}
