// 2. 运行模块和退出函数
use std::fs::File;
use std::io::{Read, stdin};
use std::{fs, io, process};

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
    // let data5 = read_file().expect("TODO: panic message");

    // println!("{:?}", data5);

    // 7. 问号操作符
    // let data6 = read_file2().expect("TODO: panic message");

    // println!("----------Final------------");
    // println!("result is: {:?}", data6);

    // 8. 读字符串函数
    // let data7 = read_file3().expect("TODO: panic message");

    // println!("----------Final------------");
    // println!("result is: {:?}", data7);

    // 9. 用可选结构使用`?`操作符
    let mut data8 = vec!["Geeq", "fi878", "popfg00"];

    println!("result is: {:?}", length_of_last_element(&mut data8));
    println!("result is: {:?}", length_of_last_element(&mut data8));
    println!("result is: {:?}", length_of_last_element(&mut data8));
    println!("result is: {:?}", length_of_last_element(&mut data8));

    // test
    match write_to_file() {
        Ok(val) => println!("写入成功！！"),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
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

// 7. 问号操作符
fn read_file2() -> Result<String, io::Error> {
    // 5. 用户输入
    println!("Please enter a text");
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    // 4. 读取文件内容
    let mut data4_read = String::new();
    File::open(input.trim())?.read_to_string(&mut data4_read)?;

    Ok(data4_read)
}

// 8. 读字符串函数
fn read_file3() -> Result<String, io::Error> {
    // 5. 用户输入
    println!("Please enter a text");
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim())
}

// 9. 用可选结构使用`?`操作符
fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}

// test
fn write_to_file() -> io::Result<String> {
    /*
        创建一个字符串对象
        调用IO对象的输入方法，读取一行的内容，并使用input1进行收集
        调用fs对象的写入方法，提供文件名称、文件内容进行写入
        最后将结果以Result结构的形式传递出去
    */
    println!("What file would you like to write to?");
    let mut input1 = String::new();
    stdin().read_line(&mut input1)?;

    println!("What would you like to write to the file?");
    let mut input2 = String::new();
    stdin().read_line(&mut input2)?;

    fs::write(input1.trim(), input2.trim())?;

    Ok(input1)
}
