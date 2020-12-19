use crate::common::Part;

#[derive(PartialEq, Debug)]
enum Token {
    Number(i64),
    Plus,
    Times,
    LeftP,
    RightP,
}

fn tokenize(s: &String) -> Vec<Token> {
    s.replace("(", " ( ").replace(")", " ) ").split_whitespace().map(|t| {
        match t {
            "+" => Token::Plus,
            "*" => Token::Times,
            ")" => Token::RightP,
            "(" => Token::LeftP,
            _ => Token::Number(t.parse().expect("Number expected")),
        }
    }).collect()
}

fn eval_until<C>(values: &mut Vec<i64>, operators: &mut Vec<Token>, cond: C)
    where C: Fn(&Token) -> bool
{
    while operators.len() > 0 && cond(operators.last().unwrap()) {
        let op = operators.pop().expect("Missing operator.");
        let a = values.pop().expect("Not enough operands.");
        let b = values.pop().expect("Not enough operands.");
        match op {
            Token::Plus => values.push(a+b),
            Token::Times => values.push(a*b),
            _ => panic!("Unknown operator."),
        }
    }
}

fn evaluate(tokens: &Vec<Token>, precedence: fn(&Token) -> i32) -> i64 {
    let mut operators = Vec::new();
    let mut values = Vec::new();
    // println!("tokens: {:?}", tokens);
    for token in tokens {
        match token {
            Token::Plus => {
                eval_until(&mut values, &mut operators, |op| precedence(op) >= precedence(&Token::Plus));
                operators.push(Token::Plus);
            },
            Token::Times => {
                eval_until(&mut values, &mut operators, |op| precedence(op) >= precedence(&Token::Times));
                operators.push(Token::Times);
            },
            Token::LeftP => operators.push(Token::LeftP),
            Token::RightP => {
                eval_until(&mut values, &mut operators, |op| op != &Token::LeftP);
                let lp = operators.pop();
                assert_eq!(lp.unwrap(), Token::LeftP);
            },
            Token::Number(num) => values.push(*num),
        }
    }
    eval_until(&mut values, &mut operators, |_| true);
    assert!(operators.is_empty());
    assert_eq!(values.len(), 1);
    values[0]
}

fn precedence1(op: &Token) -> i32 {
    match op {
        Token::Plus => 1,
        Token::Times => 1,
        _ => 0,
    }
}

fn precedence2(op: &Token) -> i32 {
    match op {
        Token::Plus => 2,
        Token::Times => 1,
        _ => 0,
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let iter : Vec<Vec<Token>> = data.iter().map(tokenize).collect();
    match part {
        Part::First => {
            println!("{}", iter.iter().map(|e| evaluate(e, precedence1)).sum::<i64>());
        },

        Part::Second => {
            println!("{}", iter.iter().map(|e| evaluate(e, precedence2)).sum::<i64>());
        },
    }
}
