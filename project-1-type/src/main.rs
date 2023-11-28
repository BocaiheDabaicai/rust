fn main() {
    /*    // 1.明确数据类型
        println!("{}",(-4).abs());
        println!("{}",(-4_i32).abs());
        println!("{}",i32::abs(-4));

        // 2. 检查运算,回绕运算,饱和运算,溢出运算
        assert_eq!(10_u8.checked_add(20), Some(30));    // 相加结果是否等于30
        assert_eq!(100_u8.checked_add(200), None);  // 300 超过了类型范围上限

        assert_eq!((100_u16.wrapping_mul(200)), 20000); // 相乘得到的结果是否在范围之内
        assert_eq!((500_u16.wrapping_mul(500)), 53392);
        assert_eq!(5_i16.wrapping_shl(17), 10);  // 移动17位，数据范围有16位，实际移动一位，得到十位数

        assert_eq!(32760_i16.saturating_add(10), 32767);  // 得到紧贴于范围上限的值
        assert_eq!((-32760_i16).saturating_sub(10), -32768);

        assert_eq!(255_u8.overflowing_sub(2), (253, false));  // 运算结果是否溢出
        assert_eq!(255_u8.overflowing_add(2), (1, true));

        // 3. 浮点类型
        assert!((-1. / f32::INFINITY).is_sign_negative());
        assert_eq!(-f32::MIN, f32::MAX);
        assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
        assert_eq!((-1.01f64).floor(), -2.0);
        println!("{}", (2.0_f64).sqrt());
        println!("{}", f64::sqrt(2.0));

        // 4. 布尔类型
        assert_eq!(false as i32, 0);
        assert_eq!(true as i32, 1);

        // 5. 字符类型
        assert_eq!('*' as i32, 42);
        assert_eq!('\u{CA0}' as u16, 0xca0);
        assert_eq!('\u{CA0}' as i8, -0x60);
        assert_eq!('*'.is_alphabetic(), false);

        // 6. 元组类型
        let text = "I see the eigenvalue in thine eye.";
        let (head, tail) = text.split_at(21);
        println!("{} \n {}", head, tail);*/
    /*
    // 7. 数组类型
    // 数组
    let lazy_caterer: [u32; 6] = [1, 2, 3, 4, 5, 6];
    let taxonomy = ["Animalia", "Arthropod", "Insect"];

    println!("{:?}", lazy_caterer);
    println!("{:?}", taxonomy);

    let mut sieve = [true; 10000];

    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    // println!("{:?}",sieve.split_at(10));
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 2, 1];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
    // 向量
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
        vec![0; rows * cols]
    }

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");

    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    println!("capacity is now {}", v.capacity());

    let mut v = vec![10, 20, 30, 40, 50];

    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);

    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
                 if l.len() % 2 == 0 { "functional" } else { "imperative" });
    }

    // 切片
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    fn print(n:&[f64]){
        for elt in n {
            print!("{} ",elt);
        }
        println!();
    }

    print(&v);
    print(&a);

    print(&v[0..2]);
    print(&a[2..]);
    print(&sv[1..3]);
*/
}
