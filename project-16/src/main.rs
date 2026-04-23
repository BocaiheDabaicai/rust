use std::collections::{HashMap, HashSet};

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

    // 4. 获取键值
    let mut data5_list = HashMap::from([("Bibilabu", "Bababoyi"), ("jinitaimei", "wodedaodun")]);
    let data5 = data5_list
        .get("Bibilabu")
        .copied()
        .unwrap_or("Unknown Milk");

    println!("{:?}", data5);

    // 5.覆盖键值
    data5_list.insert("jinitaimei", "Cadage"); // 对已存在的键进行插入操作，即可实现覆盖效果

    println!("{:?}", data5_list);

    // 6. `entry`方法
    // 防止意外地覆盖和重复添加出现
    data5_list.entry("jinitaimei").or_insert("Cadage123");
    data5_list.entry("Nihaoma").or_insert("Wohenhao");

    println!("{:?}", data5_list);

    // 7. `HashSet`类型
    // 只能存储唯一值的类型
    let mut data6 = HashSet::new();

    data6.insert("Bibilabu");
    data6.insert("Bababoyi");

    println!("{:?}", data6);
    println!("length is: {:?}", data6.len());

    data6.insert("Bibilabu"); // 该插入操作将失败，因为值已经存在
    data6.insert("Wodedaodun");

    println!("{:?}", data6);
    println!("length is: {:?}", data6.len());

    data6.remove("Wodedaodun1"); // 没有找到值，则删除操作失败
    data6.remove("Wodedaodun");

    println!("{:?}", data6);
    println!("length is: {:?}", data6.len());

    println!(
        "data6.contains(\"Bibilabu\"): {:?}",
        data6.contains("Bibilabu")
    );
    println!(
        "data6.contains(\"Bibilabu123\"): {:?}",
        data6.contains("Bibilabu123")
    );

    println!("data6.get(\"Bababoyi\"): {:?}", data6.get("Bababoyi"));
    println!("data6.get(\"Bababoyi123\"): {:?}", data6.get("Bababoyi123"));

    // 8. `HashSet`方法
    let mut data7: HashSet<&str> = HashSet::new();
    let mut data8: HashSet<&str> = HashSet::new();

    data7.insert("asd1");
    data7.insert("asd2");

    data8.insert("rtye1");
    data8.insert("rtye2");
    data8.insert("asd2");

    // 结合方法
    // 把两者的值结合起来，排除重复值，并形成数组
    println!("{:?}", data7.union(&data8));
    println!("{:?}", data8.union(&data7));

    // 显示不同方法，找出自身相比于另一个HashSet独特的值
    // 找出自己唯一的值，并形成数组
    println!("{:?}", data7.difference(&data8));
    println!("{:?}", data8.difference(&data7));

    // 找出两者唯一的值，并形成数组
    println!("{:?}", data7.symmetric_difference(&data8));
    println!("{:?}", data8.symmetric_difference(&data7));

    // 判断两者是否完全没有相同的值
    println!("{:?}", data7.is_disjoint(&data8));
    println!("{:?}", data8.is_disjoint(&data7));

    // 是否自己的集合包含于另一个集合
    println!("{:?}", data7.is_subset(&data8));
    println!("{:?}", data8.is_subset(&data7));

    // 是否自己的集合包含另一个集合
    println!("{:?}", data7.is_superset(&data8));
    println!("{:?}", data8.is_superset(&data7));

    // 9. test
    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from([
        (
            "Ketchup",
            Vec::from(["French Fries", "Burgers", "Hot Dogs"]),
        ),
        (
            "Mayonnaise",
            Vec::from(["Sandwiches", "Burgers", "Coleslaw"]),
        ),
    ]);

    sauces_to_meals.insert("Mustard", Vec::from(["Hot dog", "Burgers", "Pretzels"]));

    let data9 = sauces_to_meals.remove("Mayonnaise");
    println!("{:?}", data9.unwrap());

    let data10 = sauces_to_meals.get("Mustard");
    println!("{:?}", data10.unwrap());

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(Vec::from(["Sushi", "Dumplings"]));

    println!("{:#?}",sauces_to_meals);
}
