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

fn main() {
    // 2. 公共关键字
    println!("Hello, world! This is my manager {}", inventory::MANAGER);
}
