// 19. 创建多个依赖包文件
// 运行指令
// cargo run --bin summary
use project_14::{FS, ORDER_MANAGER, INVENTORY_MANAGER};

// 20. 文档注释
/// 第二个实现接口路径地址
fn main() {
    println!("We have {ORDER_MANAGER}, {INVENTORY_MANAGER} and {FS}");
}