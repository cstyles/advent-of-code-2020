use regex::Regex;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Default)]
struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

lazy_static::lazy_static!(
    static ref FIELD_REGEX: Regex = {
        Regex::new(r#"([^:]+):(.*)"#).unwrap()
    };

    static ref HGT_REGEX: Regex = {
        Regex::new(r#"^(\d+)(in|cm)$"#).unwrap()
    };

    static ref HCL_REGEX: Regex = {
        Regex::new(r#"^#[0-9a-f]{6}$"#).unwrap()
    };

    static ref ECL_REGEX: Regex = {
        Regex::new(r#"^(amb|blu|brn|gry|grn|hzl|oth)$"#).unwrap()
    };

    static ref PID_REGEX: Regex = {
        Regex::new(r#"^\d{9}$"#).unwrap()
    };
);

impl<'a> Passport<'a> {
    fn is_valid_part_1(&self) -> bool {
        let necessary_fields = [
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid,
        ];

        necessary_fields.iter().all(|field| field.is_some())
    }

    fn is_valid_part_2(&self) -> Option<bool> {
        let necessary_fields = vec![
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid,
        ];

        let wow = necessary_fields
            .into_iter()
            .collect::<Option<Vec<&str>>>()?;

        let byr: i32 = wow[0].parse().ok()?;
        if byr < 1920 || byr > 2002 {
            return Some(false);
        }

        let iyr: i32 = wow[1].parse().ok()?;
        if iyr < 2010 || iyr > 2020 {
            return Some(false);
        }

        let eyr: i32 = wow[2].parse().ok()?;
        if eyr < 2020 || eyr > 2030 {
            return Some(false);
        }

        let hgt: Height = wow[3].try_into().ok()?;
        if !hgt.is_valid() {
            return Some(false);
        }

        if !HCL_REGEX.is_match(wow[4]) {
            return Some(false);
        }

        if !ECL_REGEX.is_match(wow[5]) {
            return Some(false);
        }

        if !PID_REGEX.is_match(wow[6]) {
            return Some(false);
        }

        Some(true)
    }
}

impl<'a> std::convert::From<Vec<&'a str>> for Passport<'a> {
    fn from(fields: Vec<&'a str>) -> Self {
        let mut passport: Passport<'a> = Default::default();

        for field in fields {
            let captures = FIELD_REGEX.captures(field).unwrap();
            let field_name = captures.get(1).expect("no field_name").as_str();
            let field_val = captures.get(2).expect("no field_val").as_str();

            match field_name {
                "byr" => passport.byr = Some(field_val),
                "iyr" => passport.iyr = Some(field_val),
                "eyr" => passport.eyr = Some(field_val),
                "hgt" => passport.hgt = Some(field_val),
                "hcl" => passport.hcl = Some(field_val),
                "ecl" => passport.ecl = Some(field_val),
                "pid" => passport.pid = Some(field_val),
                "cid" => passport.cid = Some(field_val),
                _ => {
                    eprintln!("not a valid field_name: {}", field_name);
                    panic!();
                }
            }
        }

        passport
    }
}

enum Unit {
    Inch,
    Centimeter,
}

impl TryFrom<&str> for Unit {
    type Error = ();

    fn try_from(string: &str) -> std::result::Result<Self, Self::Error> {
        match string {
            "in" => Ok(Self::Inch),
            "cm" => Ok(Self::Centimeter),
            _ => Err(()),
        }
    }
}

struct Height {
    unit: Unit,
    value: i64,
}

impl TryFrom<&str> for Height {
    type Error = ();

    fn try_from(string: &str) -> std::result::Result<Self, Self::Error> {
        let captures = HGT_REGEX.captures(string).ok_or(())?;

        let value: i64 = captures
            .get(1)
            .expect("no value")
            .as_str()
            .parse()
            .map_err(|_| ())?;

        let unit: Unit = captures.get(2).expect("no unit").as_str().try_into()?;

        Ok(Height { unit, value })
    }
}

impl Height {
    fn is_valid(&self) -> bool {
        match self.unit {
            Unit::Inch => self.value >= 59 && self.value <= 76,
            Unit::Centimeter => self.value >= 150 && self.value <= 193,
        }
    }
}

fn main() {
    let split_regex = Regex::new(r#"\s"#).unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let passports: Vec<Passport> = input
        .trim()
        .split("\n\n")
        .map(|group| split_regex.split(group))
        .map(|split| split.collect::<Vec<&str>>())
        .map(|fields| Passport::from(fields))
        .collect();

    part1(&passports);
    println!();
    part2(&passports);
}

fn part1(passports: &[Passport]) {
    println!("== PART 1 ==");

    let valid_passports = passports
        .iter()
        .filter(|passport| passport.is_valid_part_1())
        .count();

    println!("{}", valid_passports);
}

fn part2(passports: &[Passport]) {
    println!("== PART 2 ==");

    let valid_passports = passports
        .iter()
        .filter(|passport| passport.is_valid_part_2().unwrap_or(false))
        .count();

    println!("{}", valid_passports);
}
