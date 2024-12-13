mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod input;
mod iter;
#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Must provide 2 command line arguments: the day number and problem number");
    }
    print_problem_answer(args[1].as_str(), args[2].as_str());
}

macro_rules! days {
    ( $( ($package: ident, $answer1: expr, $answer2: expr), )+ ) => {
        fn print_problem_answer(day: &str, problem: &str) {
            let day = format!["day{day}"];
            let f = match (day.as_str(), problem) {
                $(
                    (stringify![$package], "1") => $package::problem_1,
                    (stringify![$package], "2") => $package::problem_2,
                )+
                _ => {
                    panic!("Unknown day {day} and problem {problem}");
                }
            };
            let data = load_data(&day);
            let answer = f(&data);
            println!["{answer}"];
        }
        #[cfg(test)]
        mod test {
            $(
                mod $package {
                    #[test]
                    fn test_problem_1() {
                        super::super::run_test($answer1.into(), super::super::$package::problem_1, stringify![$package]);
                    }
                    #[test]
                    fn test_problem_2() {
                        super::super::run_test($answer2.into(), super::super::$package::problem_2, stringify![$package]);
                    }
                }
            )+
        }
    }
}

days!(
    (day1, 2000468, 18567089),
    (day2, 341, 404),
    (day3, 182780583, 90772405),
    (day4, 2462, 1877),
    (day5, 4637, 6370),
    (day6, 5145, 1523),
    (day7, 2299996598890, 362646859298554),
    (day8, 336, 1131),
    (day9, 6288707484810, 6311837662089),
    (day10, 552, 1225),
    (day11, 187738, 223767210249237),
    (day12, 1431440, 869070),
    (day13, 29023, None),
    (day14, None, None),
    (day15, None, None),
    (day16, None, None),
    (day17, None, None),
    (day18, None, None),
    (day19, None, None),
    (day20, None, None),
    (day21, None, None),
    (day22, None, None),
    (day23, None, None),
    (day24, None, None),
    (day25, None, None),
);

fn load_data(file_name: &str) -> String {
    let path = format!["data/{}.txt", file_name];
    std::fs::read_to_string(path).expect("expect file to exist")
}

#[cfg(test)]
fn run_test(want: Option<i64>, problem_func: fn(&str) -> i64, package_name: &str) {
    let want = match want {
        None => return,
        Some(want) => want,
    };
    let data = load_data(package_name);
    let got = problem_func(&data);
    assert_eq!(got, want);
}
