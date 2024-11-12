use crate::grid::{Grid, Path, PathsResult, Point};
use rand::Rng;
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::SystemTime;

#[allow(non_snake_case)]
pub fn find_optimal_path_for_n_drones(
    grid: Grid,
    ndrones: usize,
    N: u16,
    t: usize,
    T: u128,
) -> PathsResult {
    // multiple drones are calculated one after another
    // grid hits must be updated for the next drone after the optimal for the previous has been calculated

    let mut paths = vec![];
    let mut grid = grid.clone();
    for _ in 0..ndrones {
        let mut current_grid = grid.clone();
        let mut rng = rand::thread_rng();
        // start at random position
        let x: u16 = rng.gen_range(0..N);
        let y: u16 = rng.gen_range(0..N);
        // divide max algorithm time by number of drones
        let time_per_drone = T / ndrones as u128;
        let optimal = find_optimal_path(&mut current_grid, N, t, time_per_drone, x, y);

        // update the grid to the lastes state
        for (index, p) in optimal.points.iter().enumerate() {
            grid.hit(p.x, p.y, index);
        }
        paths.push(optimal);
    }

    let overall_score = paths.iter().map(|p| p.value).sum();

    PathsResult {
        paths,
        overall_score,
    }
}

/// finds the optimal path for a drone in a grid of points(x,y) that each has a fixed initial value
/// observing a point (hit) resets the value to 0, after wich it gradually increases with time in a fixed rate
/// N size of square grid (rows and cols)
/// t max length of a flight path
/// T max duration of the algorithm
/// x,y drone start position in the grid
#[allow(non_snake_case)]
pub fn find_optimal_path(grid: &mut Grid, N: u16, t: usize, T: u128, x: u16, y: u16) -> Path {
    let mut paths_to_consider: Vec<Path> = Vec::new();
    let mut taken_paths: HashSet<u64> = HashSet::new();

    // starting point
    let path = Path::new(grid, x, y);

    // always current max
    let mut max: Path = path.clone(); // sorry

    // add the first path
    paths_to_consider.push(path);

    // keep track of time
    let t0 = SystemTime::now();
    let mut running = true;

    // will keep at most 8 new directions from current location
    let mut new_directions = vec![];

    let mut current_path;
    while running && !paths_to_consider.is_empty() {
        // would have liked a list like datastructure that is guaranteed to be sorted
        // BTreeSet would be nice, but equals/hash/cmp calls would be unneeded overhead
        paths_to_consider.sort(); // but this is also overhead compared to BTreeSet
        current_path = paths_to_consider.last().unwrap().clone(); // assert = Some

        // evict paths that are of max len
        while current_path.length() >= t {
            _ = paths_to_consider.pop(); // discards element that = current_path
            if current_path.value() > max.value() {
                max = current_path.clone(); // sorry
            }
            current_path = paths_to_consider.last().unwrap().clone();
        }

        let head = current_path.last();
        let x = head.x;
        let y = head.y;

        // create a list of directions to take
        new_directions.clear();
        if y > 0 {
            new_directions.push(Point::new(
                x,
                y - 1,
                grid.get_value(x, y - 1, Some(&current_path)),
            ));
            if x < N - 1 {
                new_directions.push(Point::new(
                    x + 1,
                    y - 1,
                    grid.get_value(x + 1, y - 1, Some(&current_path)),
                ));
            }
        }

        if x > 0 {
            new_directions.push(Point::new(
                x - 1,
                y,
                grid.get_value(x - 1, y, Some(&current_path)),
            ));
            if y > 0 {
                new_directions.push(Point::new(
                    x - 1,
                    y - 1,
                    grid.get_value(x - 1, y - 1, Some(&current_path)),
                ));
            }
        }

        if x < N - 1 {
            new_directions.push(Point::new(
                x + 1,
                y,
                grid.get_value(x + 1, y, Some(&current_path)),
            ));
            if y < N - 1 {
                new_directions.push(Point::new(
                    x + 1,
                    y + 1,
                    grid.get_value(x + 1, y + 1, Some(&current_path)),
                ));
            }
        }

        if y < N - 1 {
            new_directions.push(Point::new(
                x,
                y + 1,
                grid.get_value(x, y + 1, Some(&current_path)),
            ));
            if x > 0 {
                new_directions.push(Point::new(
                    x - 1,
                    y + 1,
                    grid.get_value(x - 1, y + 1, Some(&current_path)),
                ));
            }
        }

        let mut points_added = false;
        for point in new_directions.iter() {
            if point.value > 0.0 {
                let mut new_path = current_path.clone();
                new_path.add(point.clone());

                let mut s = DefaultHasher::new();
                new_path.hash(&mut s);
                let hash = s.finish();

                if !taken_paths.contains(&hash) {
                    points_added = true;
                    grid.hit(point.x, point.y, new_path.length());
                    paths_to_consider.push(new_path);
                    taken_paths.insert(hash);
                }
            }
        }
        if !points_added {
            // dead end, evict
            let ended = paths_to_consider.pop().unwrap();
            if ended.value > max.value {
                max = ended;
            }
        }

        // continue?
        let t1 = SystemTime::now();
        if let Ok(elapsed) = t1.duration_since(t0) {
            if elapsed.as_millis() > T {
                running = false;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_single_drone() {
        let mut grid = Grid::new(100);
        // values for x and y are chosen so that a loop must occur
        let opt = find_optimal_path(&mut grid, 20, 20, 1000, 9, 9);

        let mut all_points: HashSet<Point> = HashSet::new();
        let mut loop_in_path = false;
        for point in opt.points.iter() {
            if all_points.contains(point) {
                // we have a path that crosses itself
                // path value should be less than sum of individual initial values of point
                loop_in_path = true
            }
            all_points.insert(point.clone());
        }
        if loop_in_path {
            println!("check"); //verify that this occurs
            let max_sum: f32 = opt
                .points
                .iter()
                .map(|p| grid.get_initial_value(p.x, p.y))
                .sum();
            println!("max sum {:?}, opt path {:?}", max_sum, opt);
            // verify that the grid value was updated because of the hit
            assert!(max_sum > opt.value());
        }
    }
}
