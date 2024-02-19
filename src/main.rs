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

#[post("/recipes/new", data = "<recipe>")]
async fn new_recipe(db: &State<Database>, recipe: Form<Recipe>) -> String {
    let response = format!("Add Recipe: {}, Preps: {:?}, ingredients: {:?}", recipe.title, recipe.preparation_steps, recipe.ingredients);
    db.recipe.create(&recipe.into_inner()).await;
    response
}

#[get("/recipes")]
async fn recipes(db: &State<Database>) -> Template {
    let recipes: Vec<RecipeWithId> = db.recipe.get_all().await;
    Template::render("recipes", context!{recipes: recipes})
}

#[delete("/recipes/<id>")]
async fn delete_recipe(db: &State<Database>, id: &str) {
    db.recipe.delete(id).await;
}

#[get("/recipes/add")]
fn add_recipe() -> Template {
    Template::render("add_recipe", context!{
        content: "Hello",
    })
}

#[post("/recipes/ingredient", data = "<ingredient>")]
fn new_ingredient(ingredient: Form<Ingredient>) -> Template {
    //format!("Nom: {}, Quantité: {}, Unité: {}", ingredient.ingredient_name, ingredient.ingredient_quantity, ingredient.ingredient_unit)
    Template::render("ingredient", context!{
        ingredient_name: ingredient.ingredient_name.to_string(),
        ingredient_quantity: ingredient.ingredient_quantity.to_string(),
        ingredient_unit: ingredient.ingredient_unit.to_string()
    })
}

#[post("/recipes/preparation_step", data = "<preparation_step>")]
fn new_preparation_step(preparation_step: Form<PreparationStep>) -> Template {
    Template::render("preparation_step", context!{preparation_step: preparation_step.preparation_step.to_string()})
}

#[get("/recipes/pdf/<id>")]
async fn get_recipe_pdf(db: &State<Database>, id: &str) -> IncludedPdf {
    let recipe = db.recipe.get(id).await.unwrap();
    let pdf_bytes = recipes_printer::print_pdf_recipe(&recipe).await;
    IncludedPdf(pdf_bytes)
}

#[get("/ingredients_list")]
async fn ingredients_list(db: &State<Database>) -> Template {
    let allergens = db.allergen.get_all().await;
    Template::render("ingredients_list", context!{
        allergens: allergens
    })
}

#[post("/ingredients_list/allergens", data = "<allergen>")]
async fn add_new_allergen(db: &State<Database>, allergen: Form<Allergen>) {
    db.allergen.create(&allergen.into_inner()).await;
}

#[post("/ingredients_list/allergens/ingredient", data = "<allergen_ingredient>")]
fn new_allergen_ingredient(allergen_ingredient: Form<AllergenIngredient>) -> Template {
    Template::render("allergen_ingredient", context!{
        ingredient: allergen_ingredient.ingredient.to_string(),
    })
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
        .mount("/", routes![index, new_recipe, recipes, add_recipe, delete_recipe, 
                            new_ingredient, new_preparation_step, get_recipe_pdf, 
                            ingredients_list, add_new_allergen, new_allergen_ingredient])
        .mount("/public", FileServer::from(relative!("static")))
        .manage(db)
        .attach(Template::fairing())
}
