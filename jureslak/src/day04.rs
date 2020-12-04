use crate::common::Part;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ParseError;


impl FromStr for ExpirationYear {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year : i32 = s.parse()?;
        if 1920 <= year && year <= 2002 {
            Err(ParseError::from("sss"))
        } else {
            Ok(ExpirationYear{ year })
        }
    }
}

impl FromStr for Height {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("in") {
            Ok(Height{value: s[..s.len()-2].parse()?, unit: Unit::In})
        } else if s.ends_with("cm") {
            Ok(Height{value: s[..s.len()-2].parse()?, unit: Unit::Cm})
        } else {
            Err(ParseError::from("asdf"))
        }
    }
}

impl FromStr for HairColor {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {}
            // && s.as_bytes()[0] == b'#' &&
            //     s.as_bytes()[1..].iter()
            //         .all(|&c| b'0' <= c && c <= b'9' || b'a' <= c && c <= b'f')
        // }
    }
}

struct Passport {
    birth_year: BirthYear,
    issue_year: IssueYear,
    expiration_year: ExpirationYear,
    height: Height,
    hair_color: HairColor,
    eye_color: EyeColor,
    passport_id: PassportId,
}

const BYR: &'static str = "byr";
const IYR: &'static str = "iyr";
const EYR: &'static str = "eyr";
const HGT: &'static str = "hgt";
const HCL: &'static str = "hcl";
const ECL: &'static str = "ecl";
const PID: &'static str = "pid";

const VALID_EYE_COLORS : [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
type Validator = fn(&str) -> bool;
const VALIDATORS: HashMap<&'static str, Validator> = [
    (BYR, int_range_validator(1920, 2002)),
    (IYR, int_range_validator(2010, 2020)),
    (EYR, int_range_validator(2020, 2030)),
    // (HGT, length_validator([("cm", (150, 193)), ("in", (59, 76))])),
    // (HCL, rbg_validator),
    // (ECL, one_of_validator(VALID_EYE_COLORS)),
    // (PID, int_seq_validator(9)),
].iter().cloned().collect();

impl Passport {
    fn valid_keys(data: &HashMap<String, String>) -> bool {
        VALIDATORS.keys().all(|&k| data.contains_key(&String::from(k)))
    }

    fn valid_values(data: &HashMap<String, String>) -> bool {
        VALIDATORS.keys().all(|&k| VALIDATORS[k](data[k]))
    }
}

fn int_range_validator(from: i32, to: i32) -> Validator {
    |s| {
        let x = s.parse();
        if x.is_err() { return false; }
        let x = x.unwrap();
        return 
    }
}

// impl Validatable for EyeColor {
//     fn valid(&self) -> bool {
//         VALID_EYE_COLORS.any(|c| self.value.as_bytes() == c)
//     }
// }
// impl Validatable for PassportId {
//     fn valid(&self) -> bool {
//         self.id.len() == 9 &&
//             self.id.as_bytes().iter().all(|&c| b'0' <= c && c <= b'9')
//     }
// }
// impl Validatable for Passport {
//     fn valid(&self) -> bool {
//     }
// }

pub fn solve(data : &Vec<String>, part : Part) {
    let mut passports = Vec::new();
    let mut cur_passport = HashMap::new();
    for line in data.iter() {
        if line.is_empty() {
            passports.push(cur_passport.clone());
            cur_passport.clear();
        } else {
            for item in line.split(' ') {
                let kv : Vec<&str>= item.split(':').collect();
                assert_eq!(kv.len(), 2);
                cur_passport.insert(kv[0].to_string(), kv[1].to_string());
            }
        }
    }
    if !cur_passport.is_empty() {
        passports.push(cur_passport.clone());
    }
    let passports: Vec<HashMap<String, String>> = passports.into_iter()
        .filter(Passport::valid_keys).collect();

    match part {
        Part::First => println!("{}", passports.len()),
        Part::Second => println!("{}", passports.iter().filter_map(|s| Passport::from(s).ok())),
    }
}