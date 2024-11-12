use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, LinkedList},
    hash::{Hash, Hasher},
};

const RECOVERY_FACTOR: f32 = 0.1;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Grid {
    data: Vec<Vec<u16>>,
    // keep track of every point that has been hit at some time
    hits: HashMap<(u16, u16), usize>, // TreeSet<usize> is integer times that point has been visited.
                                      // Must always be sorted and could probably be a single int
                                      // (keep track of last time hit)
}

impl Grid {
    #[allow(non_snake_case)]
    pub fn new(N: usize) -> Self {
        let grid20 = include_str!("grids/20.txt");
        let grid100 = include_str!("grids/100.txt");
        let grid1000 = include_str!("grids/1000.txt");

        let datafile = match N {
            20 => grid20,
            100 => grid100,
            1000 => grid1000,
            _ => panic!("only 20, 100 or 1000 are available"),
        };

        let mut data = vec![];
        for row in datafile.split('\n') {
            let mut datarow = vec![];
            for col in row.split(' ') {
                if let Ok(v) = col.parse::<u16>() {
                    datarow.push(v);
                }
            }
            data.push(datarow);
        }
        Grid {
            data,
            hits: HashMap::new(),
        }
    }

    pub fn get_initial_value(&self, x: u16, y: u16) -> f32 {
        *(self.data.get(y as usize).unwrap().get(x as usize).unwrap()) as f32
    }

    /// get the value of a point on the grid
    /// 1. initial value, given by the datafile
    /// 2. value possibly updated by a previous drone in the multiple drone scenario
    /// 3. value possibly updated by the drone itself, when it gets to a point that it already occupied
    pub fn get_value(&self, x: u16, y: u16, path: Option<&Path>) -> f32 {
        let pathlen = path.map_or(0, |p| p.length());
        let initial_value = self.get_initial_value(x, y);

        // 1.
        let mut value = initial_value;

        // 2.
        let hit_time = self.hits.get(&(x, y));
        if let Some(hit_time) = hit_time {
            if pathlen > *hit_time {
                // +1 because we are in the process of adding a new point to the path
                let elapsed_since_hit = (pathlen + 1 - hit_time) as f32;
                value = f32::min(
                    elapsed_since_hit * initial_value * RECOVERY_FACTOR,
                    initial_value,
                );
            }
        }

        // 3.
        if let Some(path) = path {
            let maybe_hit = path.points_lookup.get(&(x, y));
            if let Some(hit_time) = maybe_hit {
                let elapsed_since_hit = (path.points.len() - hit_time) as f32;
                value = f32::min(
                    elapsed_since_hit * initial_value * RECOVERY_FACTOR,
                    initial_value,
                );
            }
        }

        value
    }

    pub fn hit(&mut self, x: u16, y: u16, time: usize) {
        self.hits.insert((x, y), time);
    }

    pub fn size(&self) -> u16 {
        self.data.len() as u16
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pub points: LinkedList<Point>,
    pub points_lookup: HashMap<(u16, u16), usize>,
    pub value: f32,
}

impl Path {
    pub fn new(grid: &Grid, initial_x: u16, initial_y: u16) -> Self {
        let value = grid.get_value(initial_x, initial_y, None);

        let p = Point {
            x: initial_x,
            y: initial_y,
            value,
        };

        let mut points = LinkedList::new();
        points.push_front(p);
        let mut points_lookup = HashMap::new();
        points_lookup.insert((initial_x, initial_y), 0);

        Self {
            points,
            value,
            points_lookup,
        }
    }

    // length = age of the path
    pub fn length(&self) -> usize {
        self.points.len()
    }

    pub fn last(&self) -> &Point {
        self.points.front().unwrap() // assert Some
    }

    pub fn value(&self) -> f32 {
        self.points.iter().map(|p| p.value).sum()
    }

    pub fn add(&mut self, p: Point) {
        self.points.push_front(p);
        self.value = self.value();
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Path {}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value > other.value {
            true => Ordering::Greater,
            false => {
                if self.value() == other.value {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        if self.points.len() != other.points.len() {
            return false;
        }
        for p in self.points.iter() {
            if !other.points.contains(p) {
                return false;
            }
        }
        true
    }
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for p in self.points.iter() {
            p.x.hash(state);
            p.y.hash(state);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
    pub value: f32,
}

impl Point {
    pub fn new(x: u16, y: u16, value: f32) -> Self {
        Self { x, y, value }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[cfg(test)]
mod test {
    use super::Grid;

    #[test]
    pub fn test() {
        let grid = Grid::new(20);
        assert_eq!(grid.get_value(0, 0, None), 0.0);
        assert_eq!(grid.get_value(0, 1, None), 1.0);
    }
}

#[derive(Debug)]
pub struct PathsResult {
    pub paths: Vec<Path>,
    pub overall_score: f32,
}
