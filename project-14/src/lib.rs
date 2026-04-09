// 18. 创建依赖包文件
pub mod inventory;
pub mod orders;

pub use inventory::products::{Item, ProductCategory};
pub use inventory::{FLOOR_SPACE as FS, INVENTORY_MANAGER, ORDER_MANAGER, talk_to_manager as ttm};
