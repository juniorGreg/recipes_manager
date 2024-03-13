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
    db.recipe.create(&recipe.into_inner()).await;
    Redirect::to(uri!("/recipes"))
}

#[get("/modify/<id>")]
async fn modify_recipe_form(db: &State<Database>, id: &str) -> Template {
    let recipe = db.recipe.get(id).await;
    Template::render("recipes/add_recipe", context!{recipe, id, modify: true})
}

#[get("/")]
async fn recipes(db: &State<Database>) -> Template {
    let recipes: Vec<RecipeWithId> = db.recipe.get_all().await;
    let default_recipe = Recipe{title: "Ajouter une recette".to_string(), ..Default::default()};
    Template::render("recipes/recipes", context!{recipes: recipes, recipe: default_recipe})
}

#[delete("/<id>")]
async fn delete_recipe(db: &State<Database>, id: &str) -> Redirect {
    db.recipe.delete(id).await;
    Redirect::to(uri!("/recipes"))
}

#[put("/<id>", data ="<recipe>")]
async fn modify_recipe(db: &State<Database>, recipe: Form<Recipe>, id: &str) -> Redirect {
    db.recipe.modify(id, &recipe.into_inner()).await;
    Redirect::to(uri!("/recipes"))
}

#[post("/ingredient", data = "<ingredient>")]
fn new_ingredient(ingredient: Form<Ingredient>) -> Template {
    Template::render("recipes/ingredient", context!{
        ingredient: &ingredient.into_inner()
    })
}

#[post("/preparation_step", data = "<preparation_step>")]
fn new_preparation_step(preparation_step: Form<PreparationStep>) -> Template {
    Template::render("recipes/preparation_step", context!{preparation_step: preparation_step.preparation_step.to_string()})
}

#[get("/pdf/<id>")]
async fn get_recipe_pdf(db: &State<Database>, id: &str) -> IncludedPdf {
    let recipe = db.recipe.get(id).await.unwrap();
    let pdf_bytes = recipes_printer::print_pdf_recipe(&recipe).await;
    IncludedPdf(pdf_bytes)
}

pub async fn get_routes() -> Vec<Route> {
   routes![new_recipe, recipes, delete_recipe, modify_recipe, modify_recipe_form,
           new_ingredient, new_preparation_step, get_recipe_pdf] 
}
