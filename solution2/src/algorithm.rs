use crate::grid::{Grid, Path, PathsResult, Point};
use rand::Rng;
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::SystemTime;
use std::{
    sync::{Arc, Mutex},
    thread,
};

#[allow(non_snake_case)]
pub fn find_optimal_path_for_n_drones(
    grid: Grid,
    ndrones: usize,
    N: u16,
    t: usize,
    T: u128,
) -> PathsResult {
    let arc = Arc::new(Mutex::new(grid));

    let mut handles = vec![];

    for _ in 0..ndrones {
        let gridref = Arc::clone(&arc);

        let mut rng = rand::thread_rng();
        // start at random position
        let x: u16 = rng.gen_range(0..N);
        let y: u16 = rng.gen_range(0..N);
        // start new thread for a single drone
        let handle = thread::spawn(move || find_optimal_path(gridref, N, t, T, x, y));
        handles.push(handle);
    }

    let mut paths = vec![];
    for handle in handles {
        paths.push(handle.join().unwrap());
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
pub fn find_optimal_path(
    grid: Arc<Mutex<Grid>>,
    N: u16,
    t: usize,
    T: u128,
    x: u16,
    y: u16,
) -> Path {
    let mut paths_to_consider: Vec<Path> = Vec::new();
    let mut taken_paths: HashSet<u64> = HashSet::new();

    // starting point
    let path = Path::new(Arc::clone(&grid), x, y);

    // always current max
    let mut max: Path = path.clone(); // sorry

    // add the first path
    paths_to_consider.push(path);

    // keep track of time
    let t0 = SystemTime::now();
    let mut running = true;

    // this is time spent for the path, grid keeps the global timeframe
    // same as path length, so remove?
    let mut discrete_elapsed = 0;

    // will keep at most 8 new directions from current location
    let mut new_directions = vec![];

    let mut current_path;
    while running && !paths_to_consider.is_empty() {
        paths_to_consider.sort();
        current_path = paths_to_consider.last().unwrap().clone(); // assert = Some

        // evict paths that are of max len
        while current_path.length() >= t {
            _ = paths_to_consider.remove(paths_to_consider.len() - 1);
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
        {
            let arc = Arc::clone(&grid);
            let mut lock = arc.lock();
            let grid = lock.as_mut().unwrap();
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
                        grid.hit(point.x, point.y, discrete_elapsed);
                        paths_to_consider.push(new_path);
                        taken_paths.insert(hash);
                    }
                }
            }
            if !points_added {
                // dead end, evict
                let ended = paths_to_consider.remove(paths_to_consider.len() - 1);
                if ended.value > max.value {
                    max = ended;
                }
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
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_single_drone() {
        let grid = Grid::new(20);
        let opt = find_optimal_path(Arc::new(Mutex::new(grid.clone())), 100, 10, 1000, 9, 9);

        let mut all_points: HashSet<Point> = HashSet::new();
        for point in opt.points.iter() {
            if all_points.contains(point) {
                if all_points.contains(point) {
                    let iv = grid.get_initial_value(point.x, point.y);
                    assert!(point.value < iv);
                }
            }
            all_points.insert(point.clone());
        }
    }

    #[test]
    pub fn test_multiple_drones() {
        let grid = Grid::new(100);
        let paths_result = find_optimal_path_for_n_drones(grid.clone(), 4, 100, 10, 1000);

        let mut all_points: HashSet<Point> = HashSet::new();

        for path in paths_result.paths.iter() {
            for point in path.points.iter() {
                if all_points.contains(point) {
                    let iv = grid.get_initial_value(point.x, point.y);
                    assert!(point.value < iv);
                }
                all_points.insert(point.clone());
            }
        }
    }
}
