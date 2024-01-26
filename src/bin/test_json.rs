pub fn main() {
    let s = "123.123";
    println!("{:?}", s.char_indices());
    let yes = "y̆es";
    let subs = &yes[1..3];
    println!("{}, len: {}", yes, yes.len());
    println!("subs: {:?}", subs);

    let c: char = yes.chars().nth(0).unwrap(); //char类型是4字节!
    let a = &c; //char 无法转回 &str
    println!("{:?}", yes.chars().nth(0));
    let mut peak = yes.char_indices().peekable();
    let a = peak.peek();
    peak.nth_back(1);
    let mut indices = yes.char_indices();
    println!("{:?}", indices.next());
    println!("{:?}", indices.next());
    println!("{:?}", indices.next());
    println!("a{}b", indices.next().unwrap().1);
}
