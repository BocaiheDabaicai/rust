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
}
