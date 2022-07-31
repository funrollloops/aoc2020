fn main() {
    let txt = include_str!("../../inputs/01.txt");
    let data: Vec<i64> = txt.lines().map(|l| str::parse::<i64>(l).unwrap()).collect();
    for i in &data {
        for j in &data {
            for k in &data {
                if i < j && j <  k && i + j + k == 2020 {
                    println!("i={} j={} k={} i*j*k={}", i, j, k, i*j*k);
                }
            }
        }
    }
}
