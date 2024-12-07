use std::collections::HashMap;

pub fn problem_1(data: &str) -> i32 {
    let data = Data::build(data);
    let mut sum = 0;
    for update in data.updates {
        let mut correct = true;
        for i in 0..update.len() {
            for j in (i + 1)..update.len() {
                if let Some(rule) = data.rules.get(&update[j]) {
                    if rule.contains(&update[i]) {
                        correct = false;
                        break;
                    }
                }
            }
        }
        if !correct {
            continue;
        }
        sum += update[(update.len() - 1) / 2];
    }
    sum
}

pub fn problem_2(_data: &str) -> i32 {
    0
}

#[derive(Debug)]
struct Data {
    rules: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>,
}

impl Data {
    fn build(raw_data: &str) -> Self {
        let mut rules: HashMap<i32, Vec<i32>> = Default::default();
        let mut updates: Vec<Vec<i32>> = Default::default();
        let mut parsing_rules = true;
        for line in raw_data.lines() {
            if line.is_empty() {
                parsing_rules = false;
                continue;
            }
            if parsing_rules {
                let raw: [i32; 2] = line
                    .split("|")
                    .into_iter()
                    .map(|f| {
                        f.parse::<i32>()
                            .expect("expect to be able to parse rule part as integer")
                    })
                    .collect::<Vec<i32>>()
                    .try_into()
                    .expect("expected rule to have two parts");
                rules.entry(raw[0]).or_default().push(raw[1]);
            } else {
                updates.push(
                    line.split_terminator(',')
                        .map(|f| {
                            f.parse::<i32>()
                                .expect("expect to be able to parse rule part as integer")
                        })
                        .collect(),
                );
            }
        }
        Self { rules, updates }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_problem_1() {
        let data = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(143, super::problem_1(data));
    }
}
