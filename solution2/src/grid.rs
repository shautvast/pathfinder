use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    hash::{Hash, Hasher},
};

#[derive(Debug)]
pub struct Grid {
    data: Vec<Vec<u16>>,
    // keep track of every point that has been hit at some time
    hits: HashMap<(u16, u16), BTreeSet<usize>>, // TreeSet<usize> is integer times that point has been visited.
                                                // Must always be sorted and could probably be a single int
                                                // (keep track of last time hit)
}

impl Grid {
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
        for row in datafile.split("\n") {
            let mut datarow = vec![];
            for col in row.split(" ") {
                datarow.push(u16::from_str_radix(col, 10).unwrap());
            }
            data.push(datarow);
        }
        Grid {
            data,
            hits: HashMap::new(),
        }
    }

    pub fn get_value(&self, x: u16, y: u16, time: usize) -> f32 {
        let hit = self.hits.get(&(x, y));

        let initial_value = *(self.data.get(y as usize).unwrap().get(x as usize).unwrap()) as f32;

        if let Some(hit_times) = hit {
            for t in hit_times.iter().rev() {
                if time > *t {
                    let elapsed_since_hit = (time - *t) as f32;

                    return f32::min(
                        elapsed_since_hit * initial_value as f32 * 0.1,
                        initial_value,
                    );
                }
            }
            0.0
        } else {
            initial_value
        }
    }

    pub fn size(&self) -> u16 {
        self.data.len() as u16
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Point>,
    pub value: f32,
}

impl Path {
    pub fn new(grid: &Grid, initial_x: u16, initial_y: u16) -> Self {
        let mut points = vec![];
        let value = grid.get_value(initial_x, initial_y, 0);

        let p = Point {
            x: initial_x,
            y: initial_y,
            value,
        };

        points.push(p);
        Self { points, value }
    }

    pub fn length(&self) -> usize {
        self.points.len()
    }

    pub fn head(&self) -> &Point {
        self.points.get(self.points.len() - 1).unwrap() // assert Some
    }

    pub fn value(&self) -> f32 {
        self.points.iter().map(|p| p.value).sum()
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        match self.value > other.value {
            true => Some(Ordering::Greater),
            false => {
                if self.value == other.value {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Less)
                }
            }
        }
    }
}

impl Eq for Path {}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        self.value == other.value
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
        Self { x, y, value: 0.0 }
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
