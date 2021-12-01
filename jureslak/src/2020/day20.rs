use crate::common::Part;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    img: Vec<Vec<u8>>,
    borders: [Vec<u8>; 4],
}

fn normalize(v: Vec<u8>) -> Vec<u8> {
    let mut r = v.clone();
    r.reverse();
    if r < v { r } else { v }
}

impl Tile {
    fn from(id: usize, img: Vec<Vec<u8>>) -> Tile {
        let mut start_col = Vec::new();
        let mut end_col = Vec::new();
        for line in img.iter() {
            start_col.push(line[0]);
            end_col.push(line[line.len()-1]);
        }
        let start_row = img[0].clone();
        let end_row = img[img.len()-1].clone();

        Tile {
            id,
            img,
            borders: [
                normalize(start_col),
                normalize(start_row),
                normalize(end_col),
                normalize(end_row),
            ],
        }
    }
    fn flip(&self) -> Tile {
        let mut new_img = self.img.clone();
        for line in new_img.iter_mut() {
            line.reverse();
        }
        Tile {
            id: self.id,
            img: new_img,
            borders: self.borders.to_owned(),
        }
    }

    fn rotate(&self) -> Tile {
        let mut new_img = self.img.clone();
        let side = new_img.len();
        for i in 0..side {
            for j in 0..side {
                new_img[i][j] = self.img[side-j-1][i];
            }
        }
        Tile {
            id: self.id,
            img: new_img,
            borders: self.borders.to_owned(),
        }
    }

    fn bot(&self) -> &Vec<u8> {
        &self.img[self.img.len()-1]
    }
    fn top(&self) -> &Vec<u8> {
        &self.img[0]
    }
    fn left(&self) -> Vec<u8> {
        let mut start_col = Vec::new();
        for line in self.img.iter() {
            start_col.push(line[0]);
        }
        start_col
    }
    fn right(&self) -> Vec<u8> {
        let mut end_col = Vec::new();
        for line in self.img.iter() {
            end_col.push(line[line.len()-1]);
        }
        end_col
    }
}

fn parse_tile(s: &str) -> Tile {
    let mut it = s.split("\n");
    let id = it.next().unwrap()[5..9].parse().unwrap();
    let mut img = Vec::new();
    for line in it {
        img.push(line.as_bytes().to_owned());
    }
    Tile::from(id, img)
}

fn match_tiles(tiles: &Vec<Tile>) -> HashMap<Vec<u8>, Vec<usize>> {
    let mut same_border = HashMap::new();
    for tile in tiles {
        for border in &tile.borders {
            same_border.entry(border.clone()).or_insert(vec![]).push(tile.id);
        }
    }
    same_border
}


fn assemble_tiles(tiles: &Vec<Tile>, same_border: &HashMap<Vec<u8>, Vec<usize>>, corner: usize) -> Vec<Vec<Tile>> {
    let side = (tiles.len() as f64).sqrt() as usize;
    assert_eq!(side*side, tiles.len());
    let tiles_map : HashMap<usize, &Tile> = tiles.iter().map(|t| (t.id, t)).collect();

    let mut img = Vec::new();
    // Find the correct initial rotation manually.
    img.push(vec![tiles_map[&corner].rotate().rotate().rotate()]);
    for j in 1..side {
        let border = img[0][j-1].right();
        let neighbours = same_border.get(&normalize(border.clone())).unwrap();
        assert_eq!(neighbours.len(), 2, "Left border of tile {} at j={} has too few neighbours: {:?}.", img[0][j-1].id, j, neighbours);
        let first = neighbours[0];
        let second = neighbours[1];
        assert!(first == img[0][j-1].id || second == img[0][j-1].id);
        let my_id = if img[0][j-1].id == first { second } else { first };

        // Try rotations.
        let mut tile = tiles_map[&my_id].clone();
        let mut found = false;
        'f: for _ in 0..2 {
            for _ in 0..4 {
                tile = tile.rotate();
                if tile.left() == border {
                    found = true;
                    break 'f;
                }
            }
            tile = tile.flip();
        }
        assert!(found);
        img[0].push(tile);
    }
    for i in 1..side {
        img.push(vec![]);
        for j in 0..side {
            let border = img[i-1][j].bot();
            let neighbours = same_border.get(&normalize(border.clone())).unwrap();
            assert_eq!(neighbours.len(), 2, "Bottom border of tile {} at i={}, j={} has too few neighbours: {:?}.", img[i-1][j].id, i, j, neighbours);
            let first = neighbours[0];
            let second = neighbours[1];
            assert!(first == img[i-1][j].id || second == img[i-1][j].id);
            let my_id = if img[i-1][j].id == first { second } else { first };

            // Try rotations.
            let mut tile = tiles_map[&my_id].clone();
            let mut found = false;
            'o: for _ in 0..2 {
                for _ in 0..4 {
                    tile = tile.rotate();
                    if tile.top() == border {
                        found = true;
                        break 'o;
                    }
                }
                tile = tile.flip();
            }
            assert!(found);
            img[i].push(tile);
        }
    }
    img
}

fn assemble_full_image(tile_grid: &Vec<Vec<Tile>>) -> Vec<Vec<u8>> {
    const BLOCK_SIZE : usize = 8;
    let n = tile_grid.len();
    let mut full_img = vec![vec![b' '; n*BLOCK_SIZE]; n*BLOCK_SIZE];
    for i in 0..n {
        for j in 0..tile_grid[i].len() {
            let tile = &tile_grid[i][j];
            for k in 1..tile.img.len()-1 {
                for l in 1..tile.img[k].len()-1 {
                    full_img[i*BLOCK_SIZE + k-1][j*BLOCK_SIZE + l-1] = tile.img[k][l];
                }
            }
        }
    }
    full_img
}

fn identify_monsters(img: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let monster = [
        b"                  # ",
        b"#    ##    ##    ###",
        b" #  #  #  #  #  #   ",
    ];
    let n = monster.len();
    let m = monster[0].len();

    let t = Tile::from(0, img.clone()).flip().rotate();

    let mut recognized = t.img.clone();
    let mut counter = 0;
    let s = img.len();
    for i in 0..s-n {
        for j in 0..s-m {
            let mut found = true;
            'm: for k in 0..n {
                for l in 0..m {
                    if monster[k][l] == b'#' && t.img[i+k][j+l] != b'#' {
                        found = false;
                        break 'm;
                    }
                }
            }
            if found {
                counter += 1;
                for k in 0..n {
                    for l in 0..m {
                        if monster[k][l] == b'#' && t.img[i+k][j+l] == b'#' {
                            recognized[i+k][j+l] = b'O';
                        }
                    }
                }
            }
        }
    }
    println!("Recognized {} monsters.", counter);
    recognized
}

pub fn solve(data : &Vec<String>, part : Part) {
    let tiles: Vec<Tile> = data.join("\n").split("\n\n").map(parse_tile).collect();

    let same_border = match_tiles(&tiles);

    let mut corners = Vec::new();
    for tile in tiles.iter() {
        let count = tile.borders.iter().filter(|&b| same_border[b].len() == 2).count();
        if count == 2 {
            corners.push(tile.id);
        }
    }
    assert_eq!(corners.len(), 4);
    match part {
        Part::First => {
            println!("{}", corners.iter().product::<usize>());
        },

        Part::Second => {
            let tile_grid = assemble_tiles(&tiles, &same_border, corners[0]);
            let full_img = assemble_full_image(&tile_grid);
            let monsters = identify_monsters(&full_img);

            for m in monsters.iter() {
                println!("{}", String::from_utf8(m.clone()).unwrap());
            }

            println!("{}", monsters.iter().map(|s| s.iter().filter(|&c| *c == b'#').count()).sum::<usize>());
        },
    }
}
