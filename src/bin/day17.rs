use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}, {}, {}}}", self.x, self.y, self.z, self.w)
    }
}

#[derive(Debug, Clone)]
struct Space {
    data: HashMap<Point, char>,
}

impl Space {
    fn new() -> Space {
        Space {
            data: HashMap::new(),
        }
    }

    fn from_string_2d(string: String) -> Space {
        let mut space: Space = Space::new();
        for (y, line) in string.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                space.add_by_xyzw(x as isize, y as isize, 0, 0, ch);
            }
        }
        space
    }

    fn add_by_point(&mut self, point: Point, value: char) {
        self.data.insert(point, value);
    }

    fn add_by_xyzw(&mut self, x: isize, y: isize, z: isize, w: isize, value: char) {
        self.add_by_point(Point { x, y, z, w }, value);
    }

    fn has_point(&self, point: &Point) -> bool {
        self.data.contains_key(point)
    }

    fn point_value(&self, point: &Point) -> Option<&char> {
        self.data.get(point)
    }

    fn point_is(&self, point: &Point, value: &char) -> bool {
        if let Some(v) = self.point_value(point) {
            v == value
        } else {
            false
        }
    }

    fn point_neighbors(&self, point: &Point) -> HashSet<Point> {
        let mut resp: HashSet<Point> = HashSet::new();
        let range: &'static [isize; 3] = &[-1, 0, 1];
        for &x in range {
            for &y in range {
                for &z in range {
                    for &w in range {
                        let p: Point = Point {
                            x: point.x + x,
                            y: point.y + y,
                            z: point.z + z,
                            w: point.w + w,
                        };
                        if p != *point {
                            resp.insert(p);
                        }
                    }
                }
            }
        }
        resp
    }

    fn get_points(&self) -> HashSet<&Point> {
        self.data.keys().collect()
    }

    #[allow(unused)]
    fn point_neighbors_sorted(&self, point: &Point) -> Vec<Point> {
        let mut vec = self
            .point_neighbors(point)
            .iter()
            .copied()
            .collect::<Vec<Point>>();
        vec.sort();
        vec
    }

    #[allow(unused)]
    fn get_points_sorted(&self) -> Vec<&Point> {
        let mut vec = self.data.keys().collect::<Vec<&Point>>();
        vec.sort();
        vec
    }
}

const ACTIVE: char = '#';
const INACTIVE: char = '.';

fn main() {
    let input = fs::read_to_string("src/bin/input17.txt").unwrap();
    let space: Space = Space::from_string_2d(input);

    // For part 1, remove "w" from Point and related methods
    let result_space = permute(6, space);
    println!(
        "num active: {}",
        result_space
            .get_points()
            .iter()
            .filter(|p| result_space.point_is(p, &ACTIVE))
            .count()
    );
}

// If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
// If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
fn permute(iters: usize, space: Space) -> Space {
    let mut space_copy_last;

    // Make sure all points have neighbors that exist for next round (default INACTIVE)
    // Arbitrary block so I can reuse familiar variable names for read/write copies :(
    {
        let space_copy_read = space.clone();
        let mut space_copy_write = space;

        for point in space_copy_read.get_points() {
            for new_adj in space_copy_read.point_neighbors(point) {
                if !space_copy_read.has_point(&new_adj) {
                    space_copy_write.add_by_point(new_adj, INACTIVE);
                }
            }
        }

        space_copy_last = space_copy_write;
    }

    for _ in 0..iters {
        // Make some copies for read/write
        let space_copy_read = space_copy_last.clone();
        let mut space_copy_write = space_copy_last.clone();

        // Calculate next movements
        for point in space_copy_read.get_points() {
            let num_adj_active = space_copy_read
                .point_neighbors(point)
                .iter()
                .filter(|p| space_copy_read.point_is(p, &ACTIVE))
                .count();

            // Evaluate point in space by itself, update next copy
            match space_copy_read.point_value(point) {
                Some(&ACTIVE) => {
                    if num_adj_active < 2 || num_adj_active > 3 {
                        space_copy_write.add_by_point(*point, INACTIVE);
                    }
                }
                Some(&INACTIVE) => {
                    if num_adj_active == 3 {
                        space_copy_write.add_by_point(*point, ACTIVE);
                    }
                }
                Some(x) => {
                    panic!("point value is invalid: {:?} {}", point, x)
                }
                None => {
                    panic!("point value was none")
                }
            }
        }

        // Make sure all points have neighbors that exist for next round (default INACTIVE)
        for point in space_copy_read.get_points() {
            for new_adj in space_copy_read.point_neighbors(point) {
                if !space_copy_read.has_point(&new_adj) {
                    space_copy_write.add_by_point(new_adj, INACTIVE);
                }
            }
        }

        space_copy_last = space_copy_write;
    }
    space_copy_last
}
