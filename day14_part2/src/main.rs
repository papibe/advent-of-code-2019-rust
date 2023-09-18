use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

const TRILLION: i64 = 1000000000000;

#[derive(Debug)]
struct Ingredient {
    name: String,
    quantity: i64,
}

#[derive(Debug)]
struct Rule {
    quantity: i64,
    ingredients: Vec<Ingredient>
}

fn parse(filename: &str) -> HashMap<String, Rule> {
    // read file
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {}", filename));

    let mut rules: HashMap<String, Rule> = HashMap::new();
    let line_re = Regex::new(r"(?m)^([^=]+) => ([0-9]+) ([A-Z]+)$").unwrap();

    for (_, [ingredients, quantity, rule_name]) in line_re.captures_iter(data.as_str()).map(|c| c.extract()) {
        let mut recipes: Vec<Ingredient> = vec![];
        let ingredient_re = Regex::new(r"^ *(?<quantity>\d+) (?<name>\w+) *$").unwrap();

        for ingredient in ingredients.split(", ") {
            let recipe = ingredient_re.captures(ingredient).unwrap();
            recipes.push(
                Ingredient {
                    name: recipe["name"].to_string(),
                    quantity: recipe["quantity"].parse::<i64>().unwrap(),
                }
            );
        }
        rules.insert(rule_name.to_string(), Rule {quantity: quantity.parse::<i64>().unwrap(), ingredients: recipes});
    }
    rules
}

fn produce(rules: &HashMap<String, Rule>, fuel: i64) -> i64 {
    

    let mut ore_counter: i64 = 0;
    let mut stock: HashMap<String, i64> = HashMap::new();
    let mut queue: VecDeque<(String, i64)> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();

    visited.insert("".to_string());

    queue.push_back(("FUEL".to_string(), fuel));

    while queue.len() > 0 {
        let (ingredient, mut quantity) = queue.pop_front().unwrap();

        // update quantity if we have stock
        if stock.contains_key(&ingredient) {
            if quantity <= stock[&ingredient] {
                *stock.get_mut(&ingredient).unwrap() -= quantity;
                if stock[&ingredient] == 0 {
                    stock.remove(&ingredient);
                }
                continue;
            } else {
                quantity -= stock[&ingredient];
                stock.remove(&ingredient);
            }
        }

        // get integer multiplier
        let multiplier: i64 = (quantity + rules[&ingredient].quantity - 1) / rules[&ingredient].quantity;
        let prod_produced: i64 = rules[&ingredient].quantity * multiplier;

        // save extra production in stock
        if prod_produced > quantity {
            *stock.entry(ingredient.clone()).or_insert(0) += prod_produced - quantity;
        }
        // apply rule with other ingredients
        for other_ingredient in & rules[&ingredient].ingredients {
            if other_ingredient.name == "ORE" {
                ore_counter += other_ingredient.quantity * multiplier;
            } else {
                queue.push_back((other_ingredient.name.clone(), other_ingredient.quantity * multiplier));
            }
        }
    }
    ore_counter
}

fn solution(filename: &str) -> i64 {
    let rules = parse(filename);

    let mut lower: i64 = 0;
    let mut higher: i64 = TRILLION * 2;

    while lower < higher {
        let middle: i64 = (lower + higher + 1) / 2;
        if produce(&rules, middle) <= TRILLION {
            lower = middle;
        } else {
            higher = middle - 1;
        }
    }
    lower
}

fn main() {
    println!("{}", solution("./input.txt"));    // 1572358
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example2_should_be_82892753() {
        assert_eq!(solution("./example2.txt"), 82892753);
    }

    #[test]
    fn example3_should_be_5586022() {
        assert_eq!(solution("./example3.txt"), 5586022);
    }

    #[test]
    fn example4_should_be_460664() {
        assert_eq!(solution("./example4.txt"), 460664);
    }
}
