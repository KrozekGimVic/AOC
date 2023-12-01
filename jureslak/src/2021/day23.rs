use crate::common::Part;

#[derive(Debug, Clone)]
struct Space {
    free: [u8; 7],
    rooms: [Room; 4],
}

#[derive(Debug, clone)]
struct Room {
    deep: u8,
    shallow: u8,
}

impl Space {
    fn all_moves(&self) {
        let mut opts = vec![];
        for i in 0..self.free.len() {
            if self.free[i] > 0 {

            }
        }
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let space = Space {
        free: [0; 7],
        rooms: [
            Room {
                shallow: data[2].as_bytes()[3] - b'A' + 1,
                deep: data[3].as_bytes()[3] - b'A' + 1,
            },
            Room {
                shallow: data[2].as_bytes()[5] - b'A' + 1,
                deep: data[3].as_bytes()[5] - b'A' + 1,
            },
            Room {
                shallow: data[2].as_bytes()[7] - b'A' + 1,
                deep: data[3].as_bytes()[7] - b'A' + 1,
            },
            Room {
                shallow: data[2].as_bytes()[9] - b'A' + 1,
                deep: data[3].as_bytes()[9] - b'A' + 1,
            },
        ],
    };

    println!("{:?}", space);

    match part {
        Part::First => {
            println!("{}", 3);
        }
        Part::Second => {
            println!("{}", 2);
        }
    }
}
