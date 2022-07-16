use clap::{Arg, App};

fn main() {
    let matches = App::new("Rusty Fizzbuzz")
        .version("1.0.0")
        .author("Camille Lewis <camille.hodapp@gmail.com>")
        .about("Having some fizzbuzz fun with Rust.")
        .arg(Arg::with_name("fizz")
                 .short('f')
                 .long("fizz")
                 .takes_value(true)
                 .help("The fizz value. Defaults to 3."))
        .arg(Arg::with_name("buzz")
                 .short('b')
                 .long("buzz")
                 .takes_value(true)
                 .help("The buzz value. Defaults to 5."))
        .arg(Arg::with_name("number")
                 .short('n')
                 .long("number")
                 .takes_value(true)
                 .help("The number to count to."))
        .get_matches();

    let fizz_str = matches.value_of("fizz");
    let fizz = convert_string_to_uint_or_default(fizz_str, 3);

    let buzz_str = matches.value_of("buzz");
    let buzz = convert_string_to_uint_or_default(buzz_str, 5);

    let number_str = matches.value_of("number");
    let number = convert_string_to_uint_or_default(number_str, 15);

    println!("Fizzbuzz to {0} with fizz = {1} and buzz = {2}", number, fizz, buzz);

    let fizzbuzz = fizzbuzz_to(number, fizz, buzz);
    for fb in fizzbuzz.iter() {
        println!("{}", fb);
    }
}

fn convert_string_to_uint_or_default(string: Option<&str>, default_value: u32) -> u32 {
    match string {
        Some(s) => {
            match s.parse::<u32>() {
                Ok(n) => {return n;},
                Err(_) => {return default_value;}
            }
        },
        None => {return default_value;}
    }
}

fn is_divisible_by(left: u32, right: u32) -> bool {
    if right > 0 {
        return left % right == 0;
    }

    return false;
}

fn fizzbuzz(number: u32, fizz: u32, buzz: u32) -> String {
    if is_divisible_by(number, fizz * buzz) {
        return "fizzbuzz".to_string();
    } else if is_divisible_by(number, fizz) {
        return "fizz".to_string();
    } else if is_divisible_by(number, buzz) {
        return "buzz".to_string();
    } else {
        return number.to_string();
    }
}


fn fizzbuzz_to(number: u32, fizz: u32, buzz: u32) -> Vec<String> {
    let mut output = Vec::with_capacity(number.try_into().unwrap());

    for n in 1..=number {
        output.push(fizzbuzz(n, fizz, buzz));
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*; // imports names from outer scope

    #[test]
    fn test_is_divisible_by() {
        assert_eq!(is_divisible_by(3, 0), false);
        assert_eq!(is_divisible_by(3, 2), false);
        assert!(is_divisible_by(6, 3));
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(15, 3, 5), "fizzbuzz");
        assert_eq!(fizzbuzz(9, 3, 5), "fizz");
        assert_eq!(fizzbuzz(10, 3, 5), "buzz");
        assert_eq!(fizzbuzz(7, 3, 5), "7");
    }

    #[test]
    fn test_fizzbuzz_to() {
        let expected = vec!["1", "fizz", "buzz", "fizz", "5", "fizzbuzz"];
        assert_eq!(fizzbuzz_to(6, 2, 3), expected);
    }

    // #[test]
    // fn test_convert_string_to_uint_or_default() {
    //     assert_eq!(convert_string_to_uint_or_default("1".to_string(), 3), 1);
    //     assert_eq!(convert_string_to_uint_or_default("abc".to_string(), 5), 5);
    //     assert_eq!(convert_string_to_uint_or_default("()".to_string(), 1), 1);
    // }
}