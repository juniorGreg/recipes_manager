#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};
use rocket::form::Form;
use rocket::State;

mod database;
mod models;
mod recipes_printer;

use models::*;
use database::Database;

#[derive(Responder)]
#[response(status = 200, content_type = "pdf")]
struct IncludedPdf(Vec<u8>);



#[post("/recipe", data = "<recipe>")]
async fn new_recipe(db: &State<Database>, recipe: Form<Recipe>) -> String {
    let response = format!("Add Recipe: {}, Preps: {:?}, ingredients: {:?}", recipe.title, recipe.preparation_steps, recipe.ingredients);
    db.create_recipe(&recipe.into_inner()).await;
    response
}

#[get("/recipes")]
async fn recipes(db: &State<Database>) -> Template {
    let recipes: Vec<RecipeWithId> = db.get_all_recipes().await;
    Template::render("recipes", context!{recipes: recipes})
}

#[delete("/recipes/<id>")]
async fn delete_recipe(db: &State<Database>, id: &str) {
    db.delete_recipe(id).await;
}

#[get("/add_recipe")]
fn add_recipe() -> Template {
    Template::render("add_recipe", context!{
        content: "Hello",
    })
}

#[post("/ingredient", data = "<ingredient>")]
fn new_ingredient(ingredient: Form<Ingredient>) -> Template {
    //format!("Nom: {}, Quantité: {}, Unité: {}", ingredient.ingredient_name, ingredient.ingredient_quantity, ingredient.ingredient_unit)
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
    let recipe = db.get_recipe(id).await.unwrap();
    let pdf_bytes = recipes_printer::print_pdf_recipe(&recipe).await;
    IncludedPdf(pdf_bytes)
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{
    })
}

#[launch]
async fn rocket() -> _ {
    let db = Database::new().await.unwrap();
    rocket::build()
        .mount("/", routes![index, new_recipe, recipes, add_recipe, delete_recipe, new_ingredient, new_preparation_step, get_recipe_pdf])
        .mount("/public", FileServer::from(relative!("static")))
        .manage(db)
        .attach(Template::fairing())
}
