// 4. 模块文件
// mod inventory;
// 5. 模块文件夹
// 引入文件夹下所有的.rs文件
// mod orders;

// 17. 全局操作符
// use std::collections::*;

// 16. 标准库
/*
// 方式一
use std::fmt;
use std::io;
*/

/*
// 方式二
use std::{
    fmt,
    io::{self, stdin, stdout},
};
*/
// 15. 外部依赖库
use fake::{Fake, Faker};

// 18. 创建依赖包文件
// use project_14::{FS, Item, ProductCategory, inventory, orders, ttm};
use project_14::*;

// 10. 使用关键字一
// 使用之前任然需要引入模块
// use inventory::INVENTORY_MANAGER;
// use inventory::products::ProductCategory;   // 注意不能重复获取

// 11. 使用关键字二
// 取多个模块数据
// use inventory::products::{Item, ProductCategory};

// 12. 自我关键字
// use inventory::{FLOOR_SPACE, talk_to_INVENTORY_MANAGER};

// 13. 给关键字设置别名
// use inventory::products::{Item, ProductCategory};
// use inventory::{FLOOR_SPACE as FS, talk_to_INVENTORY_MANAGER as ttm};

/*
// 1. 创建模块
mod inventory {
    const FLOOR_SPACE: i32 = 10000; // 必须声明数据类型
    pub const INVENTORY_MANAGER: &str = "Liu Inventory"; // 不声明公开性，则默认为私有的

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

    fn talk_to_INVENTORY_MANAGER() {
        println!("That is your coffee? Mr.{INVENTORY_MANAGER}");
    }
}
*/
/*
// 3. 创建多个模块
mod orders {
    pub const INVENTORY_MANAGER: &str = "Bob Steven";
}
*/

// 20. 文档注释
/// 主要的实现接口路径地址 main.rs
fn main() {
    // 2. 公共关键字
    println!("Hello, world! This is my INVENTORY_MANAGER {}", inventory::INVENTORY_MANAGER);
    println!("Hello, world! This is my orders {}", orders::MANAGER);

    // 6. 模块模糊性
    // 文件夹和文件的名字不能重复，使代码能够准确找到对应的文件或文件夹

    // 7. 公共枚举、数据结构、领域
    // 包括数据结构的每个属性的属性范围都需要指定
    println!(
        "ProductCategory has {:?}",
        inventory::products::ProductCategory::Hammer
    );
    println!(
        "ProductCategory has {:?}",
        inventory::products::ProductCategory::Ladder
    );

    let item = inventory::products::Item {
        name: String::from("pop-wwasd 2033"),
        category: inventory::products::ProductCategory::Hammer,
        quantity: 28,
    };

    println!("{:#?}", item);

    // 9. 项目前缀
    // 绝对路径寻找
    println!(
        "Hello, world! This is my INVENTORY_MANAGER {}",
        crate::inventory::INVENTORY_MANAGER
    );
    println!("Hello, world! This is my orders {}", crate::orders::MANAGER);

    // 10.使用关键字一
    let item_config = inventory::products::Item {
        name: String::from("pop-wwasd 2033"),
        category: ProductCategory::Hammer,
        quantity: 28,
    };

    println!("{:#?}", item_config);

    // 13. 顶层关键字
    let item_config_two = Item::new(String::from("bq1-xxzeq 1599"), ProductCategory::Ladder, 56);

    println!("{:#?}", item_config_two);

    // 15. 外部依赖库
    let faker_item: Item = Faker.fake();

    println!("{:?}", faker_item);

    let random_category: ProductCategory = Faker.fake();

    println!("{:?}", random_category);
}
