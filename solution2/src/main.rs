use rand::Rng;
use solution2::{find_optimal_path, grid::Grid};
use std::{
    sync::{Arc, Mutex},
    thread,
};

/// this app calculates paths for 4 drones, concurrently, with a shared grid
fn main() {
    find_optimal_path_for_n_drones(4, 100, 10, 1000);
}

#[allow(non_snake_case)]
pub fn find_optimal_path_for_n_drones(ndrones: usize, N: u16, t: usize, T: u128) {
    let grid = Grid::new(100);
    let arc = Arc::new(Mutex::new(grid));

    let mut handles = vec![];

    for _ in 0..ndrones {
        let gridref = Arc::clone(&arc);

        let mut rng = rand::thread_rng();
        // start at random position
        let x: u16 = rng.gen_range(0..100);
        let y: u16 = rng.gen_range(0..100);
        // start new thread for a single drone
        let handle = thread::spawn(move || find_optimal_path(gridref, N, t, T, x, y));
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("{result:?}");
    }
}
