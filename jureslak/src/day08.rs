use crate::common::Part;

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc,
    Jmp,
}

fn parse_line(s: &String) -> (Instruction, i32) {
    let parts : Vec<&str> = s.split(' ').collect();
    assert_eq!(parts.len(), 2);
    let inst = match parts[0] {
        "jmp" => Instruction::Jmp,
        "acc" => Instruction::Acc,
        "nop" => Instruction::Nop,
        _ => panic!("Unknown instruction."),
    };
    let off : i32 = parts[1].parse().expect("Expected an integer.");
    (inst, off)
}

#[derive(Debug)]
enum ExitKind {
    Loop(i32, usize),
    Success(i32, usize),
    Fail(i32, usize),
}

fn run_program(prog: &Vec<(Instruction, i32)>) -> ExitKind {
    let mut pc : usize = 0;
    let mut acc : i32 = 0;
    let mut seen = vec![false; prog.len()];
    loop {
        seen[pc] = true;
        let next = match prog[pc].0 {
            Instruction::Nop => pc + 1,
            Instruction::Jmp => ((pc as i32) + prog[pc].1) as usize,
            Instruction::Acc => {
                acc += prog[pc].1;
                pc + 1
            }
        };
        if next == prog.len() {
            return ExitKind::Success(acc, pc);
        }
        if next >= prog.len() {
            return ExitKind::Fail(acc, pc);
        }
        if seen[next] {
            return ExitKind::Loop(acc, pc);
        }
        pc = next;
    }
}

fn toggle(program: &mut Vec<(Instruction, i32)>, idx: usize) {
    program[idx].0 = match program[idx].0 {
        Instruction::Jmp => Instruction::Nop,
        Instruction::Nop => Instruction::Jmp,
        Instruction::Acc => Instruction::Acc,
    };
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut program : Vec<(Instruction, i32)> = data.iter().map(parse_line).collect();
    // println!("{:?}", program);
    let exit = run_program(&program);
    let (acc, _) = match exit {
        ExitKind::Loop(acc, pc) => (acc, pc),
        _ => panic!("Unexpected exit."),
    };
    match part {
        Part::First => println!("{}", acc),
        Part::Second => {
            for pc in 0..program.len() {
                match program[pc].0 { Instruction::Acc => continue, _ => () }
                toggle(&mut program, pc);
                let exit = run_program(&program);
                match exit {
                    ExitKind::Success(acc, pc) => println!("acc: {}, pc: {}", acc, pc),
                    _ => (),
                }
                toggle(&mut program, pc);
            }
        },
    }
}
