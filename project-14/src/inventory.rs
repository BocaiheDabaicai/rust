pub mod products;
// 14. 使用模块中的具体内容
pub use products::{Item, ProductCategory};
// 4. 文件模块
// 不需要使用包裹模式编写
// 7. 公共枚举、数据结构、领域
pub const FLOOR_SPACE: i32 = 10000; // 必须声明数据类型
pub const MANAGER: &str = "Liu Inventory"; // 不声明公开性，则默认为私有的

pub fn talk_to_manager() {
    // println!("That is your coffee? Mr.{MANAGER}");
    // 9. 项目前缀
    println!(
        "That is your coffee? Mr.{}, your type is {:?}",
        crate::inventory::MANAGER,
        ProductCategory::Ladder
    );
}

// inventory/products.rs
// inventory/products/mod.rs
/*
    是 inventory.rs 在引用 inventory 目录下的子集模块
*/

// 8. 子级模块
/*mod products {
    #[derive(Debug)]
    pub enum ProductCategory {
        Ladder,
        Hammer,
    }

    #[derive(Debug)]
    pub struct Item {
        pub name: String,
        pub category: ProductCategory,
        pub quantity: u32,
    }
}*/
