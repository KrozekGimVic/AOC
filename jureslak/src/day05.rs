use crate::common::Part;

#[derive(Ord, Debug, Eq, PartialOrd, PartialEq)]
struct Range {
    start: i64,
    map_start: i64,
    len: i64,
}

impl Range {
    fn contains(&self, x: i64) -> bool {
        self.start <= x && x < self.start + self.len
    }
    fn map(&self, x: i64) -> i64 {
        assert!(self.start <= x, "{} <= {}", self.start, x);
        assert!(x <= self.start + self.len, "{} <= {}", x, self.start+self.len);
        self.map_start + (x - self.start)
    }
}

#[derive(Debug)]
struct MyMap {
    // name: String,
    ranges: Vec<Range>
}

fn map_seed(map: &MyMap, seed: i64) -> i64 {
    for range in &map.ranges {
        if range.contains(seed) {
            return range.map(seed);
        }
    }
    seed
}

fn map_range(map: &MyMap, range: &std::ops::Range<i64>) -> Vec<std::ops::Range<i64>> {
    let mut result = vec![];

    // println!("mapping {:?} per {:?}", range, map);
    let mut s = range.start;
    for r in &map.ranges {
        if range.end < r.start || r.start+r.len < range.start { continue; }
        if s < r.start {
            result.push(s..r.start);
            s = r.start;
        }
        if range.end < r.start + r.len {
            result.push(r.map(s)..r.map(range.end));
            s = range.end;
            break;
        }
        result.push(r.map(s)..r.map(r.start+r.len));
        s = r.start+r.len;
    }
    if s < range.end {
        result.push(s..range.end);
    }


    // println!(".. {:?} -> {:?}", range, result);
    result
}

fn full_map_seed(maps: &Vec<MyMap>, seed: i64) -> i64 {
    let mut s = seed;
    for m in maps {
        // println!("{} -> {}", s, map_seed(m, s));
        s = map_seed(m, s);
    }
    // println!("------------");
    s
}

pub fn solve(data : &Vec<String>, part : Part) {
    let seeds: Vec<i64> = data[0].strip_prefix("seeds: ").unwrap().split(' ').map(|s| s.parse().unwrap()).collect();

    let mut maps : Vec<MyMap> = vec![];
    let mut i = 2;
    while i < data.len() {
        // let name = data[i].clone();
        i += 1;
        let mut ranges = vec![];
        while i < data.len() && !data[i].is_empty() {
            let nums : Vec<i64> = data[i].split(' ').map(|s| s.parse().unwrap()).collect();
            ranges.push(Range{ start: nums[1], map_start: nums[0], len: nums[2] });
            i += 1;
        }
        ranges.sort();
        i += 1; // skup empty row
        if !ranges.is_empty() {
            maps.push(MyMap{
                // name,
                ranges,
            })
        }
    }

    match part {
        Part::First => {
            println!("{}", seeds.iter().map(|s| full_map_seed(&maps, *s)).min().unwrap());
        }
        Part::Second => {
            let mut start_ranges = vec![];
            for i in 0..(seeds.len() / 2) {
                start_ranges.push(seeds[2*i]..(seeds[2*i]+seeds[2*i+1]));
            }
            let sum: i64 = start_ranges.iter().map(|r| r.end-r.start).sum();

            for map in maps {
                let mut new_ranges = vec![];
                for r in &start_ranges {
                    new_ranges.extend(map_range(&map, &r));
                }
                // println!("{:?} -> {:?}", start_ranges, new_ranges);
                start_ranges = new_ranges;
                start_ranges.sort_by(|s, t| s.start.cmp(&t.start));
                assert_eq!(sum, start_ranges.iter().map(|r| r.end-r.start).sum());
            }

            println!("{}", start_ranges.iter().map(|r| r.start).min().unwrap());
        }
    }
}
