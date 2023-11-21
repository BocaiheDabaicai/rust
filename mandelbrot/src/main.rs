use num::Complex;
use std::str::FromStr;

struct Complex<T>{
    re:T,
    im:T
}

fn main() {
    println!("Hello, world!");
}

fn complex_square _add_loop(c:Complex<f64>){
    let mut z = Complex {re:0.0,im:0.0};
    loop {
        z = z * z + c;
    }
}

fn escape_time(c:Complex<f64>,limit:usize) -> Option<usize> {
    let mut z = Complex{re:0.0,im:0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn parse_pair<T: FromStr>(s:&str,separator:char) -> Option<(T,T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T:from_str(&s[...index]), T::from_str(&s[index+1..])){
                (Ok(l),Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

fn parse_complex(s:&str) -> Option<Complex<f64>> {
    match parse_pair(s,','){
        Some((re,im)) => Some(Complex{re,im}),
        None => None
    }
}

#[test]
fn test_parse_complex(){
    assert_eq!(parse_complex("1.25,-0.0625"),Some(complex {re:1.25,im:-0.0625}));
    assert_eq!(parse_complex(",-0.0625"),None);
}