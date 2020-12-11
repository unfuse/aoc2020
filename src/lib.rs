pub mod data {
    use std::collections::HashMap;
    use std::fmt;
    use std::fmt::Formatter;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Data {
        width: usize,
        height: usize,
        data: HashMap<(usize, usize), char>,
    }

    impl Data {
        pub fn new(input: &str) -> Data {
            let mut data: HashMap<(usize, usize), char> = HashMap::new();
            let mut width: usize = 0;
            let mut height: usize = 0;

            for (i, line) in input.lines().enumerate() {
                width = line.len();
                for (j, c) in line.chars().enumerate() {
                    data.insert((j, i), c);
                }
                height += 1;
            }

            Data {
                width,
                height,
                data,
            }
        }

        pub fn get(&self, x: usize, y: usize) -> Option<char> {
            if let Some(&data) = self.data.get(&(x, y)) {
                Some(data)
            } else {
                None
            }
        }

        pub fn is(&self, x: usize, y: usize, target: char) -> bool {
            if let Some(data) = self.get(x, y) {
                data == target
            } else {
                false
            }
        }

        pub fn update(&mut self, x: usize, y: usize, target: char) -> Option<char> {
            self.data.insert((x, y), target)
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.height
        }

        pub fn has_key(&self, x: isize, y: isize) -> bool {
            x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
        }
    }

    impl fmt::Display for Data {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for y in 0..self.height {
                for x in 0..self.width {
                    write!(f, "{}", self.get(x, y).unwrap())?;
                }
                println!();
            }

            write!(f, "")
        }
    }
}
