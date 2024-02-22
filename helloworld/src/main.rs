/*fn main() {
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
        z: i64,
    }
    let tmp_s = Point {
        x: 12,
        y: 33,
        z: 20,
    };
    let tmp_s2 = tmp_s;
    // println!("s is {}",tmp_s.x);
    println!("s is {}", tmp_s2.x);

    let mut a1 = 10u32;
    let mut b = &mut a1;
    let mut c = &mut b;
    let d = &mut c;

    ***d = 30;

    println!("{d}");

    // let mut a: u32 = 10;
    // let b = &mut a;
    // *b += 10;
    // let c = &mut a;
    // *c += 10;
    // let d = &mut a;
    //
    // println!("{}", d);
}*/

/*fn main() {
    // let s1: &'static str = "I am a superman.";
    // let s2: String = s1.to_string();
    // let s3: &String = &s2;
    // let s4: &str = &s2[..];
    // let s5: &str = &s2[..6];
    //
    // let s: String = "I am a superman.".to_string();
    // let a_slice: &str = &s[..];
    // let another_String: String = a_slice.to_string();

    // let s: &str = "I am a superman.";
    // let s1: String = String::from(s);  // 使用 String 的from构造器
    // let s2: String = s.to_string();    // 使用 to_string() 方法
    // let s3: String = s.to_owned();     // 使用 to_owned() 方法
    //
    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);

    // String 与 str 类型的相互转换
    // let s: String = "I am a superman.".to_string();
    // let a_slice: &str = &s[..];
    // let another_String: String = a_slice.to_string();

    // let s = String::from("abcdefg");
    // let s1 = &s[..];    // s1 内容是 "abcdefg"
    // let s2 = &s[0..4];  // s2 内容是 "abcd"
    // let s3 = &s[2..5];    // s3 内容是 "cde"

    // let a_vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // let a_slice: &[u8] = &a_vec[0..5]; // a_slice 是 [1,2,3,4,5]
    // let another_vec = a_slice.to_vec(); // 用 .to_vec() 方法将切片转换成Vec
    // let another_vec = a_slice.to_owned(); // 或者用 .to_owned() 方法

    let s = String::from("foo");
    assert_eq!("foo", s.as_str());

    let s = String::from("hello");
    assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());

    let bytes = "bors".as_bytes();
    assert_eq!(b"bors", bytes);

    let a = "10".parse::<u32>();
    let aa: u32 = "10".parse().unwrap(); // 这种写法也很常见
    println!("{:?}", a);

    let a = "10".parse::<f32>();
    println!("{:?}", a);

    let a = "4.2".parse::<f32>();
    println!("{:?}", a);

    let a = "true".parse::<bool>();
    println!("{:?}", a);

    let a = "a".parse::<char>();
    println!("{:?}", a);

    let a = "192.168.1.100".parse::<std::net::IpAddr>();
    println!("{:?}", a);
}*/

/*// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
//
// struct Class {
//     serial_number: u32,
//     grade_number: u32,
//     entry_year: String,
//     members: Vec<User>,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 就像这样去实现
    fn area(self) -> u32 {      // area就是方法，被放在impl实现体中
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()      // 使用点号操作符调用area方法
    );
}*/


#[derive(Debug)]
pub enum Option<T> {
    Some(T),
    None,
}

pub enum Result<T,E>{
    Ok(T),
    Err(E)
}

// fn main() {
//     let s = String::from("");
//     let a: Option<String> = Option::Some(s);
//
//     println!("{:?}",a);
// }

fn main() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");
    let s4 = String::from("ddd");

    let v = vec![s1, s2, s3, s4];
    for s in &v {      // 这里，s拿到了集合元素的所有权
        println!("{}", s);
    }
    println!("{:?}", v);
}