// 1. 介绍枚举 - part1
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

// 2. 枚举关联值 - part1
#[derive(Debug)]
enum PaymentMethodType {
    // CreditCard(String, i32, bool),
    CreditCard(String),
    DebitCard(String),
    // PayPal(String, String),
    // PayPal { name: String, password: String },
    PayPal(Credentials),
    Cash, // 不存储任何数据
}
struct Card {
    rank: String,
    suit: CardSuit,
}

// 5. 结构变量 - part1
#[derive(Debug)]
struct Credentials {
    name: String,
    password: String,
}

// 6. 枚举中的枚举 - part1
#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    VeganPlate,
}
#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}
#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

// 7. 匹配关键字 - part1
enum OperationSystem {
    Windows,
    MacOs,
    Linux,
}

// 9. 匹配关键字 - part3
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

// 10. 在枚举中定义方法 - part1
impl LaundryCycle {
    fn wash_laundry(self: &Self) {
        match self {
            LaundryCycle::Cold => println!("LaundryCycle is Cold"),
            LaundryCycle::Hot { temperature } => {
                println!(
                    "LaundryCycle is temperature, its value is : {}",
                    temperature
                );
            }
            LaundryCycle::Delicate(fabric) => {
                println!("LaundryCycle is String, its value is : {}", fabric);
            }
        }
    }
}

// 11. 匹配关键字 - part4
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(self: &Self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Your Item has been delivered for Ordered or Packed")
            }
            other_status => {
                println!("Your Item has been delivered for Shipped or Delivered, {other_status:#?}")
            }
        }
    }
}

// 11. 匹配关键字 - part5
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

impl Milk {
    fn drink(self: &Self) {
        match self {
            Milk::Lowfat(2) => {
                println!("The milk is lowfat with 2.")
            }
            Milk::Lowfat(percent) => {
                println!("The milk is lowfat with {}.", percent)
            }
            Milk::Whole => {
                println!("The milk is whole fat.");
            }
            other_milk => {
                println!("The milk is other_milk.");
            }
        }
    }
}

// test
#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}
#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(self: &Self) {
        match self {
            Subscription::Free => println!("You have limited access to the site"),
            Subscription::Basic(price, months) => println!(
                "You have limited access to the site's premium features for {price} for {months} months"
            ),
            Subscription::Premium { tier } => println!(
                "You have full access to the site's premium features. Your tier is {tier:?}"
            ),
        }
    }
}

fn main() {
    // 1. 介绍枚举
    let first_card = CardSuit::Diamonds;
    let mut second_card = CardSuit::Hearts;
    second_card = CardSuit::Clubs;
    println!("{:#?}", first_card);
    println!("{:#?}", second_card);

    let card_suits = [
        CardSuit::Hearts,
        CardSuit::Diamonds,
        CardSuit::Clubs,
        CardSuit::Spades,
    ];

    let card_suits_two = (CardSuit::Hearts, CardSuit::Spades);

    println!("{:?}", card_suits);
    println!("{:?}", card_suits_two);

    // 2. 枚举关联值 - part1
    let visa = PaymentMethodType::CreditCard(String::from("0034-2331-221"));
    let mastercard = PaymentMethodType::DebitCard(String::from("0998-4412-133"));

    println!("{:#?}", visa);
    println!("{:#?}", mastercard);

    // 3. 枚举关联值 - part2
    /*let mut my_visa = PaymentMethodType::CreditCard(String::from("0034-2331-221"));
    my_visa = PaymentMethodType::PayPal(String::from("3877-213"), String::from("1221-399"));

    println!("{:#?}", my_visa);*/

    // 4. 枚举存储的简单描述
    // 5. 结构变量

    let paypal_credentials = Credentials {
        name: String::from("wiiw@qq.com"),
        password: String::from("228371"),
    };

    /*let my_visa = PaymentMethodType::PayPal{
        name: String::from("wiiw@qq.com"),
        password: String::from("228371"),
    };*/

    let my_visa = PaymentMethodType::PayPal(paypal_credentials);

    println!("my_visa: {:#?}", my_visa);

    // 6. 枚举中的枚举
    let lunch = RestaurantItem::Burrito {
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let meal = RestaurantItem::VeganPlate;

    println!("lunch: {:#?}", lunch);
    println!("dinner: {:#?}", dinner);
    println!("meal: {:#?}", meal);

    // 7. 匹配关键字 - part1
    let number = 5;

    match number {
        5 => println!("The number is 5"),
        8 => println!("The number is 8"),
        _ => println!("The number is not 5 or 8"),
    }

    let computer_1 = OperationSystem::Linux;
    let computer_2 = OperationSystem::Windows;
    let computer_3 = OperationSystem::MacOs;

    println!("{}", years_since_release(computer_1));
    println!("{}", years_since_release(computer_2));
    println!("{}", years_since_release(computer_3));

    // 8. 匹配关键字 - part2
    // 扩展匹配模块，在 match 模块中实现打印和返回值

    // 9. 匹配关键字 - part3
    let type_1 = LaundryCycle::Cold;
    let type_2 = LaundryCycle::Hot { temperature: 31 };
    let type_3 = LaundryCycle::Delicate(String::from("Beautiful!!"));

    wash_laundry(&type_1);
    wash_laundry(&type_2);
    wash_laundry(&type_3);

    // 10. 在枚举中定义方法
    type_1.wash_laundry();
    type_2.wash_laundry();
    type_3.wash_laundry();

    // 11. 匹配关键字 - part4
    let status_1 = OnlineOrderStatus::Ordered;
    let status_2 = OnlineOrderStatus::Shipped;

    status_1.check();
    status_2.check();

    // 11. 匹配关键字 - part5
    let milk_1 = Milk::Lowfat(2);
    let milk_2 = Milk::Lowfat(23);
    let milk_3 = Milk::Whole;

    milk_1.drink();
    milk_2.drink();
    milk_3.drink();

    // 12. if 结构运用
    /*
        条件中就是判断左边变量与右边变量是否一致
        本处没有返回值
    */
    let milk_if_1 = Milk::Whole;
    let milk_if_2 = Milk::Lowfat(12);
    let milk_if_3 = Milk::NonDairy {
        kind: String::from("Plant"),
    };

    let data_milk_1 = if let Milk::Whole = milk_if_1 {
        println!("The milk is Milk::Whole.");
    } else {
        println!("The milk is other Milk.");
    };

    let data_milk_2 = if let Milk::Whole = milk_if_2 {
        println!("The milk is Milk::Whole.");
    } else {
        println!("The milk is other Milk.");
    };

    let data_milk_3 = if let Milk::NonDairy { kind } = milk_if_3 {
        println!("The milk is Milk::NonDairy{{{}}}.", kind);
    } else {
        println!("The milk is other Milk.");
    };

    println!("data_milk_1 : {:?}", data_milk_1);
    println!("data_milk_2 : {:?}", data_milk_2);
    println!("data_milk_3 : {:?}", data_milk_3);

    // 13. if 与 else 结构运用
    let milk_if_4 = Milk::Lowfat(2);

    // 相等时 值会扩展出范围区域
    /*let Milk::Lowfat(percent) = milk_if_4 else {
        println!("The milk is wrong Milk.");
        return;
    };

    println!("{percent} % Lowfat milk");

    // 不相等时 值不会扩展出范围区域
    let Milk::NonDairy { kind } = milk_if_4 else {
        println!("The milk is wrong Milk.");
        return;
    };

    println!("{kind} % Lowfat milk");*/

    // test
    let instance_1 = Subscription::Free;
    let instance_2 = Subscription::Premium { tier: Tier::Gold};
    let instance_3 = Subscription::Basic(12.4,6);

    instance_1.summarize();
    instance_2.summarize();
    instance_3.summarize();
}

fn years_since_release(os: OperationSystem) -> u32 {
    // 会要求覆盖所有的枚举属性
    match os {
        OperationSystem::Windows => {
            println!("The computer System is windows");
            39
        }
        OperationSystem::MacOs => {
            println!("The computer System is MacOs");
            21
        }
        OperationSystem::Linux => {
            println!("The computer System is MacOs");
            7
        }
    }
}

fn wash_laundry(cycle: &LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => println!("LaundryCycle is Cold"),
        LaundryCycle::Hot { temperature } => {
            println!(
                "LaundryCycle is temperature, its value is : {}",
                temperature
            );
        }
        LaundryCycle::Delicate(fabric) => {
            println!("LaundryCycle is String, its value is : {}", fabric);
        }
    }
}
