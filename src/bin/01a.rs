fn main() {
    let txt = include_str!("../../inputs/01.txt");
    let data: Vec<i64> = txt.lines().map(|l| str::parse::<i64>(l).unwrap()).collect();
    for i in &data {
        for j in &data {
            if (i != j && i + j == 2020) {
                println!("i={} j={} i*j={}", i, j, i*j);
            }
        }
    }
}
