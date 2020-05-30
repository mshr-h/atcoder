fn main() {
    let k: usize = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned().parse().unwrap()
    };
 
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
 
    if s.chars().count() > k {
        let mut str1 = "".to_string();
        for (i, c) in s.chars().enumerate() {
            if i < k {
                str1.push(c);
            }
        }
        println!("{}...", str1);
    } else {
        println!("{}", s);
    }
}
