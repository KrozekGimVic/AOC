use crate::common::Part;

fn roll(data: &mut Vec<Vec<u8>>, di: isize, dj: isize) {
    for mut i in 0..data.len() {
        if di > 0 { i = data.len() - i - 1; }
        for mut j in 0..data[i].len() {
            if dj > 0 { j = data[i].len() - j - 1; }
            if data[i][j] == b'O' {
                let mut k = 1;
                loop {
                    let ni = i as isize + k*di;
                    let nj = j as isize + k*dj;
                    if 0 <= ni && (ni as usize) < data.len() && 0 <= nj && (nj as usize) < data[0].len() && data[ni as usize][nj as usize] == b'.' {
                        k += 1;
                    } else {
                        break;
                    }
                }
                k -= 1;
                if k > 0 {
                    data[(i as isize + k*di) as usize][(j as isize + k*dj) as usize] = b'O';
                    data[i][j] = b'.';
                }
            }
        }
    }
}

fn get_load(data: &Vec<Vec<u8>>) -> usize {
    let mut load = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == b'O' {
                load += data.len()-i;
            }
        }
    }
    load
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut data : Vec<Vec<u8>> = data.iter().map(|s| s.as_bytes().to_vec()).collect();

    match part {
        Part::First => {
            let mut data = data.to_vec();
            roll(&mut data, -1, 0);
            println!("{}", get_load(&data));
        }
        Part::Second => {
            const LIMIT : usize = 1000;
            let dis = [-1, 0, 1, 0, -1];
            let djs = [0, -1, 0, 1, 0];
            let mut loads = vec![];
            for i in 0..2000 {
                for j in 0..4 {
                    roll(&mut data, dis[j % 4], djs[j % 4]);
                }
                if i >= LIMIT {
                    loads.push(get_load(&data));
                }
            }

            let mut period = 0;
            for per in 1..100 {
                let mut ok = true;
                'out: for j in 0..per {
                    let c = loads[j];
                    for i in (j..loads.len()).step_by(per) {
                        if loads[i] != c {
                            ok = false;
                            break 'out;
                        }
                    }
                }
                if ok {
                    period = per;
                    break;
                }
            }
            println!("Period is {}", period);
            assert_ne!(period, 0);

            let idx = (1000000000 - LIMIT - 1) % period;
            println!("{:?}, idx: {}", &loads[..period], idx);

            println!("{}", loads[idx]);
        }
    }
}
