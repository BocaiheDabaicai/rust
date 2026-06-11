use std::collections::HashMap;
use std::fmt::{Debug, Display}; // 7. 多`trait`绑定

// 1. trait 定义
pub trait Accommodation {
    // fn get_description(&self) -> String;
    // 3. 默认接入
    /*fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }*/

    fn book(&mut self, name: &str, nights: u32);
}

// 7. 多`trait`绑定
pub trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay two double.")
    }

    fn summarize(&self) -> String {
        format!("Summary: {}", self.get_description())
    }
}

// 2. 数据结构接入`traits`
#[derive(Debug)]
// 10. `trait`绑定条件方法
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
    // 4. 回调`trait`方法
    /*fn summarize(&self) -> String {
        // 只要语言识别到接入体有 get_description 函数即可成立
        format!("{}: {}", self.name, self.get_description())
    }*/
}

impl<T> Accommodation for Hotel<T> {
    /*fn get_description(&self) -> String {
        format!("Hotel {}", self.name)
    }*/

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}
// 7. 多`trait`绑定
impl<T: Display> Description for Hotel<T> {
    fn get_description(&self) -> String {
        format!("Hotel {}", self.name)
    }

    fn summarize(&self) -> String {
        // 只要语言识别到接入体有 get_description 函数即可成立
        format!("{}: {}", self.name, self.get_description())
    }
}

#[derive(Debug)]
pub struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    /*fn get_description(&self) -> String {
        format!("Enjoy {}", self.host)
    }*/

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

// 7. 多`trait`绑定
impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Enjoy {}", self.host)
    }
}

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