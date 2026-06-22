/*
// 12. 项目化代码
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
fn choose_best_place_to_stay() -> impl Accommodation + Description + Debug {
    Hotel::new("BabaBoy")
}*/

/*use project_18::{
    Accommodation, AirBnB, Description, Hotel, book_for_one_night, choose_best_place_to_stay,
    mix_and_match,
};*/
use std::cmp::Ordering;
// 22. 接入`drop`特征
use std::fs;
use std::ops::Drop;

// 18.  接入`display`特征
use std::fmt::{Debug, Display, Formatter, Result};
// 12. 项目化代码（多模块）
use project_18::lodging::{Accommodation, AirBnB, Description, Hotel};
use project_18::utils::{book_for_one_night, choose_best_place_to_stay, mix_and_match};

// 16. `super`特征
// 17.  特征类型
trait Investment<T> {
    fn amount(&self) -> T;
    // fn set_amount(&mut self, new_amount: f64);

    /*fn double_amount(&mut self) {
        // 17.  特征类型
        self.set_amount(self.amount() * 2.0)
    }*/

    // 17.  特征类型
    fn double_amount(&mut self);
}

// 16. `super`特征
// 13. 关联约束
trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.21;

    // 14. `getter`方法
    // fn tax_bill(&self) -> f64;

    // 16. `super`特征
    /*fn amount(&self) -> f64;

    // 15. `setter`方法
    // 使用 double_amount()
    // ----  外部调用方法 获取值
    // 结构中设置值
    // 通过 amount() 调用值，然后 * 2.0
    // 走 set_amount() ，设置结构的 amount 值
    // ** 总结，通过：调用结构的amount，使用特征中的set_amount方法，改变结构的amount
    fn set_amount(&mut self, new_amount: f64);

    fn double_amount(&mut self) {
        // self.amount = self.amount() * 2.0;
        self.set_amount(self.amount() * 2.0)
    }*/

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    /*fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }*/

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }

    /*fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }*/
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    amount: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.amount
    }

    /*fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }*/

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;

    /*fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }*/
    /*fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }*/
}

#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}

impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    /*fn set_amount(&mut self, new_amount: u32) {
        self.minutes = new_amount;
    }*/

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

// 20. 接入`Debug`特征
// 19. 在枚举中接入`display`特征
enum AppleTpe {
    RefDelicious,
    GrannySmith,
}

impl Display for AppleTpe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleTpe::RefDelicious => write!(f, "🥂 非常好吃！good job!"),
            AppleTpe::GrannySmith => write!(f, "🥠 美味可口！ Blaster!"),
        }
    }
}
impl Debug for AppleTpe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleTpe::RefDelicious => write!(f, "AppleTpe::RefDelicious"),
            AppleTpe::GrannySmith => write!(f, "AppleTpe::GrannySmith"),
        }
    }
}

// 20. 接入`Debug`特征
// 18.  接入`display`特征
struct Apple {
    kind: AppleTpe,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} 🍎  for 🍎  price is: {}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // write!(f, "Apple::[ Kind: {}, Price: {} ]", self.kind, self.price)
        // 21. 格式化方法
        f.debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("Price", &self.price)
            .finish()
    }
}

// 22. 接入`drop`特征
/*
    使用前，记得在项目根目录下创建一个"apple.txt"文件
*/
impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Apple is being cleaned up."),
            Err(err) => eprintln!("{}", err),
        }
    }
}

// 23. 接入`clone`特征
#[derive(Debug, Clone)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}
/*
// 等同于 #[derive(Debug)] 的效果
impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self {
            doctor: self.doctor.clone(), // 开辟一个新空间存储数据
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}
*/

// 24. 接入`copy`特征
#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Copy for Duration {}

// 25. 接入`partialEq`特征
// 28. 接入`Eq`特征
#[derive(PartialEq, Eq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

// 等同于 #[derive(PartialEq)]
/*impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}*/

// 26. 定义不同类型的比较
struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

// 实现 Flight 能够与 BusTrip 进行比较
// 实现 BusTrip 能够与 Flight 进行比较，并未实现
// 实现 BusTrip 能够与 BusTrip 进行比较，并未实现
// 如果两者的结构体相同，那么可以采用 #[derive(PartialEq)]，放在结构体下面
impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
}

// 27. 为枚举接入`partialEq`特征
// #[derive(PartialEq,Eq)]
enum Musician {
    SingerSoneWriter(String),
    Band(u32),
}

use Musician::{Band, SingerSoneWriter};

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSoneWriter(name) => match other {
                SingerSoneWriter(other_name) => name == other_name,
                Band(_) => false,
            },
            Band(name) => match other {
                SingerSoneWriter(_) => false,
                Band(other_name) => name == other_name,
            },
        }
    }
}

// 29. 接入`partialOld`特征
struct Job {
    salary: u32,
    commute_time: u32,
}

impl Job {
    fn new(salary: u32, commute_time: u32) -> Self {
        Self {
            salary,
            commute_time,
        }
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 最终版本
        self.salary.partial_cmp(&other.salary)

        // 优化版本
        /*match self.salary.partial_cmp(&other.salary) {
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Greater) => Some(Ordering::Greater),
            None => None
        }*/

        // 等同的比较效果
        /*if self.salary == other.salary {
            Some(Ordering::Equal)
        } else if self.salary < other.salary {
            Some(Ordering::Less)
        } else if self.salary > other.salary {
            Some(Ordering::Greater)
        } else {
            None
        }*/
    }
}

// 30. 关联类型一
use std::ops::Add;

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

impl Lunch {
    fn new(cost: f64) -> Self {
        Self { cost }
    }
}

impl Add for Lunch {
    // f64版本
    /*type Output = f64;

    fn add(self, rhs: Self) -> Self::Output {
        self.cost + rhs.cost
    }*/

    // Lunch版本
    type Output = Lunch;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

// 31. 关联类型二
fn add_two_number<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 32. test
// use std::fmt::{Debug,Display,Formatter};

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data());
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("COFFEE")
            .field("kind", &self.kind)
            .field("Milk", &self.milk)
            .field("Ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0
    }

    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}", self.ounces, self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0
    }

    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "** {} Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}

fn main() {
    // let mut hotel = Hotel::new("BabaBoy");
    // 9. `trait`函数返回值
    let mut hotel = choose_best_place_to_stay(); // 存在写法 project_18::utils::choose_best_place_to_stay()

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

    let mut hotel3 = Hotel::new(vec!["GGboy", "Nizhenmei"]);

    // 方法会失效，因为vec数组里面没有hotel3的表达方法
    // println!("{:?}", hotel3.get_description());
    // println!("{:?}", hotel3.summarize());

    hotel3.book("zzz", 3);
    hotel3.book("www", 1);

    println!("{:?}", hotel3);

    // 11. `trait`对象
    /*let stays: Vec<&dyn Description> = vec![&hotel2, &airbnb];

    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());*/

    let mut stays2: Vec<&mut dyn Accommodation> = vec![&mut hotel2, &mut airbnb];

    stays2[0].book("Labby", 2);
    stays2[1].book("Labby", 2);

    println!("{:?}", hotel2);
    println!("{:?}", airbnb);

    // 13. 关联约束
    let income = Income { amount: 200.21 };

    println!("your bill tax is: {:.2}", income.tax_bill());

    let mut bonus = Bonus { amount: 105.44 };

    println!("your bill bonus is: {:.2}", bonus.tax_bill());

    // 14. `getter`方法

    bonus.double_amount();

    println!(
        "After bonus.double_amount(). Your bill bonus is: {:.2}",
        bonus.tax_bill()
    );

    // 16. `super`特征
    let mut quality_time = QualityTime { minutes: 120 };

    println!("Time is:{:?}", quality_time);

    quality_time.double_amount();

    println!("Time is:{:?}", quality_time);

    // 18.  接入`display`特征
    // 20. 接入`Debug`特征
    // 22. 接入`drop`特征
    let apple = Apple {
        kind: AppleTpe::GrannySmith,
        price: 1.21,
    };

    let apple1 = Apple {
        kind: AppleTpe::RefDelicious,
        price: 3.27,
    };

    println!("{}", apple);
    println!("{}", apple1);

    println!("{:?}", apple);
    println!("{:?}", apple1);

    // 23. 接入`clone`特征
    let app1 = Appointment::new("Dr. Andrews", "20260613", "20260616");
    let app2 = app1.clone();

    println!("{:?}", app1);
    println!("{:?}", app2);

    // 24. 接入`copy`特征
    let dura1 = Duration::new(2, 24, 33);
    let dura2 = dura1;

    println!("The duration result is: {:?}", dura1);
    println!("The duration result is: {:?}", dura2);

    // 25. 接入`partialEq`特征
    // 26. 定义不同类型的比较
    let equal1 = Flight::new("Beijing", "Shanghai", "8:21");
    let equal2 = Flight::new("Beijing", "Shanghai", "8:21");
    let equal3 = Flight::new("Chengdu", "Shanghai", "16:22");

    println!("equal1 == equal2, result is: {}", equal1 == equal2);
    println!("equal1 == equal3, result is: {}", equal1 == equal3);
    println!("equal1.eq(&equal2), result is: {}", equal1.eq(&equal2));
    println!("equal1.ne(&equal3), result is: {}", equal1.ne(&equal3));

    let equal4 = BusTrip::new("Chengdu", "Shanghai", "16:22");
    let equal5 = BusTrip::new("Chengdu", "Shanghai", "18:52");

    println!("equal3.eq(&equal4), result is: {}", equal3.eq(&equal4));
    println!("equal3.ne(&equal4), result is: {}", equal3.ne(&equal4));
    println!("equal3.eq(&equal5), result is: {}", equal3.eq(&equal5));
    println!("equal3.ne(&equal5), result is: {}", equal3.ne(&equal5));

    // 27. 为枚举接入`partialEq`特征
    let music1 = SingerSoneWriter("Bubuster".to_string());
    let music2 = SingerSoneWriter("Bubuster".to_string());
    let music3 = SingerSoneWriter("ppoomomo".to_string());
    let band1 = Band(2);
    let band2 = Band(3);
    let band3 = Band(2);

    println!("music1 == music2 : {}", music1 == music2);
    println!("music1 == music3 : {}", music1 == music3);
    println!("band1 == band2 : {}", band1 == band2);
    println!("band1 == band3 : {}", band1 == band3);

    // 29. 接入`partialOld`特征
    let job1 = Job::new(25, 2);
    let job2 = Job::new(33, 5);
    let job3 = Job::new(10, 1);
    let job4 = Job::new(25, 1);

    println!("{}", job1 > job2);
    println!("{}", job1 > job3);
    println!("{}", job1 > job4);

    // 30. 关联类型一
    let cost1 = Lunch::new(24.11);
    let cost2 = Lunch::new(7.89);

    println!(
        "{} + {} = {:?}",
        cost1.cost.clone(),
        cost2.cost.clone(),
        cost1 + cost2
    );

    // 31. 关联类型二
    let cost3 = Lunch::new(24.11);
    let cost4 = Lunch::new(7.89);

    println!("Result is: {}", add_two_number(2, 3));
    println!("Result is: {}", add_two_number(2.15, 3.31));
    println!("Result is: {}", add_two_number(-2, 3));
    println!("Result is: {:?}", add_two_number(cost3, cost4));

    // 32. test
    let mut latte = Coffee::new("Bibilabo", Milk::Whole, 13);

    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new("Cappuccino", Milk::Almond, 24);

    println!("{}", cappuccino.get_data());

    let pepsi = Soda::new(127, 21.58, "Cherry".to_string());

    println!("{}", pepsi); // Display 模式

    let mut coke = pepsi.clone();

    println!("{}", pepsi.eq(&coke));
    println!("{}", pepsi == coke);
    coke.consume();
    println!("{:?}", coke); // Debug 模式
}
