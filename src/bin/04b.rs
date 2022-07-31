#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let mut valid = 0;
    for data in include_str!("../../inputs/04.txt").split("\n\n") {
        if parse(data)? {
            valid += 1;
        }
    }
    println!("valid={}", valid);
    Ok(())
}

fn parse(data: &str) -> anyhow::Result<bool> {
    let mut valid_fields = 0;
    for part in data.split_whitespace() {
        let kv: Vec<&str> = part.split(":").collect();
        assert!(kv.len() == 2);
        valid_fields += if is_field_valid(kv[0], kv[1])? { 1 } else { 0 };
    }
    Ok(valid_fields == 7) // Assuming no dups.
}

fn as_int(value: &str) -> u16 {
    str::parse::<u16>(value).unwrap_or(0)
}

fn is_field_valid(key: &str, value: &str) -> anyhow::Result<bool> {
    lazy_static! {
        static ref hcl_re: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    }
    Ok(match key {
        "byr" => (1920..=2002).contains(&as_int(value)),
        "iyr" => (2010..=2020).contains(&as_int(value)),
        "eyr" => (2020..=2030).contains(&as_int(value)),
        "hgt" => {
            if let Some(cm) = value.strip_suffix("cm") {
                (150..=193).contains(&as_int(cm))
            } else if let Some(inches) = value.strip_suffix("in") {
                (59..=76).contains(&as_int(inches))
            } else {
                false
            }
        },
        "hcl" => hcl_re.is_match(value),
        "ecl" => value == "amb" || value == "blu" || value == "brn" || value == "gry" || value == "grn" || value == "hzl" || value == "oth",
        "pid" => value.len() == 9 && str::parse::<u64>(value).is_ok(),
        "cid" => false,
        _ => panic!("bad part: {}:{}", key, value),
    })
}
