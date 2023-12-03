use crate::common::Part;
use std::str::FromStr;
use std::error;
use std::fmt;

struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct UnknownColor;
impl fmt::Display for UnknownColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number out of valid range or otherwise invalid data...")
    }
}
impl error::Error for UnknownColor {}

impl FromStr for Draw {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let splitter = s.split(", ");
        let mut draw = Draw { red: 0, green: 0, blue: 0};
        for color in splitter {
            let mut draw_splitter = color.splitn(2, ' ');
            let num : i32 = draw_splitter.next().unwrap().parse()?;
            let col = draw_splitter.next().unwrap();

            if col == "red" {
                draw.red = num;
            } else if col == "blue" {
                draw.blue = num;
            } else if col == "green" {
                draw.green = num;
            } else {
                return Err(UnknownColor{})?
            }
        }
        Ok(draw)
    }
}

struct Game {
    id: i32,
    draws: Vec<Draw>
}

impl FromStr for Game {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let mut splitter = s.splitn(2, ": ");
        let game = splitter.next().unwrap();

        let mut game_splitter = game.splitn(2, ' ');
        game_splitter.next();
        let game_id : i32 = game_splitter.next().unwrap().parse()?;

        let draws = splitter.next().unwrap();

        Ok(Game {
            id: game_id,
            draws: draws.split("; ").map(|s| s.parse().unwrap()).collect()
        })
    }
}

fn is_ok(draw: &Draw) -> bool {
    draw.red <= 12 && draw.green <= 13 && draw.blue <= 14
}


pub fn solve(data : &Vec<String>, part : Part) {
    let games : Vec<Game> = data.iter().map(|s| s.parse().expect("parsing failed")).collect();

    match part {
        Part::First => {
            println!("{}", games.iter().filter(|g| g.draws.iter().all(is_ok)).map(|g| g.id).sum::<i32>());
        }
        Part::Second => {
            let power_sum : i32 = games.iter().map(|game| {
                let mut d = Draw { red: 0, green: 0, blue: 0};
                for draw in game.draws.iter() {
                    d.red = d.red.max(draw.red);
                    d.green = d.green.max(draw.green);
                    d.blue = d.blue.max(draw.blue);
                }
                d.red*d.green*d.blue
            }).sum();

            println!("{}", power_sum);

        }
    }
}
