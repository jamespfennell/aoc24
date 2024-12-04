mod day1;
mod day2;
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
        _ => {
            panic!("Unknown day {} and problem {}", args[1], args[2]);
        }
    };
    println!("{:?}", f());
}
