use std::fmt::Pointer;

fn main() {
    println!("Hello, world!");

    // 将""号进行转义
    let byte_escape = "I'm saying \"Hello\"";
    println!("{}", byte_escape);

    // 分两行打印
    let byte_escape = "I'm saying \n 你好";
    println!("{}", byte_escape);

    // Windows下的换行符
    let byte_escape = "I'm saying \r\n 你好";
    println!("{}", byte_escape);

    // 打印出 \ 本身
    let byte_escape = "I'm saying \\ Ok";
    println!("{}", byte_escape);

    // 强行在字符串后面加个0，与C语言的字符串一致。
    let byte_escape = "I'm saying hello.\0";
    println!("{}", byte_escape);

    // 字符串字面量前面加r，表示不转义
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 这个字面量必须使用r##这种形式，因为我们希望在字符串字面量里面保留""
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果遇到字面量里面有#号的情况，可以在r后面，加任意多的前后配对的#号，
    // 只要能帮助Rust编译器识别就行
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 字节串的类型是字节的数组，而不是字符串了
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    // 可以看看下面这串打印出什么
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    // 字节串与原始字面量结合使用
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    let a = 10u32;             // 局部变量

    fn add_v1(x: u32, a: u32) -> u32 { x + a } // 定义一个内部函数
    let add_v2 = |x: u32| -> u32 { x + a };   // 定义一个闭包

    let result1 = add_v1(20, a);  // 调用函数
    let result2 = add_v2(20);  // 调用闭包
    println!("{}", result2);

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let a = 1;
    let b = a;

    let str1 = String::from("asdb haskdq");
    let str2 = str1;

    println!("{a}");
    println!("{b}");

    // println!("{str1}");
    println!("{str2}");

    let s = "I am a superman.".to_string();

    for i in 1..10 {
        // 在第二次进行循环时，s已经没有所有权
        // let tmp_s = s;
        let tmp_s = s.clone();
        println!("s is {}", tmp_s);
    }

    struct Point {
        x: i64,
        y: i64,
        z: i64
    }
    let tmp_s = Point{
        x:12,
        y:33,
        z:20
    };
    let tmp_s2 = tmp_s;
    // println!("s is {}",tmp_s.x);
    println!("s is {}",tmp_s2.x)
}
