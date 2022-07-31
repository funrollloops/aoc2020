fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let mut ids: Vec<i16> = include_str!("../../inputs/05.txt").lines().map(|l| {
        let d = decode(l).unwrap();
        d.0 * 8 + d.1
    }).collect();
    ids.sort();
    for i in 0..(ids.len() - 1) {
        if ids[i] + 2 == ids[i + 1] {
            println!("missing id={}", ids[i] + 1);
        }
    }
    Ok(())
}

fn decode(s: &str) -> anyhow::Result<(i16, i16)> {
    assert!(s.len() == 10);
    let s = s.as_bytes();
    let mut row: i16 = 0;
    for i in 0..7 {
        row <<= 1;
        if s[i] == b'B' {
            row |= 1;
        }
    }
    let mut col: i16 = 0;
    for i in 0..3 {
        col <<= 1;
        if s[7 + i] == b'R' {
            col |= 1;
        }
    }
    Ok((row, col))
}
