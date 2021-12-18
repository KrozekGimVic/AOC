use crate::common::Part;
use std::iter;
use itertools::Itertools;

fn read_number(bits: &mut impl Iterator<Item=bool>, n: i64) -> i64 {
    let mut r = 0;
    for _i in 0..n {
        r <<= 1;
        r |= bits.next().unwrap() as i64;
    }
    r
}

fn read_number_payload(bits: &mut impl Iterator<Item=bool>) -> (i64, usize) {
    let mut r = 0;
    let mut bit_length = 0;
    loop {
        let is_next = bits.next().unwrap();
        r <<= 4;
        r |= read_number(bits, 4);
        bit_length += 5;
        if !is_next { break; }
    }
    (r, bit_length)
}

#[derive(Debug)]
struct Packet {
    version: i32,
    bit_length: usize,
    payload: PacketPayload,
}


#[derive(Debug)]
enum OperationType {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}
#[derive(Debug)]
enum PacketPayload {
    Operation((OperationType, Vec<Packet>)),
    Number(i64),
}

fn parse_packet(it: &mut impl Iterator<Item=bool>) -> Packet {
    let version = read_number(it, 3) as i32;
    let type_id = read_number(it, 3) as i32;
    match type_id {
        4 => {
            let (num, bit_length) = read_number_payload(it);
            Packet{ version, bit_length: 6 + bit_length, payload: PacketPayload::Number(num)}
        },
        type_id => {
            let len_id = it.next().unwrap();
            let (sub_packets, bit_length) = if !len_id {
                let num_bits = read_number(it, 15) as usize;
                let mut sub_packets = vec![];
                let mut bit_length = 0;
                loop {
                    let p = parse_packet(it);
                    bit_length += p.bit_length;
                    sub_packets.push(p);
                    if bit_length >= num_bits {
                        if bit_length > num_bits { panic!("read {} bits, which is over budget for {}", bit_length, num_bits); }
                        break;
                    }
                }
                (sub_packets, 16 + bit_length)
            } else {
                let num_packets = read_number(it, 11) as usize;
                let sub_packets: Vec<Packet> = iter::repeat_with(|| parse_packet(it)).take(num_packets).collect();
                let sub_bit_length = sub_packets.iter().map(|p| p.bit_length).sum::<usize>();
                (sub_packets, 12 + sub_bit_length)
            };
            let op_type = match type_id {
                0 => OperationType::Sum,
                1 => OperationType::Product,
                2 => OperationType::Min,
                3 => OperationType::Max,
                5 => OperationType::Gt,
                6 => OperationType::Lt,
                7 => OperationType::Eq,
                _ => panic!("Unknown type"),
            };
            Packet {
                version,
                bit_length: 6 + bit_length,
                payload: PacketPayload::Operation((op_type, sub_packets))
            }
        }
    }
}

fn sum_versions(p: &Packet) -> i32 {
    p.version + match &p.payload {
        PacketPayload::Operation((_, sub_packets)) => sub_packets.iter().map(sum_versions).sum(),
        _ => 0,
    }
}

fn eval(p: &Packet) -> i64 {
    match &p.payload {
        PacketPayload::Number(a) => *a,
        PacketPayload::Operation((op_type, sub_packages)) => {
            let subp = sub_packages.iter().map(eval);
            match op_type {
                OperationType::Sum => subp.sum(),
                OperationType::Product => subp.product(),
                OperationType::Max => subp.max().unwrap(),
                OperationType::Min => subp.min().unwrap(),
                OperationType::Gt => {
                    let (first, second) = subp.collect_tuple().unwrap();
                    (first > second) as i64
                },
                OperationType::Lt => {
                    let (first, second) = subp.collect_tuple().unwrap();
                    (first < second) as i64
                },
                OperationType::Eq => {
                    let (first, second) = subp.collect_tuple().unwrap();
                    (first == second) as i64
                },
            }
        }
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let bits = data[0].as_bytes().iter().map(|&b| {
        if b'A' <= b && b <= b'F' { b - b'A' + 10 } else { b - b'0' }
    }).map(|n| {
        iter::once(n & 0b1000 > 0).chain(
            iter::once(n & 0b0100 > 0)).chain(
            iter::once(n & 0b0010 > 0)).chain(
            iter::once(n & 0b0001 > 0))
    }).flatten();
    let mut it = bits.into_iter();

    let p = parse_packet(&mut it);

    match part {
        Part::First => {
            println!("{:?}", p);
            println!("{}", sum_versions(&p))
        }
        Part::Second => {
            println!("{}", eval(&p));
        }
    }
}
