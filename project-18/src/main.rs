use std::collections::HashMap;
use std::fmt::{Debug, Display}; // 7. 多`trait`绑定

// 1. trait 定义
trait Accommodation {
    // fn get_description(&self) -> String;
    // 3. 默认接入
    /*fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }*/

    fn book(&mut self, name: &str, nights: u32);
}

// 7. 多`trait`绑定
trait Description {
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
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
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
fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
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
fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    second.book(guest, 1);
}

// 9. `trait`函数返回值
fn choose_best_place_to_stay() -> impl Accommodation + Description + Debug{
    Hotel::new("BabaBoy")
}

fn main() {
    // let mut hotel = Hotel::new("BabaBoy");
    // 9. `trait`函数返回值
    let mut hotel = choose_best_place_to_stay();

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

    // 10. `trait`绑定条件方法
    let mut hotel2 = Hotel::new(String::from("Jinitaimei"));

    println!("{:?}", hotel2.get_description());
    println!("{:?}", hotel2.summarize());

    hotel2.book("zzz", 3);
    hotel2.book("www", 1);

    println!("{:?}", hotel2);

    let mut hotel3 = Hotel::new(vec!["GGboy","Nizhenmei"]);

    // 方法会失效，因为vec数组里面没有hotel3的表达方法
    // println!("{:?}", hotel3.get_description());
    // println!("{:?}", hotel3.summarize());

    hotel3.book("zzz", 3);
    hotel3.book("www", 1);

    println!("{:?}", hotel3);
}
