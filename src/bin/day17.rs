use std::collections::{ HashMap, HashSet };
use std::fs;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug)]
struct Space {
    data: HashMap<Point, char>,
}

impl Space {
    fn new() -> Space {
        Space { data: HashMap::new() }
    }

    fn from_string_2d(string: String) -> Space {
        let mut space: Space = Space::new();
        for (y , line) in string.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                space.add_by_xyz(x as isize, y as isize, 0, ch);
            }
        }
        space
    }

    fn add_by_point(&mut self, point: Point, value: char) {
        self.data.insert(point, value);
    }

    fn add_by_xyz(&mut self, x: isize, y: isize, z: isize, value: char) {
        self.add_by_point(Point { x, y, z }, value);
    }

    fn has_point(&self, point: &Point) -> bool {
        self.data.contains_key(point)
    }

    fn point_value(&self, point: &Point) -> Option<&char> {
        self.data.get(point)
    }

    fn point_neighbors(&self, point: &Point) -> HashSet<Point> {
        let mut resp: HashSet<Point> = HashSet::new();
        let range: &'static [isize; 3] = &[-1, 0, 1];
        for &x in range {
            for &y in range {
                for &z in range {
                    let p: Point = Point {x, y, z};
                    if p != *point {
                        resp.insert(p);
                    }
                }
            }
        }
        resp
    }

    fn point_is(&self, point: &Point, value: &char) -> bool {
        if let Some(v) = self.point_value(point) {
            v == value
        } else {
            false
        }
    }

    fn get_points(&self) -> HashSet<&Point> {
        self.data.keys().collect()
    }
}

const ACTIVE: char = '#';
const INACTIVE: char = '.';

fn main() {
    // let input = fs::read_to_string("src/bin/input17.txt").unwrap();
    let input = String::from(r".#.
..#
###");
    let mut space: Space = Space::from_string_2d(input);

    permute(6, &mut space);
    let num_active: usize = space.get_points().iter().map(|p| space.point_is(p, &ACTIVE)).count();
    println!("num active: {}", num_active);
}

// If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
// If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
fn permute(iters: usize, space: &mut Space) {
    for _ in 0..iters {
        // immutable borrow
        let points: HashSet<&Point> = space.get_points();

        // immutable borrow used here
        for point in points {
            let num_adj_active: usize = space.point_neighbors(point).iter().map(|p| space.point_is(p, &ACTIVE)).count();

            // Evaluate point in space by itself
            match space.point_value(point) {
                Some(&ACTIVE) => {
                    if num_adj_active < 2 || num_adj_active > 3 {
                        space.add_by_point(*point, INACTIVE);
                    }
                }
                Some(&INACTIVE) => {
                    if num_adj_active == 3 {
                        space.add_by_point(*point, ACTIVE);
                    }
                }
                Some(x) => { panic!("point value is invalid: {:?} {}", point, x) }
                None => { panic!("point value was none") }
            }

            // also an immutable borrow
            for new_adj in space.point_neighbors(point) {
                if !space.has_point(&new_adj) {
                    // mutable borrow here =[
                    space.add_by_point(new_adj, INACTIVE);
                }
            }
        }
    }
}
