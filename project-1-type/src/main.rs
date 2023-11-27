fn main() {

    // 1.明确数据类型
    // println!("{}",(-4).abs());
    // println!("{}",(-4_i32).abs());
    // println!("{}",i32::abs(-4));

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
    let (head,tail) = text.split_at(21);
    println!("{} \n {}",head,tail);
}
