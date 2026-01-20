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
}
