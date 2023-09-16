use std::fs;
use std::collections::{HashMap, VecDeque};
use regex::Regex;


#[derive(Debug)]
struct Ingredient {
    name: String,
    quantity: i32,
}

#[derive(Debug)]
struct Rule {
    quantity: i32,
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
                    quantity: recipe["quantity"].parse::<i32>().unwrap(),
                }
            );
        }
        rules.insert(rule_name.to_string(), Rule {quantity: quantity.parse::<i32>().unwrap(), ingredients: recipes});
    }
    rules
}

fn solution(filename: &str) -> i32 {
    let rules = parse(filename);

    let mut ore_counter: i32 = 0;
    let mut stock: HashMap<String, i32> = HashMap::new();
    let mut queue: VecDeque<(String, i32)> = VecDeque::new();

    queue.push_back(("FUEL".to_string(), 1));

    while queue.len() > 0 {
        let (ingredient, mut quantity) = queue.pop_front().unwrap();

        // update quantity if we have stock
        if stock.contains_key(&ingredient) {
            if quantity <= stock[&ingredient] {
                *stock.get_mut(&ingredient).unwrap() -= quantity;
                continue;
            } else {
                quantity -= stock[&ingredient];
                stock.remove(&ingredient);
            }
        }

        // get integer multiplier
        let multiplier: i32 = (quantity + rules[&ingredient].quantity - 1) / rules[&ingredient].quantity;
        let prod_produced: i32 = rules[&ingredient].quantity * multiplier;

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

fn main() {
    println!("{}", solution("./input.txt"));    // 1037742
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_should_be_31() {
        assert_eq!(solution("./example1.txt"), 31);
    }

    #[test]
    fn example2_should_be_165() {
        assert_eq!(solution("./example2.txt"), 165);
    }

    #[test]
    fn example3_should_be_13312() {
        assert_eq!(solution("./example3.txt"), 13312);
    }

    #[test]
    fn example4_should_be_180697() {
        assert_eq!(solution("./example4.txt"), 180697);
    }

    #[test]
    fn example5_should_be_2210736() {
        assert_eq!(solution("./example5.txt"), 2210736);
    }
}
