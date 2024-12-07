use std::collections::HashMap;

pub fn problem_1(data: &str) -> i64 {
    let data = Data::build(data);
    data.updates
        .iter()
        .filter(|update| update.is_correct(&data.rules))
        .map(Update::median)
        .sum()
}

pub fn problem_2(data: &str) -> i64 {
    let mut data = Data::build(data);
    data.updates
        .iter_mut()
        .filter(|update| !update.is_correct(&data.rules))
        .map(|update| {
            update.sort(&data.rules);
            update.median()
        })
        .sum()
}

#[derive(Debug)]
struct Data {
    rules: HashMap<i64, Vec<i64>>,
    updates: Vec<Update>,
}

impl Data {
    fn build(raw_data: &str) -> Self {
        let mut rules: HashMap<i64, Vec<i64>> = Default::default();
        let mut updates: Vec<Update> = Default::default();
        let mut parsing_rules = true;
        for line in raw_data.lines() {
            if line.is_empty() {
                parsing_rules = false;
                continue;
            }
            if parsing_rules {
                let raw: [i64; 2] = line
                    .split("|")
                    .into_iter()
                    .map(|f| {
                        f.parse::<i64>()
                            .expect("expect to be able to parse rule part as integer")
                    })
                    .collect::<Vec<i64>>()
                    .try_into()
                    .expect("expected rule to have two parts");
                rules.entry(raw[0]).or_default().push(raw[1]);
            } else {
                updates.push(Update(
                    line.split_terminator(',')
                        .map(|f| {
                            f.parse::<i64>()
                                .expect("expect to be able to parse rule part as integer")
                        })
                        .collect(),
                ));
            }
        }
        Self { rules, updates }
    }
}

#[derive(Debug)]
struct Update(Vec<i64>);

impl Update {
    fn is_correct(&self, rules: &HashMap<i64, Vec<i64>>) -> bool {
        for i in 0..self.0.len() {
            for j in (i + 1)..self.0.len() {
                if let Some(rule) = rules.get(&self.0[j]) {
                    if rule.contains(&self.0[i]) {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn median(&self) -> i64 {
        self.0[(self.0.len() - 1) / 2]
    }
    fn sort(&mut self, rules: &HashMap<i64, Vec<i64>>) {
        let mut v: Vec<UpdateElem> = self
            .0
            .iter()
            .copied()
            .map(|value| UpdateElem { value, rules })
            .collect();
        v.sort();
        self.0 = v.into_iter().map(|elem| elem.value).collect();
    }
}

struct UpdateElem<'a> {
    value: i64,
    rules: &'a HashMap<i64, Vec<i64>>,
}

impl<'a> PartialEq for UpdateElem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'a> PartialOrd for UpdateElem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value == other.value {
            return Some(std::cmp::Ordering::Equal);
        }
        let rule = match self.rules.get(&self.value) {
            None => &[],
            Some(rule) => rule.as_slice(),
        };
        if rule.contains(&other.value) {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

impl<'a> Eq for UpdateElem<'a> {}

impl<'a> Ord for UpdateElem<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod test {
    const DATA: &str = "47|53
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

    #[test]
    fn test_problem_1() {
        assert_eq!(143, super::problem_1(DATA));
    }

    #[test]
    fn test_problem_2() {
        assert_eq!(123, super::problem_2(DATA));
    }
}
