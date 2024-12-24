mod algorithms;
mod common;
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
            let data = load_data(&day);
            let now = std::time::Instant::now();
            let answer = match (day.as_str(), problem) {
                $(
                    (stringify![$package], "1") => format!["{}", $package::problem_1(&data)],
                    (stringify![$package], "2") => format!["{}", $package::problem_2(&data)],
                )+
                _ => {
                    panic!("Unknown day {day} and problem {problem}");
                }
            };
            let elapsed = now.elapsed();
            println!("{answer} (took {:.2?})", elapsed);
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
    (day13, 29023, 96787395375634),
    (day14, 224969976, 7892),
    (day15, 1495147, 1524905),
    (day16, 134588, 631),
    (day17, 2_1_0_4_6_2_4_2_0, 109685330781408),
    (day18, 234, "58,19".to_string()),
    (day19, 342, 891192814474630),
    (day20, 1404, 1010981),
    (day21, 219254, 264518225304496),
    (day22, 14476723788, 1630),
    (
        day23,
        893,
        "cw,dy,ef,iw,ji,jv,ka,ob,qv,ry,ua,wt,xz".to_string()
    ),
    (day24, None, None),
    (day25, None, None),
);

fn load_data(file_name: &str) -> String {
    let path = format!["data/{}.txt", file_name];
    std::fs::read_to_string(path).expect("expect file to exist")
}

#[cfg(test)]
fn run_test<T: Eq + std::fmt::Debug>(
    want: Option<T>,
    problem_func: fn(&str) -> T,
    package_name: &str,
) {
    let want = match want {
        None => return,
        Some(want) => want,
    };
    let data = load_data(package_name);
    let got = problem_func(&data);
    assert_eq!(got, want);
}
