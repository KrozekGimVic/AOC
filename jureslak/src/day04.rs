use crate::common::Part;
use std::collections::HashMap;
use std::str::FromStr;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct InvalidData;

impl fmt::Display for InvalidData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number out of valid range or otherwise invalid data...")
    }
}
impl error::Error for InvalidData {}

#[derive(Debug)]
struct BirthYear { year: i32 }
#[derive(Debug)]
struct IssueYear { year: i32 }
#[derive(Debug)]
struct ExpirationYear { year: i32 }
#[derive(Debug)]
enum Unit { In, Cm }
#[derive(Debug)]
struct Height { value: i32, unit: Unit}
#[derive(Debug)]
struct Color { r: u8, g: u8, b: u8 }
const VALID_EYE_COLORS : [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
#[derive(Debug)]
enum EyeColor { Whatever }
#[derive(Debug)]
struct PassportId { id: String }

fn is_in_range(s: &str, from: i32, to: i32) -> Result<i32> {
    let year : i32 = s.parse()?;
    if from <= year && year <= to {
        Ok(year)
    } else {
        Err(InvalidData)?
    }
}

impl FromStr for BirthYear {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> { Ok(BirthYear{ year: is_in_range(s, 1920, 2002)? }) }
}
impl FromStr for IssueYear {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> { Ok(IssueYear{ year: is_in_range(s, 2010, 2020)? }) }
}
impl FromStr for ExpirationYear {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> { Ok(ExpirationYear{ year: is_in_range(s, 2020, 2030)? }) }
}

impl FromStr for Height {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        if s.ends_with("cm") {
            Ok(Height{value: is_in_range(&s[..s.len() - 2], 150, 193)?, unit: Unit::Cm})
        } else if s.ends_with("in") {
            Ok(Height{value: is_in_range(&s[..s.len() - 2], 59, 76)?, unit: Unit::In})
        } else {
            Err(InvalidData)?
        }
    }
}

impl FromStr for Color {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.as_bytes();
        if s.len() != 7 || s[0] != b'#' { return Err(InvalidData)? }
        if !s[1..].iter().all(|&c| b'0' <= c && c <= b'9' || b'a' <= c && c <= b'f') {
            return Err(InvalidData)?
        }
        Ok(Color{r: 0, g: 0, b: 0})  // doesn't matter
    }
}

impl FromStr for EyeColor {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        if !VALID_EYE_COLORS.contains(&s) { return Err(InvalidData)?; }
        Ok(EyeColor::Whatever)  // doesn't matter
    }
}

impl FromStr for PassportId {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let b = s.as_bytes();
        if !(b.len() == 9 && b.iter().all(|&c| b'0' <= c && c <= b'9')) {
            return Err(InvalidData)?
        }
        Ok(PassportId { id: s.to_owned()})
    }
}

#[derive(Debug)]
struct Passport {
    birth_year: BirthYear,
    issue_year: IssueYear,
    expiration_year: ExpirationYear,
    height: Height,
    hair_color: Color,
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
const VALID_KEYS : [&str; 7] = [BYR, IYR, EYR, HGT, HCL, ECL, PID];

impl Passport {
    fn valid_keys(data: &HashMap<String, String>) -> bool {
        VALID_KEYS.iter().all(|&k| data.contains_key(&String::from(k)))
    }

    fn from(data: &HashMap<String, String>) -> Result<Passport> {
        Ok(Passport {
            birth_year: data[BYR].parse()?,
            issue_year: data[IYR].parse()?,
            expiration_year: data[EYR].parse()?,
            height: data[HGT].parse()?,
            hair_color: data[HCL].parse()?,
            eye_color: data[ECL].parse()?,
            passport_id: data[PID].parse()?,
        })
    }
}

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
        Part::Second => {
            let passports : Vec<Passport> = passports.iter().filter_map(|h| Passport::from(h).ok()).collect();
            eprintln!("{:?}", passports);
            println!("{}", passports.len())
        }
    }
}