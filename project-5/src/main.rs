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

    // match 匹配多个值
    let number = 2;
    match number {
        1 | 3 | 5 | 7 | 9 | 11 | 13 | 15 | 17 | 19 | 21 | 23 | 25 | 27 | 29 => {
            println!("The number is odd.")
        }
        2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 | 20 | 22 | 24 | 26 | 28 => {
            println!("The number is odd.")
        }
        _ => println!("The number is unknow."),
    }

    // 本初有两种做法
    let value = 8;
    match value {
        value if value % 2 == 0 => println!("The value is even"),
        _ => println!("The number is even"),
    }

    match value {
        value if value % 2 == 0 => println!("The value is even"),
        x if x % 2 != 0 => println!("The number is even"),
        _ => println!("The value is unknow."),
    }

    match value {
        value if value % 2 == 0 => println!("The value is even"),
        x if x % 2 != 0 => println!("The number is even"),
        _ => unreachable!(),
    }

    // loop 迭代循环
    let mut value = 10;

    loop {
        if value == 0 {
            break;
        }
        println!("The number is: {}", value);
        value -= 1;
    }
    println!("Now, The number is: {}", value);

    // continue 语句 与 loop 循环结合

    let mut seconds = 23;

    loop {
        if seconds <= 0 {
            println!("Time is over.");
            break;
        }

        if seconds % 2 == 0 {
            println!("The seconds is even. Jump. {}s", seconds);
            seconds -= 3;
            continue;
        }

        println!("Time is running... {}s", seconds);
        seconds -= 1;
    }

    seconds = 23;
    println!("while version.");
    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("The seconds is even. Jump. {}s", seconds);
            seconds -= 3;
            continue;
        }

        println!("Time is running... {}s", seconds);
        seconds -= 1;
    }

    // 递归
    countdown(12);

    // test
    println!("color is: {}", color_to_number("blue"));
    println!("color is: {}", factorial(5));
    println!("color is: {}", factorial_recursion(5));
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is: {}", result);
}

fn countdown(seconds: i16) {
    if seconds <= 0 {
        println!("finished seconds.");
    } else {
        println!("{} seconds...", seconds);
        countdown(seconds - 1)
    }
}
/*
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.
*/
fn color_to_number(color: &str) -> i16 {
    /*if color == "red"{
        return 1;
    }else if color == "green" {
        return 2;
    }else if color == "blue" {
        return 3;
    }else {
        return 0;
    }*/

    /*
        return if color == "red" {
            1
        } else if color == "green" {
            2
        } else if color == "blue" {
            3
        } else {
            0
        }
    */

    match color {
        "red" => {
            return 1;
        }
        "green" => {
            return 2;
        }
        "blue" => {
            return 3;
        }
        _ => {
            return 0;
        }
    }

    /*match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }*/

    /*return match color {
        "red" => {
            1
        }
        "green" => {
            2
        }
        "blue" => {
            3
        }
        _ => {
            0
        }
    }*/
}

fn factorial(number: i32) -> i32 {
    let mut sum = 1;
    let mut count = number;

    while count >= 1 {
        sum *= count;
        count -= 1;
    }

    // 等同于 return sum;
    sum
}

/*
    只要返回了递归函数，那么这个值就会被一起带到下一层
*/
fn factorial_recursion(number: i32) -> i32 {
    if number < 1 {
        return 1;
    } else {
        // 这句话等同于 return number * factorial_recursion(number - 1)
        number * factorial_recursion(number - 1)
    }
}
