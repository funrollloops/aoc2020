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
        match kv[0] {
            "byr" => valid_fields += 1,
            "iyr" => valid_fields += 1,
            "eyr" => valid_fields += 1,
            "hgt" => valid_fields += 1,
            "hcl" => valid_fields += 1,
            "ecl" => valid_fields += 1,
            "pid" => valid_fields += 1,
            "cid" => valid_fields += 0,
            _ => panic!("bad part: {}", part),
        }
    }
    Ok(valid_fields == 7) // Assuming no dups.
}
