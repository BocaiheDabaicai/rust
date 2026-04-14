fn main() {
    // 1. String复习
    let data1 = "Big Boy";
    let data2 = String::from(data1);
    let data3 = data1.to_string();

    print_string(&data1.to_string());
    print_string(&data2);
    print_string(&data3);

    let data4 = &data2[0..2];

    print_string(&data4.to_string());

    // 2. 字符串合并
    let mut data5 = String::from("Hello");
    let data6 = "World";

    data5.push(' '); // 为字符串添加单个字符
    data5.push_str(data6);
    print_string(&data5);

    let data7 = String::from("BigDog");
    let data8 = data5 + " " + &data7; // 加法原则

    print_string(&data8);

    // 3. 格式化
    let data9 = format!("{} ?=%&^$ {}", data7, data8);

    print_string(&data9);

    //4. 常见的字符串方法
    let data10 = " djsal djqwi cjjcd dkkkk, potrop ohko    ";

    print_string(&data10.trim().to_string()); // trim()，消除字符串两侧的空格
    print_string(&data10.trim_start().to_string()); // trim_start()，消除字符串左侧的空格
    print_string(&data10.trim_end().to_string()); // trim_end()，消除字符串右侧的空格
    print_string(&data10.to_uppercase().to_string()); // to_uppercase()，字符串全大写
    print_string(&data10.to_lowercase().to_string()); // to_lowercase()，字符串全小写
    print_string(&data10.replace("k", "*").to_string()); // replace(a,b)，把字符a全部替换成b

    // 用split()函数做数组分割，并使用collect()收集到向量当中
    let data11: Vec<&str> = data10.trim().split(" ").collect();
    println!("{:?}", data11);
}
fn print_string(data: &String) {
    println!("That result is: {}", data);
}
