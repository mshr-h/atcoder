fn main()
{
    let n: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned().parse().unwrap()
    };
    
    match n % 10 {
        2 | 4 | 5 | 7 | 9 => println!("hon"),
        0 | 1 | 6 | 8 => println!("pon"),
        3 => println!("bon"),
        _ => {}
    }
}
