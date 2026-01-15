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
    PayPal(String, String),
}
struct Card {
    rank: String,
    suit: CardSuit,
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
    let mut my_visa = PaymentMethodType::CreditCard(String::from("0034-2331-221"));
    my_visa = PaymentMethodType::PayPal(String::from("3877-213"), String::from("1221-399"));

    println!("{:#?}", my_visa);
}
