use std::collections::HashMap;

fn main() {
    // 1. 创建`HashMap`
    let mut data1: HashMap<String, f64> = HashMap::new();

    data1.insert("apple".to_string(), 0.5);
    data1.insert("pear".to_string(), 1.52);
    data1.insert("orange".to_string(), 0.89);

    println!("{:?}", data1);

    let mut data2: HashMap<&str, &str> = HashMap::new();

    data2.insert("bibilabu", "bababoyi");
    data2.insert("wodedaodun", "jinitaimei");
    data2.insert("funiga", "ya~~ya");

    println!("{:?}", data2);

    // 2. 移除`HashMap`
    let mut data3 = HashMap::from([("Booby", 7), ("Grant", 4), ("Ben", 6)]);

    println!("{:?}", data3);

    let item = data3.remove("Ben");

    println!("remove item: {:?}", item);
    println!("{:?}", data3);

    let item = data3.remove("Ben"); // 重复删除
    println!("remove item: {:?}", item);
    println!("{:?}", data3);

    // 3. `HashMap`所有权
    let mut data4: HashMap<String, String> = HashMap::new();
    let string1 = String::from("Bibilabu");
    let string2 = String::from("Bababoyi");

    data4.insert(string1.to_string(), string2.to_string());
    data4.insert("jinitaimei".to_string(), "wodedaodun".to_string());

    println!("{:?}", data4);
    println!("{string1} {string2}");
}
