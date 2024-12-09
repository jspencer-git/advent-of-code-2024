use std::convert::From;
use std::fmt;
use std::str::FromStr;
use std::{fs::File, io::Read};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Grid<T>
where
    T: From<char> + fmt::Display,
{
    pub data: Vec<T>,
    size: (i64, i64),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGridError;

impl<T> FromStr for Grid<T>
where
    T: From<char> + fmt::Display,
{
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or("Could not get width")
            .map_err(|_| ParseGridError)?
            .len();

        let height = s.lines().count();
        let size = (width as i64, height as i64);

        let mut data = Vec::new();

        for ch in s.chars() {
            if ch == '\n' {
                continue;
            }

            let grid_elem = T::from(ch);
            data.push(grid_elem);
        }

        Ok(Self { data, size })
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: From<char> + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut line_idx = 0;
        for elem in self.data.iter() {
            write!(f, "{}", elem)?;
            line_idx += 1;
            if line_idx == self.size.0 {
                line_idx = 0;
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl<T> Grid<T> where T: From<char> + fmt::Display {
    pub fn get_idx_from_xy(&self, xy: (i64, i64)) -> Option<usize> {
        if xy.0 < 0 || xy.1 < 0 || xy.0 >= self.size.0 || xy.1 >= self.size.1 {
            return None;
        } 

        let idx = xy.1 * self.size.0 + xy.0;
        if idx < 0 || idx >= self.size.0 * self.size.1 {
            return None;
        }

        Some(idx as usize)
    }

    pub fn get_xy_from_idx(&self, idx: usize) -> Option<(i64, i64)> {
        let y  = idx as i64 / self.size.0;
        let x = idx as i64 % self.size.0;
        if y < 0 || y >= self.size.1 {
            return None;
        }

        Some((x, y))
    }

    pub fn get(&self, idx: usize) -> &T {
        &self.data[idx]
    }

    pub fn set(&mut self, idx: usize, value: T) {
        self.data[idx] = value;
    }
}

pub fn get_input(path: &str) -> String {
    let file = File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).unwrap();
    input_data
}

#[cfg(test)]
mod tests {
    use std::path::Display;

    use super::*;

    enum TestGridElem {
        Empty,
        Object,
    }

    impl From<char> for TestGridElem {
        fn from(item: char) -> Self {
            match item {
                '.' => Self::Empty,
                'x' => Self::Object,
                _ => unreachable!(),
            }
        }
    }

    impl fmt::Display for TestGridElem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let ch = match self {
                Self::Empty => '.',
                Self::Object => 'x',
                _ => unreachable!(),
            };
            write!(f, "{}", ch)
        }
    }

    #[test]
    fn test_grid() {
        const TEST_STR: &str = ".....\n\
                                .....\n\
                                .x...\n\
                                .....\n\
                                ....x";

        let g: Grid<TestGridElem> = Grid::from_str(TEST_STR).unwrap();
        println!("{}", g);
    }

    #[test]
    fn test_grid_modify() {
        const TEST_STR: &str = ".....\n\
                                .....\n\
                                .x...\n\
                                .....\n\
                                ....x";

        let mut g: Grid<TestGridElem> = Grid::from_str(TEST_STR).unwrap();
        g.set(g.get_idx_from_xy((1, 1)).unwrap(), TestGridElem::Object);
        
        println!("{}", g);
    }
}
