fn main() {
    // if 条件
    let data1: bool = true;

    if true {
        println!("Hello, world!");
    }

    if false {
        println!("can't execute.");
    }

    if data1 {
        println!("data1 can execute.");
    }

    // if else 条件
    let season = "summer";

    if season == "summer" {
        println!("Now is summer day.");
    } else if season == "winter" {
        println!("Now is winter day.");
    } else if season == "spring" {
        println!("Now is spring day.");
    } else if season == "autumn" {
        println!("Now is autumn day.");
    } else {
        println!("We don't know the season that we face.");
    }

    // if 简短式
    even_or_odd(12);
    even_or_odd(17);

    // match
    let evaluation = true;

    match evaluation {
        true => {
            println!("The evaluation is true.");
        }
        false => {
            println!("The evaluation is false.");
        }
    }

    let data2 = true;
    let data3 = match data2 {
        true => 20,
        false => 40,
    };

    println!("The data2 is: {}", data2);
    println!("The data3 is: {}", data3);

    // match for season， 两种表达方式：语句块模式、单行模式
    match season {
        "summer" => {
            println!("Now is summer day.");
        }
        "winter" => {
            println!("Now is winter day.");
        }
        "spring" => {
            println!("Now is spring day.");
        }
        "autumn" => {
            println!("Now is autumn day.");
        }
        _ => {
            println!("We don't know the season that we face.");
        }
    }

    match season {
        "summer" => println!("Now is summer day."),
        "winter" => println!("Now is winter day."),
        "spring" => println!("Now is spring day."),
        "autumn" => println!("Now is autumn day."),
        _ => println!("We don't know the season that we face."),
    }
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is: {}", result);
}
