fn main() {
    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let a: f64 = iter.next().unwrap().parse().unwrap();
    let b: f64 = iter.next().unwrap().parse().unwrap();
    let h: f64 = iter.next().unwrap().parse().unwrap();
    let m: f64 = iter.next().unwrap().parse().unwrap();

    let rad: f64 = std::f64::consts::PI * 2.0 * (h / 12.0 + m / 60.0 / 12.0 - m / 60.0);

    let rsq: f64 = (a * a + b * b) - (2.0 * a * b * rad.cos());

    println!("{:.20}", rsq.sqrt());
}
