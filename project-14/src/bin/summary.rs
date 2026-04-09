// 19. 创建多个依赖包文件
// 运行指令
// cargo run --bin summary
use project_14::{FS, ORDER_MANAGER, INVENTORY_MANAGER};

fn main() {
    println!("We have {ORDER_MANAGER}, {INVENTORY_MANAGER} and {FS}");
}