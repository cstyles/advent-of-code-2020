use regex::Regex;

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
);

impl<'a> Passport<'a> {
    fn is_valid(&self) -> bool {
        let necessary_fields = [
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid,
        ];

        necessary_fields.iter().all(|field| field.is_some())
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

fn main() {
    let split_regex = Regex::new(r#"\s"#).unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let valid_passports: usize = input
        .trim()
        .split("\n\n")
        .map(|group| split_regex.split(group))
        .map(|split| split.collect::<Vec<&str>>())
        .map(|fields| Passport::from(fields))
        .filter(|passport| passport.is_valid())
        .count();

    println!("{:?}", valid_passports);
}
