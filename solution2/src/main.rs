use solution2::{algorithm::find_optimal_path_for_n_drones, grid::Grid};

/// this app calculates paths for 4 drones, concurrently, with a shared grid
fn main() {
    let result = find_optimal_path_for_n_drones(Grid::new(100), 4, 100, 15, 2000);
    for (i, path) in result.paths.iter().enumerate() {
        println!("path {}: score {}", i, path.value())
    }
    println!("overall: {}", result.overall_score);
}
