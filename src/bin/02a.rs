use regex::Regex;

fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let re: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]*)$")?;
    let mut valid = 0;
    for line in include_str!("../../inputs/02.txt").lines() {
        let cap = re.captures(&line).unwrap(); // Must match.
        let min: i32 = str::parse(&cap[1])?;
        let max: i32 = str::parse(&cap[2])?;
        let c: u8 = cap[3].as_bytes()[0];
        let pw: &[u8] = &cap[4].as_bytes();
        let mut cnt: i32 = 0;
        for i in pw {
            if *i == c {
                cnt += 1;
            }
        }
        if cnt >= min && cnt <= max {
            valid += 1;
        }
    }
    println!("valid: {}", valid);
    Ok(())
}
