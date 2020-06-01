fn main() {
    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let a: i16 = iter.next().unwrap().parse().unwrap();
    let b: i16 = iter.next().unwrap().parse().unwrap();

    println!("{}", a * b);
}
