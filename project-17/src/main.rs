// 2. 运行模块和退出函数
use std::fs::File;
use std::process;

fn main() {
    // 1. `panic!`处理
    // None.unwrap()   // 生成错误

    // let data1 = 1;
    // panic!("Here is a panic! {}", data1); // panic!() 生成错误

    // 2. 运行模块和退出函数
    // 0 表示成功运行完毕
    println!("successfully exit.");
    // process::exit(0);

    // 3. 标准错误模块
    println!("A new message:");
    eprintln!("Here is a error!"); // 用于输出错误信息

    // 4. 打开文件
    let data2 = match File::open("story1.txt") {
        Ok(val) => val,
        // Err(error) => panic!("{}", error),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    println!("{:?}", data2);
}
