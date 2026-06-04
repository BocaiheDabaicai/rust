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

fn main() {
    let mut hotel = Hotel::new("BabaBoy");

    println!("{:?}", hotel.get_description());

    hotel.book("John", 3);
    hotel.book("Bob", 1);

    println!("{:?}", hotel);

    let mut airbnb = AirBnB::new("Bibilabu");

    println!("{:?}", airbnb.get_description());

    airbnb.book("John", 3);
    airbnb.book("Bob", 1);
    airbnb.book("sailuo", 12);

    println!("{:?}", airbnb);
}
