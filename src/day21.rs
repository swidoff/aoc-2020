use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day21.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

struct Food {
    ingredients: HashSet<String>,
    allergens: Vec<String>,
}

fn parse_foods(iter: impl Iterator<Item = String>) -> Vec<Food> {
    iter.map(|line| {
        let (part1, part2) = line.split(" (contains ").collect_tuple().unwrap();
        let ingredients = HashSet::from_iter(part1.split_whitespace().map(|s| s.to_string()));
        let allergens = part2
            .strip_suffix(")")
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect_vec();
        Food {
            ingredients,
            allergens,
        }
    })
    .collect_vec()
}

fn find_allergen_candidates(foods: &Vec<Food>) -> HashMap<&String, HashSet<&String>> {
    let mut allergens: HashMap<&String, HashSet<&String>> = HashMap::new();
    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            match allergens.get_mut(allergen) {
                Some(ingredients) => ingredients.retain(|i| food.ingredients.contains(*i)),
                None => allergens
                    .insert(allergen, HashSet::from_iter(food.ingredients.iter()))
                    .map(|_| ())
                    .unwrap_or(()),
            };
        }
    }
    allergens
}

fn find_safe_ingredients(foods: &Vec<Food>) -> Vec<&String> {
    let allergens = find_allergen_candidates(foods);

    foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .into_iter()
        .filter(|i| allergens.values().all(|set| !set.contains(*i)))
        .collect_vec()
}

fn find_unsafe_ingredients(foods: &Vec<Food>) -> HashMap<&String, &String> {
    let mut allergens = find_allergen_candidates(foods);

    // Remove unique field assignments from the other sets.
    // Continue until all sets are unique.
    loop {
        let singletons = allergens
            .iter()
            .filter_map(|(_allergen, ingredients)| {
                if ingredients.len() == 1 {
                    Some(*ingredients.iter().next().unwrap())
                } else {
                    None
                }
            })
            .collect_vec();

        if singletons.len() == allergens.len() {
            break;
        }

        for ingredient in singletons {
            for (_allergen, ingredients) in allergens.iter_mut() {
                if ingredients.len() > 1 {
                    ingredients.remove(ingredient);
                }
            }
        }
    }

    allergens
        .iter()
        .map(|(allergen, ingredients)| (*allergen, *ingredients.iter().next().unwrap()))
        .collect()
}

fn to_canonical_list(allergens: &HashMap<&String, &String>) -> String {
    allergens
        .iter()
        .sorted_by_key(|(allergen, _ingredient)| **allergen)
        .map(|(_allergen, ingredient)| *ingredient)
        .join(",")
}

mod tests {
    use crate::day21::{
        find_safe_ingredients, find_unsafe_ingredients, parse_foods, read_file, to_canonical_list,
    };

    const EXAMPLE: &str = "
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

    #[test]
    fn test_part1_example() {
        let foods = parse_foods(EXAMPLE[1..].lines().map(|line| line.to_string()));
        let res = find_safe_ingredients(&foods);
        println!("{:?}", res);
        assert_eq!(5, res.len());
    }

    #[test]
    fn test_part1() {
        let foods = parse_foods(read_file());
        let res = find_safe_ingredients(&foods);
        println!("{}", res.len());
        assert_eq!(1829, res.len());
    }

    #[test]
    fn test_part2_example() {
        let foods = parse_foods(EXAMPLE[1..].lines().map(|line| line.to_string()));
        let res = find_unsafe_ingredients(&foods);
        println!("{:?}", res);
        assert_eq!("mxmxvkd,sqjhc,fvjkl", to_canonical_list(&res));
    }

    #[test]
    fn test_part2() {
        let foods = parse_foods(read_file());
        let res = find_unsafe_ingredients(&foods);
        let res = to_canonical_list(&res);
        println!("{}", res);
        assert_eq!("mxkh,gkcqxs,bvh,sp,rgc,krjn,bpbdlmg,tdbcfb", res);
    }
}
