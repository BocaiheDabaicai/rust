// 9. 数据结构的debug - part1
#[derive(Debug)] // 允许打印 数据结构 的规范表达

// 1. 数据结构定义
struct Coffee {
    // 命名规则：各单词首字母大写
    name: String,
    price: f64,
    is_available: bool,
}

// 10. 定义数据结构方法 - part1
#[derive(Debug)]
struct RoseSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

// 16. 构建模式 - part1
#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl RoseSong {
    // 14. 关联函数, 构造函数设计
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        // 返回值可以表达为：RoseSong, Self
        // 如果不包含self，那么rust 会认为这是一个关联函数
        /*RoseSong {
            title,
            release_year,
            duration_secs,
        }*/

        Self {
            title,
            release_year,
            duration_secs,
        }
    }

    // 声明数据结构方法
    /*fn display_song_info(self: Self) {
        // 传入方法一： self:RoseSong
        // 传入方法二： self:Self: Self 表示 RoseSong，本处的数据结构类型
        // 传入方法三： self
        // 对数据结构体的数据引用
        // part1 - 不可变数据结构值
        println!("-- RoseSong --");
        println!("title: {}", self.title);
        println!("release_year: {}", self.release_year);
        println!("duration_secs: {}", self.duration_secs);
    }*/

    /*fn double_length(mut self) {
        self.duration_secs *= 2;
        println!("Total Information: {:#?}", self);
        // self.display_song_info();
    }*/
    // 11. 返回所有权设计
    fn display_song_info(self: &Self) {
        // 传入方法一： self: &RoseSong
        // 传入方法二： self: &Self: &Self 表示 RoseSong 的引用，引用本处的数据结构类型
        // 传入方法三： &self
        // 推荐写二、三方法，有利于数据结构名称维护，而不至于修改所有函数代码
        // 对数据结构体的数据引用
        // part1 - 不可变数据结构值
        println!("-- RoseSong --");
        println!("title: {}", self.title);
        println!("release_year: {}", self.release_year);
        println!("duration_secs: {}", self.duration_secs);
        println!("Year since Release: {}", self.years_since_release());
    }

    fn double_length(self: &mut Self) {
        // 对实例的可变引用
        // 传入方法一： self:&mut RoseSong
        // 传入方法二： self:&mut Self: &Self
        // 传入方法三： &mut self
        self.duration_secs *= 2;
    }

    // 12. 函数引入多个参数
    fn is_longer_that(self: &Self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    // 13. 函数中调用函数
    fn years_since_release(self: &Self) -> u32 {
        2029 - self.release_year
    }
}

// 15. 多模块接口
impl RoseSong {
    // 可以存在多个相关于一个数据结构的接口模块
    fn display_song_info_two(self: &Self) {
        println!("-- RoseSong Version 2.0 --");
        println!("title: {}", self.title);
        println!("release_year: {}", self.release_year);
        println!("duration_secs: {}", self.duration_secs);
        println!("Year since Release: {}", self.years_since_release());
    }
}

// 16. 构建模式 - part2
impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(self: &mut Self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(self: &mut Self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(self: &mut Self, new_hard_drive_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_hard_drive_capacity;
        self
    }

    fn print_computer_info(self: &Self) {
        println!("-- Computer --");
        println!("cpu: {}", self.cpu);
        println!("memory: {}", self.memory);
        println!("hard_drive_capacity: {}", self.hard_drive_capacity);
    }
}

// 17. 元组数据结构 - par1
struct ShortDuration(u32, u32);
struct LongDuration(u32, u32);

// 18. 单元数据结构 - part1
struct Empty; // 这是一个单元数据结构

// 19. test - part1
#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(self: &mut Self, new_destination: String) -> &mut Self {
        self.destination = new_destination;
        self
    }

    fn increase_price(self: &mut Self) -> &mut Self {
        self.price *= 1.2;
        self
    }

    fn itinerary(self: &mut Self) -> &mut Self {
        println!("-- Flight --");
        println!(
            "origin: {} -> destination: {}",
            self.origin, self.destination
        );
        println!("price: {}", self.price);
        println!("passengers: {}", self.passengers);
        self
    }
}

fn main() {
    // 2. 创建数据结构实例
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 5.0,
        is_available: true,
    };

    // 3. 读取数据结构的属性
    println!("-- mocha --");
    println!("name: {}", mocha.name);
    println!("price: {}", mocha.price);
    println!("is_available: {}", mocha.is_available);

    let name = mocha.name; // 会转移数据结构对象中 name 的所有权

    println!("Convert name: {}", name);
    // println!("name: {}", mocha.name); // 此处不可运行

    // 4. 复写数据结构的领域
    let mut mocha_great = Coffee {
        // 数据结构只能是全部可修改或者全部不可修改
        name: String::from("Mocha"),
        price: 5.0,
        is_available: true,
    };

    mocha_great.name = String::from("Brushes");

    println!("-- mocha_great --");
    println!("name: {}", mocha_great.name);
    println!("price: {}", mocha_great.price);
    println!("is_available: {}", mocha_great.is_available);

    // 5. 创建一个返回数据结构的函数
    /*
        name 所有权转移过程：
        （1）转移到函数 make_coffee
        （2）转移到函数创建的数据结构中 Coffee
        （3）转移给数据结构对象 mocha_create
    */
    let name = String::from("Apples");
    let mocha_create = make_coffee(name, 21.19, false);

    println!("-- mocha_create --");
    println!("name: {}", mocha_create.name);
    println!("price: {}", mocha_create.price);
    println!("is_available: {}", mocha_create.is_available);

    // 6. 数据结构简洁实例化方法
    let name = String::from("Pears");
    let price = 13.39;
    let is_available = true;
    let mocha_shorthand = Coffee {
        name,
        price,
        is_available,
    };

    println!("-- mocha_shorthand --");
    println!("name: {}", mocha_shorthand.name);
    println!("price: {}", mocha_shorthand.price);
    println!("is_available: {}", mocha_shorthand.is_available);

    // 7. 数据结构更新语法
    let mocha_camel = Coffee {
        name: String::from("Camel Coffee"),
        ..mocha_shorthand // 引用该对象复制它 除name 以外 的所有属性进行使用
    };

    /*let mocha_camel = Coffee {
        ..mocha_shorthand   // 会将 该对象的属性值及所有权全部转移
    };*/

    /*let mocha_camel = Coffee {
        name:mocha_shorthand.name.clone(), // 修正方法
        ..mocha_shorthand
    };*/

    println!("-- mocha_camel --");
    println!("name: {}", mocha_camel.name);
    println!("price: {}", mocha_camel.price);
    println!("is_available: {}", mocha_camel.is_available);

    /*println!("-- mocha_shorthand --");
    println!("name: {}", mocha_shorthand.name);
    println!("price: {}", mocha_shorthand.price);
    println!("is_available: {}", mocha_shorthand.is_available);*/

    // 8. 传递数据结构给函数
    let mut mocha_function = make_coffee(String::from("Popping Coffee"), 21.19, false);
    // drink_coffee(mocha_function);
    // drink_coffee_two(&mocha_function);
    drink_coffee_three(&mut mocha_function);

    /*println!("-- mocha_function --");   // 所有权已经被转移，后续无法使用
    println!("name: {}", mocha_function.name);
    println!("price: {}", mocha_function.price);
    println!("is_available: {}", mocha_function.is_available);*/

    // 9. 数据结构的debug
    // #[allow(unused_variables)] 单行指令 - 允许存在未使用的变量
    let mocha_debug = make_coffee(String::from("Debug Coffee"), 21.19, false);

    println!("{:?}", mocha_debug);
    println!("{:#?}", mocha_debug);

    // 10. 定义数据结构方法
    let mut rose_song_one = RoseSong {
        title: String::from("Gone."),
        release_year: 2021,
        duration_secs: 207,
    };

    // rose_song_one.display_song_info(); // rose_song_one 的所有权转移到了这里

    // rose_song_one.double_length();

    // 11. 返回所有权设计
    rose_song_one.display_song_info();
    rose_song_one.double_length();

    // 12. 函数引入多个参数
    let rose_song_two = RoseSong {
        title: String::from("Two box Seconds."),
        release_year: 2026,
        duration_secs: 128,
    };

    if rose_song_one.is_longer_that(&rose_song_two) {
        println!("The compare result is: rose_song_one win !!!");
    } else {
        println!("The compare result is: rose_song_two win !!!");
    }

    // 13. 函数中调用函数
    rose_song_two.display_song_info();

    // 14. 关联函数, 构造函数设计
    /*
        关联函数
        内置函数，可以引用对象本身
    */
    let rose_song_three = RoseSong::new(String::from("Three Candy."), 2027, 256);
    rose_song_three.display_song_info();

    // 15. 多模块接口
    rose_song_three.display_song_info_two();

    // 16. 构建模式
    let mut computer = Computer {
        cpu: String::from("Intel"),
        memory: 16,
        hard_drive_capacity: 1024,
    };

    computer.print_computer_info();

    // 实现方法一
    /*computer.upgrade_cpu(String::from("Amd"));
    computer.upgrade_memory(32);
    computer.upgrade_hard_drive_capacity(4096);*/

    // 实现方法二
    computer
        .upgrade_cpu(String::from("Amd"))
        .upgrade_memory(32)
        .upgrade_hard_drive_capacity(4096);

    computer.print_computer_info();

    // 17. 元组数据结构
    /*
        从名称上区分数据，从而避免原始元组数据类型相同的问题
        以及，函数参数类型命名上的问题
        这个问题已经得到官方的修复和错误捕捉
    */
    let work_shift = ShortDuration(8, 0);
    println!("{} hour {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} year {} month", era.0, era.1);

    // let era_raw = (5,3);
    // go_to_work(era_raw);
    go_to_work(era);

    // 18. 单元数据结构
    let empty = (); // 这是一个单元结构体
    let empty_struct = Empty; // 这是一个单元结构体

    // 19. test
    let mut flight_one = Flight::new(
        String::from("BeiJing"),
        String::from("ShangHai"),
        1284.231,
        54,
    );

    println!("Origin Info: {:#?}", flight_one);

    flight_one
        .change_destination(String::from("HongKong"))
        .increase_price()
        .itinerary();

    let mut flight_two = Flight {
        origin: String::from("ChongQing"),
        destination: String::from("GuangZhou"),
        ..flight_one
    };

    println!("Origin Info: {:#?}", flight_two);
    flight_two.itinerary();
}

fn make_coffee(name: String, price: f64, is_available: bool) -> Coffee {
    // 写法一
    /*Coffee {
        name: name,
        price: price,
        is_available: is_available,
    }*/
    // 写法二
    Coffee {
        name,
        price,
        is_available,
    }
}

fn drink_coffee(mut coffee: Coffee) {
    // 数据结构对象本身不需要标注 mut
    println!("-- coffee function --");
    println!("name: {}", coffee.name);
    println!("price: {}", coffee.price);
    println!("is_available: {}", coffee.is_available);

    coffee.is_available = true;
    println!("Config is_available: {}", coffee.is_available);
}

fn drink_coffee_two(coffee: &Coffee) {
    // 不可变引用
    println!("-- coffee function --");
    println!("name: {}", coffee.name);
    println!("price: {}", coffee.price);
    println!("is_available: {}", coffee.is_available);
}

fn drink_coffee_three(coffee: &mut Coffee) {
    // 可变引用
    println!("-- coffee function --");
    println!("name: {}", coffee.name);
    println!("price: {}", coffee.price);
    println!("is_available: {}", coffee.is_available);

    coffee.is_available = true;
    println!("Config is_available: {}", coffee.is_available);
}

fn go_to_work(value: LongDuration) {
    println!("Test question: {} hours {} minutes", value.0, value.1);
}
