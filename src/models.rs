use rocket::serde::{Serialize, Deserialize};

use surrealdb::sql::Thing;


#[derive(FromForm, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub title: String,
    pub ingredients: Vec<Ingredient>,
    pub preparation_steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RecipeWithId {
    pub id: Thing,
    pub title: String,
    pub preparation_steps: Vec<String>,
    pub ingredients: Vec<Ingredient>,
}

#[derive(FromForm, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Ingredient {
    pub ingredient_name: String,
    pub ingredient_quantity: String,
    pub ingredient_unit: String,
}

#[derive(FromForm, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PreparationStep {
    pub preparation_step: String,
}

#[derive(FromForm, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Allergen {
    pub name: String,
    pub ingredients: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AllergenWithId {
    pub id: Thing,
    pub name: String,
    pub ingredients: Vec<String>,
}

#[derive(FromForm)]
pub struct AllergenIngredient {
    pub ingredient: String
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}
