// 5. `trait`方法参数约束
// 6. `trait`绑定语法
/*fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    println!(
        "book_for_one_night: {}. Name: {}",
        entity.get_description(),
        guest
    );
    entity.book(guest, 1);
}*/
use std::fmt::Debug;

// 12. 项目化代码（多模块）
// use super::lodging::{Accommodation,Description,Hotel};
use crate::lodging::{Accommodation,Description,Hotel};

// 7. 多`trait`绑定
pub fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    println!(
        "book_for_one_night: {}. Name: {}",
        entity.get_description(),
        guest
    );
    entity.book(guest, 1);
}
// 6. `trait`绑定语法
/*fn mix_and_match<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}*/

// 7. 多`trait`绑定
/*fn mix_and_match<T: Accommodation + Description, U: Accommodation>(
    first: &mut T,
    second: &mut U,
    guest: &str,
) {
    first.book(guest, 1);
    second.book(guest, 1);
}*/

// 8. `where`语句
pub fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    second.book(guest, 1);
}

// 9. `trait`函数返回值
pub fn choose_best_place_to_stay() -> impl Accommodation + Description + Debug {
    Hotel::new("BabaBoy")
}