// 2. 运行模块和退出函数
use std::fs::File;
use std::io::{Read, stdin};
use std::{io, process};

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
    let data2 = match File::open("story.txt") {
        Ok(val) => val,
        // Err(error) => panic!("{}", error),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    println!("{:?}", data2);

    /*
    // 5. 用户输入
    println!("Please enter a text");
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(val) => val,
        Err(error) => {
            // panic!("{}", error)
            eprintln!("Something is wrong, {}", error);
            process::exit(1);
        }
    };

    // 注意处理换行字符
    println!("{}", input.trim());

    let mut data3 = match File::open(input.trim()) {
        Ok(val) => val,
        // Err(error) => panic!("{}", error),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    println!("{:?}", &data3);

    // 4. 读取文件内容
    let mut data4_read = String::new();
    let data4 = match data3.read_to_string(&mut data4_read) {
        Ok(val) => val,
        Err(error) => panic!("{}", error),
    };

    println!("{:?}", data4_read);
    */

    // 6. 错误传导
    let data5 = read_file().expect("TODO: panic message");

    println!("{:?}", data5);
}

// 6. 错误传导
fn read_file() -> Result<String, io::Error> {
    // 5. 用户输入
    println!("Please enter a text");
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(val) => val,
        Err(error) => {
            // panic!("{}", error)
            eprintln!("Something is wrong, {}", error);
            return Result::Err(error);
        }
    };

    // 注意处理换行字符
    println!("{}", input.trim());

    let mut data3 = match File::open(input.trim()) {
        Ok(val) => val,
        // Err(error) => panic!("{}", error),
        Err(error) => {
            eprintln!("{}", error);
            return Result::Err(error);
        }
    };

    println!("{:?}", &data3);

    // 4. 读取文件内容
    let mut data4_read = String::new();
    let data4 = match data3.read_to_string(&mut data4_read) {
        Ok(val) => val,
        Err(error) => return Result::Err(error),
    };

    println!("{:?}", data4_read);

    Ok(data4_read)
}
