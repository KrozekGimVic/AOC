use crate::common::Part;
use std::collections::HashMap;

const MASK_SIZE : usize = 36;
#[derive(Clone, Copy, Debug)]
enum MaskBit { SetOne, SetZero, Keep }
type Mask = [MaskBit; MASK_SIZE];
#[derive(Debug)]
enum Line {
    Mask(Mask),
    Set((usize, i64))
}

fn parse_line(s: &String) -> Line {
    if s.starts_with("mask") {
        let mut mask = [MaskBit::Keep; MASK_SIZE];
        assert_eq!(s.len(), MASK_SIZE+7);
        for (i, c) in s.as_bytes()[7..].iter().enumerate() {
            mask[i] = match c {
                b'X' => MaskBit::Keep,
                b'0' => MaskBit::SetZero,
                b'1' => MaskBit::SetOne,
                _ => panic!("Invalid input."),
            };
        }
        Line::Mask(mask)
    } else {
        let parts : Vec<&str> = s.split("] = ").collect();
        assert_eq!(parts.len(), 2);
        let addr = parts[0][4..].parse().expect("Invalid input.");
        let value = parts[1].parse().expect("Invalid input.");
        Line::Set((addr, value))
    }
}

fn apply_mask(mask: &Mask, value: i64) -> i64 {
    let mut masked_value = value;
    for i in 0..MASK_SIZE {
        let sh = MASK_SIZE - 1 - i;
        match mask[i] {
            MaskBit::SetOne => masked_value |= 1 << sh,
            MaskBit::SetZero => masked_value &= !(1 << sh),
            MaskBit::Keep => continue,
        }
    }
    masked_value
}

fn execute(prog : &Vec<Line>) -> i64 {
    let mut mem = vec![0; 1 << 16];
    let mut mask = [MaskBit::Keep; MASK_SIZE];
    for line in prog.iter() {
        match line {
            Line::Mask(new_mask) => { mask = new_mask.clone(); },
            Line::Set((addr, value)) => {
                mem[*addr] = apply_mask(&mask, *value);
            }
        }
    }
    mem.iter().sum()
}

fn alter_mask(mask: &Mask, addr: usize) -> Mask {
    let mut new_mask = mask.clone();
    for i in 0..MASK_SIZE {
        let sh = MASK_SIZE - 1 - i;
        if addr & (1 << sh) != 0 {
            new_mask[i] = MaskBit::SetOne;
        }
    }
    new_mask
}

fn to_int(mask: &Mask) -> usize {
    let mut x = 0;
    for i in 0..MASK_SIZE {
        match mask[i] {
            MaskBit::SetOne => x |= 1 << (MASK_SIZE - 1 - i),
            _ => continue,
        }
    }
    x
}

fn set_addr(mem: &mut HashMap<usize, i64>, addr: usize, mask: &Mask, value: i64) {
    let floats : Vec<usize> = mask.iter().enumerate().filter_map(|(i, v)| {
        match v { MaskBit::Keep => Some(i), _ => None, }
    }).collect();
    let new_mask = alter_mask(&mask, addr);
    for opt in 0..(1 << floats.len()) {
        let mut tmp_mask = new_mask.clone();
        for i in 0..floats.len() {
            tmp_mask[floats[i]] = if opt & (1 << i) != 0 { MaskBit::SetOne } else { MaskBit::SetZero }
        }
        mem.insert(to_int(&tmp_mask), value);
    }
    // println!("{:?}", mem);
}

fn execute_as_mad(prog : &Vec<Line>) -> i64 {
    let mut mem = HashMap::new();
    let mut mask = [MaskBit::Keep; MASK_SIZE];
    for line in prog.iter() {
        match line {
            Line::Mask(new_mask) => { mask = new_mask.clone(); },
            Line::Set((addr, value)) => {
                set_addr(&mut mem, *addr, &mask, *value);
            }
        }
    }
    mem.values().sum()
}

pub fn solve(data : &Vec<String>, part : Part) {
    let prog : Vec<Line> = data.iter().map(parse_line).collect();
    // println!("{:?}", prog);
    match part {
        Part::First => {
            println!("{}", execute(&prog));
        },
        Part::Second => {
            println!("{}", execute_as_mad(&prog));
        },
    }
}
