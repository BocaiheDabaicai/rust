use std::collections::HashMap;

// 1. trait 定义
trait Accommodation {
    // fn get_description(&self) -> String;
    // 3. 默认接入
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }

    fn book(&mut self, name: &str, nights: u32);
}

// 2. 数据结构接入`traits`
#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
    // 4. 回调`trait`方法
    fn summarize(&self) -> String {
        // 只要语言识别到接入体有 get_description 函数即可成立
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("Hotel {}", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn get_description(&self) -> String {
        format!("Enjoy {}", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

// 5. `trait`方法参数约束
// 6. `trait`绑定语法
fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    println!(
        "book_for_one_night: {}. Name: {}",
        entity.get_description(),
        guest
    );
    entity.book(guest, 1);
}
// 6. `trait`绑定语法
fn mix_and_match<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("BabaBoy");

    println!("{:?}", hotel.get_description());
    // 4. 回调`trait`方法
    println!("{:?}", hotel.summarize());

    hotel.book("John", 3);
    hotel.book("Bob", 1);

    println!("{:?}", hotel);

    let mut airbnb = AirBnB::new("Bibilabu");

    println!("{:?}", airbnb.get_description());

    airbnb.book("John", 3);
    airbnb.book("Bob", 1);
    airbnb.book("sailuo", 12);

    println!("{:?}", airbnb);

    book_for_one_night(&mut hotel, "Wodedaodun");
    book_for_one_night(&mut airbnb, "Auligei");

    println!("{:?}", hotel);
    println!("{:?}", airbnb);

    mix_and_match(&mut hotel, &mut airbnb, "Souyougen");
    println!("{:?}", hotel);
    println!("{:?}", airbnb);
}
