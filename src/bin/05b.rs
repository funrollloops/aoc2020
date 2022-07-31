fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let mut ids = include_str!("../../inputs/05.txt")
        .lines()
        .map(decode)
        .collect::<anyhow::Result<Vec<_>>>()?;
    ids.sort();
    for i in 0..(ids.len() - 1) {
        if ids[i] + 2 == ids[i + 1] {
            println!("missing id={}", ids[i] + 1);
        }
    }
    Ok(())
}

fn decode(s: &str) -> anyhow::Result<i16> {
    assert!(s.len() == 10);
    let s = s.as_bytes();
    let mut id: i16 = 0;
    for i in 0..7 {
        id <<= 1;
        if s[i] == b'B' {
            id |= 1;
        }
    }
    for i in 0..3 {
        id <<= 1;
        if s[7 + i] == b'R' {
            id |= 1;
        }
    }
    Ok(id)
}
