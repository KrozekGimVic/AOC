use crate::common::Part;

use std::str::FromStr;
use gcd::Gcd;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: FromStr> FromStr for Point<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers : Vec<&str> = s.split(",").collect();
        assert_eq!(numbers.len(), 2);
        let x = numbers[0].parse()?;
        let y = numbers[1].parse()?;
        Ok(Point{x, y})
    }
}

#[derive(Debug)]
struct LineSegment<T> {
    start: Point<T>,
    end: Point<T>,
}

impl<T: FromStr> FromStr for LineSegment<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers : Vec<&str> = s.split(" -> ").collect();
        assert_eq!(numbers.len(), 2);
        let start = numbers[0].parse()?;
        let end = numbers[1].parse()?;
        Ok(LineSegment{start, end})
    }
}

impl<T: PartialEq> LineSegment<T> {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

fn draw_lines(lines: &Vec<LineSegment<isize>>, size: usize, count_diagonal: bool) -> Vec<Vec<i32>> {
    let mut drawing = vec![vec![0; size]; size];
    for line in lines.iter() {
        if line.is_horizontal() || line.is_vertical() || count_diagonal {
            let diff_x = line.end.x - line.start.x;
            let diff_y = line.end.y - line.start.y;
            let common = (diff_x.abs() as usize).gcd(diff_y.abs() as usize) as isize;
            let dx = diff_x / common;
            let dy = diff_y / common;
            for i in 0..common+1 {
                drawing[(line.start.y + i*dy) as usize][(line.start.x + i*dx) as usize] += 1;
            }
        }
    }
    drawing
}

pub fn solve(data : &Vec<String>, part : Part) {
    let lines : Vec<LineSegment<isize>> = data.iter().map(|s| s.parse().unwrap()).collect();
    let size = 1000;

    match part {
        Part::First => {
            let drawing = draw_lines(&lines, size, false);
            println!("{}", drawing.iter().flatten().filter(|&&c| c >= 2).count());
        }
        Part::Second => {
            let drawing = draw_lines(&lines, size, true);
            // for line in drawing.iter() {
            //     println!("{}", String::from_utf8(line.iter().map(|&s| s as u8 + b'0').collect()).unwrap())
            // }
            println!("{}", drawing.iter().flatten().filter(|&&c| c >= 2).count());
        }
    }
}
