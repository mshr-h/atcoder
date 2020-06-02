fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let a: u64 = iter.next().unwrap().parse().unwrap();
    let mut b_str: String = iter.next().unwrap().parse().unwrap();
    b_str.remove(1);

    let b: u64 = b_str.parse().unwrap();

    let prod = a * b / 100;

    println!("{}", prod);
}
