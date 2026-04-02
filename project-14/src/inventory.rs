// 4. 文件模块
// 不需要使用包裹模式编写
// 7. 公共枚举、数据结构、领域
const FLOOR_SPACE: i32 = 10000; // 必须声明数据类型
pub const MANAGER: &str = "Liu Inventory"; // 不声明公开性，则默认为私有的

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

pub fn talk_to_manager() {
    println!("That is your coffee? Mr.{MANAGER}");
}
