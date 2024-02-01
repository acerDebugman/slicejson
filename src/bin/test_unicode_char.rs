pub fn main() {
    let s = "123.123";
    println!("{:?}", s.char_indices());
    let mut s1 = String::from("123456");
    let s2: &mut str = &mut s1;
    let mut s2ptr = s2.as_mut_ptr();
    unsafe {
        s2ptr = s2ptr.offset(1);
    }
    // println!("{:?}", s2ptr.as_mut());

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

    // let mut ptr = yes.as_mut_ptr();
    // unsafe {
    //     ptr = ptr.offset(1);
    // }
    // let a = unsafe { &*ptr };
    // println!("const string: {}", a);

    let str_content = "Hello, world!";
    let mut byte_content: Vec<u8> = str_content.as_bytes().to_vec();
    // 获取byte_content的指针
    let ptr = byte_content.as_ptr();
    // 假设我们只关心从索引0到索引3的字节
    let offset = 0; // 起始偏移量
                    // 通过指针和偏移量访问内容
    let first_byte = unsafe { *ptr.offset(offset) };
    let second_byte = unsafe { *ptr.offset(offset + 1) };
    println!("First byte: {}, Second byte: {}", first_byte, second_byte);
    println!("First byte: H: {}, Second byte: e: {}", b'H', b'e');

    let mut ptr = byte_content.as_mut_ptr();
    unsafe {
        ptr = ptr.add(1);
    };
    unsafe { *ptr = b'I' };
    let first_b = unsafe { *ptr };
    println!("First byte: {}", first_b);
}
