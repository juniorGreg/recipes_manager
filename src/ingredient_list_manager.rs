use rocket::serde::{Serialize, Deserialize};
use crate::models::*;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct IngredientList {
    pub recipe_name: String,
    pub ingredients: Vec<String>,
    pub allergens: Vec<String>,
}

async fn get_allergens(ingredients: &Vec<String>, allergens: &Vec<AllergenWithId>) -> Vec<String> {
    let mut allergens_list: Vec<String> = Vec::new();
    let ingredients_set: HashSet<String> = ingredients.into_iter().map(|s| s.to_lowercase()).collect(); 
    for allergen in allergens {
        let allergens_set: HashSet<String> = allergen.ingredients.iter().map(|s| s.to_lowercase()).collect();
        let intersection: Vec<&String> = ingredients_set.intersection(&allergens_set).into_iter().collect();

        if intersection.len() > 0
        {
            allergens_list.push(allergen.name.to_owned())
        }
    }
    allergens_list
}

pub async fn get_ingredients_list(recipes: &Vec<RecipeWithId>, allergens: &Vec<AllergenWithId>) -> Vec<IngredientList> {
    let mut ingredients_list: Vec<IngredientList> = Vec::new();
    for recipe in recipes {
        let ingredients: Vec<String> = recipe.ingredients.iter().map(|i| i.ingredient_name.to_owned() ).collect();
        let allergens_list: Vec<String> = get_allergens(&ingredients, &allergens).await;

        let ingredient_list = IngredientList {
            recipe_name: recipe.title.to_owned(),
            ingredients: ingredients,
            allergens: allergens_list,
        };
        ingredients_list.push(ingredient_list);
    }
    ingredients_list
}
