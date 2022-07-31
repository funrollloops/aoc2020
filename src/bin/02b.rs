use regex::Regex;

fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let re: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]*)$")?;
    let mut valid = 0;
    for line in include_str!("../../inputs/02.txt").lines() {
        let cap = re.captures(&line).unwrap(); // Must match.
        let first: usize = str::parse(&cap[1])?;
        let second: usize = str::parse(&cap[2])?;
        let c: u8 = cap[3].as_bytes()[0];
        let pw: &[u8] = &cap[4].as_bytes();
        if (pw[first - 1] == c) != (pw[second - 1] == c) {
            valid += 1;
        }
    }
    println!("valid: {}", valid);
    Ok(())
}
