// 4. 模块文件
mod inventory;
// 5. 模块文件夹
// 引入文件夹下所有的.rs文件
mod orders;

/*
// 1. 创建模块
mod inventory {
    const FLOOR_SPACE: i32 = 10000; // 必须声明数据类型
    pub const MANAGER: &str = "Liu Inventory"; // 不声明公开性，则默认为私有的

    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    fn talk_to_manager() {
        println!("That is your coffee? Mr.{MANAGER}");
    }
}
*/
/*
// 3. 创建多个模块
mod orders {
    pub const MANAGER: &str = "Bob Steven";
}
*/

fn main() {
    // 2. 公共关键字
    println!("Hello, world! This is my manager {}", inventory::MANAGER);
    println!("Hello, world! This is my orders {}", orders::MANAGER);

    // 6. 模块模糊性
    // 文件夹和文件的名字不能重复，使代码能够准确找到对应的文件或文件夹

    // 7. 公共枚举、数据结构、领域
    // 包括数据结构的每个属性的属性范围都需要指定
    println!(
        "ProductCategory has {:?}",
        inventory::ProductCategory::Hammer
    );
    println!(
        "ProductCategory has {:?}",
        inventory::ProductCategory::Ladder
    );

    let item = inventory::Item {
        name: String::from("pop-wwasd 2033"),
        category: inventory::ProductCategory::Hammer,
        quantity: 28,
    };

    println!("{:#?}",item);
}
