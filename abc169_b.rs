use std::fmt::Debug;
use std::str::FromStr;

fn read_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

fn main() {
    let _: i64 = read_line()[0];
    let a: Vec<i64> = read_line();
    for a_n in &a {
        if *a_n == 0 {
            println!("0");
            return;
        }
    }
    let mut result: i64 = 1;
    for a_n in &a {
        if *a_n > 1_000_000_000_000_000_000i64 / result {
            println!("-1");
            return;
        }
        result *= a_n;
    }
    println!("{}", result);
}
