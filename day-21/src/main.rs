use std::collections::{HashMap, HashSet};
use std::convert::From;

static INPUT: &str = include_str!("../input.txt");
// static INPUT: &str = include_str!("../test-input.txt");
static CONTAINS: &str = "(contains ";

#[derive(Debug, PartialEq, Eq)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> From<&'a str> for Food<'a> {
    fn from(string: &'a str) -> Self {
        let contains_index = string.find(CONTAINS).unwrap();
        let ingredients_str = &string[..contains_index];
        let ingredients = ingredients_str.trim().split(' ').collect();

        let start = contains_index + CONTAINS.len();
        let end = string.len() - 1;
        let allergens_str = &string[start..end];
        let allergens = allergens_str.split(", ").collect();

        Self {
            ingredients,
            allergens,
        }
    }
}

fn main() {
    let mut foods: Vec<Food> = INPUT.lines().map(Food::from).collect();

    let all_allergens: HashSet<&str> = foods.iter().fold(Default::default(), |acc, elm| {
        acc.union(&elm.allergens).copied().collect()
    });

    let all_ingredients: HashSet<&str> = foods.iter().fold(Default::default(), |acc, elm| {
        acc.union(&elm.ingredients).copied().collect()
    });

    // Map of allergen => ingredient
    let mut allergen_map: HashMap<&str, &str> = HashMap::with_capacity(all_allergens.len());

    while allergen_map.len() < all_allergens.len() {
        for allergen in all_allergens.iter() {
            let intersection: HashSet<&str> = foods
                .iter()
                .filter(|food| food.allergens.contains(allergen))
                .fold(all_ingredients.clone(), |acc, food| {
                    acc.intersection(&food.ingredients).copied().collect()
                });

            if intersection.len() == 1 {
                let ingredient = intersection.into_iter().next().unwrap();
                allergen_map.insert(allergen, ingredient);

                for food in foods.iter_mut() {
                    food.ingredients.remove(ingredient);
                }
            }
        }
    }

    let count = foods.iter().flat_map(|food| &food.ingredients).count();
    println!("part1 = {}", count);

    let mut pairs: Vec<(&str, &str)> = allergen_map.iter().map(|(a, b)| (*a, *b)).collect();
    pairs.sort_unstable();

    let ingredients: Vec<&str> = pairs
        .iter()
        .map(|(_ingredient, allergen)| *allergen)
        .collect();

    println!("part2 = {}", ingredients.join(","));
}
