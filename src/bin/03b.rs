fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    println!(
        "mul={}",
        tree_count(1, 1)?
            * tree_count(3, 1)?
            * tree_count(5, 1)?
            * tree_count(7, 1)?
            * tree_count(1, 2)?
    );
    Ok(())
}

fn tree_count(right: usize, down: usize) -> anyhow::Result<usize> {
    let mut xpos: usize = 0;
    let mut ypos: usize = 0;
    let mut trees: usize = 0;
    for line in include_str!("../../inputs/03.txt").lines() {
        if ypos % down != 0 {
            ypos += 1;
            continue;
        }
        let line = line.as_bytes();
        if line[xpos % line.len()] == b'#' {
            trees += 1;
        }
        ypos += 1;
        xpos += right;
    }
    println!("right={} down={} trees = {}", right, down, trees);
    Ok(trees)
}
