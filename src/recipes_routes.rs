use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::Route;
use rocket::State;

use crate::models::*;
use crate::Database;
use crate::IncludedPdf;
use crate::recipes_printer;

#[post("/new", data = "<recipe>")]
async fn new_recipe(db: &State<Database>, recipe: Form<Recipe>) -> Redirect {
    let response = format!("Add Recipe: {}, Preps: {:?}, ingredients: {:?}", recipe.title, recipe.preparation_steps, recipe.ingredients);
    db.recipe.create(&recipe.into_inner()).await;
    Redirect::to(uri!("/recipes"))
}

#[get("/")]
async fn recipes(db: &State<Database>) -> Template {
    let recipes: Vec<RecipeWithId> = db.recipe.get_all().await;
    Template::render("recipes", context!{recipes: recipes})
}

#[delete("/<id>")]
async fn delete_recipe(db: &State<Database>, id: &str) -> Redirect {
    db.recipe.delete(id).await;
    Redirect::to(uri!("/recipes"))
}

#[post("/ingredient", data = "<ingredient>")]
fn new_ingredient(ingredient: Form<Ingredient>) -> Template {
    Template::render("ingredient", context!{
        ingredient_name: ingredient.ingredient_name.to_string(),
        ingredient_quantity: ingredient.ingredient_quantity.to_string(),
        ingredient_unit: ingredient.ingredient_unit.to_string()
    })
}

#[post("/preparation_step", data = "<preparation_step>")]
fn new_preparation_step(preparation_step: Form<PreparationStep>) -> Template {
    Template::render("preparation_step", context!{preparation_step: preparation_step.preparation_step.to_string()})
}

#[get("/pdf/<id>")]
async fn get_recipe_pdf(db: &State<Database>, id: &str) -> IncludedPdf {
    let recipe = db.recipe.get(id).await.unwrap();
    let pdf_bytes = recipes_printer::print_pdf_recipe(&recipe).await;
    IncludedPdf(pdf_bytes)
}

pub async fn get_routes() -> Vec<Route> {
   routes![new_recipe, recipes, delete_recipe, 
           new_ingredient, new_preparation_step, get_recipe_pdf] 
}
