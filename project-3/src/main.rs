#[allow(unused_variables)]
fn main() {
    let data1:i8 = 127;
    let data2:u8 = 255;

    let data3:i16 = -30000;
    let data4:i16 = 30000;

    let data5 = 25i8;
    let data6 = 25u8;

    let data7 = 25_32_1;    // 下划线表示，最终的值会消除下划线
    println!("{data7}");

    // 根据计算机操作系统的位，来分配变量的空间大小，方便跨系统使用变量
    let data8:isize = 25;
    let data9:usize = 25;
}
