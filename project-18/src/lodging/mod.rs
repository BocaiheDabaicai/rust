use std::collections::HashMap;
use std::fmt::Display;

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