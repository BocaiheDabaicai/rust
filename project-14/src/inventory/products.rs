use fake::Dummy;
// fake::Dummy
// 为结构预先填入值
// 生成模拟数值
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

// 13. 顶层关键字
impl Item {
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        // 13. 顶层关键字
        // 调用顶层模块方法 inventory.rs
        super::talk_to_manager();
        Item {
            name,
            category,
            quantity,
        }
    }
}
