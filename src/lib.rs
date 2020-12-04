pub mod data {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq)]
    pub struct Data {
        width: usize,
        height: usize,
        data: HashMap<(usize, usize), char>
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

            Data { width, height, data }
        }

        pub fn get(&self, x: usize, y: usize) -> char {
            *(self.data.get(&(x, y)).unwrap())
        }

        pub fn is(&self, x: usize, y: usize, target: char) -> bool {
            self.get(x, y) == target
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.height
        }
    }
}
