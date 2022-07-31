fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let mut pos: usize = 0;
    let mut trees: usize = 0;
    for line in include_str!("../../inputs/03.txt").lines() {
        let line = line.as_bytes();
        if line[pos % line.len()] == b'#' {
            trees += 1;
        }
        pos += 3;
    }
    println!("trees = {}", trees);
    Ok(())
}
