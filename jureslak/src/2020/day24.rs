use crate::common::Part;
use std::collections::HashSet;
use std::ops::{Add, AddAssign};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct CubeCoord {
    x: i32, y: i32, z: i32,
}

impl CubeCoord {
    fn neighbours(&self) -> Vec<CubeCoord> {
        vec![
            self + &CubeCoord::direction("nw"),
            self + &CubeCoord::direction("ne"),
            self + &CubeCoord::direction("e"),
            self + &CubeCoord::direction("se"),
            self + &CubeCoord::direction("sw"),
            self + &CubeCoord::direction("w"),
        ]
    }

    fn direction(d: &str) -> CubeCoord {
        match d {
            "e" => CubeCoord { x: 1, y: -1, z: 0},
            "ne" => CubeCoord { x: 1, y: 0, z: -1},
            "nw" => CubeCoord { x: 0, y: 1, z: -1},
            "w" => CubeCoord { x: -1, y: 1, z: 0},
            "sw" => CubeCoord { x: -1, y: 0, z: 1},
            "se" => CubeCoord { x: 0, y: -1, z: 1},
            _ => panic!("Unknown direction {}", d),
        }
    }
}

impl Add for &CubeCoord {
    type Output = CubeCoord;
    fn add(self, rhs: Self) -> Self::Output {
        CubeCoord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for CubeCoord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Debug)]
struct HexGrid {
    tiles: HashSet<CubeCoord>
}

impl HexGrid {
    fn walk(&mut self, dirs: &str) {
        let mut reference = CubeCoord { x: 0, y: 0, z: 0};
        let mut iter = dirs.chars();
        let mut c = iter.next();
        while c.is_some() {
            let mut dir = String::from(c.unwrap());
            if dir == "n" || dir == "s" {
                dir.push(iter.next().expect("Invalid direction."))
            }
            reference += CubeCoord::direction(&dir);
            c = iter.next();
        }
        if self.tiles.contains(&reference) {
            self.tiles.remove(&reference);
        } else {
            self.tiles.insert(reference);
        }
    }

    fn evolve(&mut self) {
        let mut new_tiles = HashSet::new();
        for black_tile in &self.tiles {
            let num_black_neighbours = black_tile.neighbours().iter().filter(|t| self.tiles.contains(t)).count();
            if !(num_black_neighbours == 0 || num_black_neighbours >= 2) {
                new_tiles.insert(black_tile.clone());
            }
        }
        for while_tile in self.tiles.iter().flat_map(|t| t.neighbours().into_iter()) {
            let num_black_neighbours = while_tile.neighbours().iter().filter(|t| self.tiles.contains(t)).count();
            if num_black_neighbours == 2 {
                new_tiles.insert(while_tile.clone());
            }
        }
        self.tiles = new_tiles;
    }
}

pub fn solve(data : &Vec<String>, part : Part) {

    let mut grid = HexGrid{ tiles: HashSet::new() };
    for s in data {
        grid.walk(s);
    }

    match part {
        Part::First => {
            println!("{}", grid.tiles.len());
        },
        Part::Second => {
            for _ in 0..100 {
                grid.evolve();
            }
            println!("{}", grid.tiles.len());
        },
    }
}
