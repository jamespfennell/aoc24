mod day1;
mod day2;
mod day3;
mod day4;
mod input;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Must provide 2 command line arguments: the day number and problem number");
    }
    let f = match (args[1].as_str(), args[2].as_str()) {
        ("1", "1") => day1::problem_1,
        ("1", "2") => day1::problem_2,
        ("2", "1") => day2::problem_1,
        ("2", "2") => day2::problem_2,
        ("3", "1") => day3::problem_1,
        ("3", "2") => day3::problem_2,
        ("4", "1") => day4::problem_1,
        ("4", "2") => day4::problem_2,
        _ => {
            panic!("Unknown day {} and problem {}", args[1], args[2]);
        }
    };
    println!("{:?}", f());
}

#[cfg(test)]
mod test {
    macro_rules! test_answers {
        ( $( ($package: ident, $answer1: expr, $answer2: expr), )+ ) => {
            $(
                mod $package {
                    #[test]
                    fn test_problem_1() {
                        assert_eq!($answer1, super::super::$package::problem_1());
                    }
                    #[test]
                    fn test_problem_2() {
                        assert_eq!($answer2, super::super::$package::problem_2());
                    }
                }
            )+
        };
    }

    test_answers!(
        (day1, 2000468, 18567089),
        (day2, 341, 404),
        (day3, 182780583, 90772405),
        (day4, 2462, 1877),
    );
}
