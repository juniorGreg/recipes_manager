#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};
use rocket::form::Form;
use rocket::State;
use rocket::response::Redirect;

mod database;
mod models;
mod ingredient_list_manager;
mod recipes_printer;
mod recipes_routes;

use models::*;
use database::Database;
use ingredient_list_manager::*;

#[derive(Responder)]
#[response(status = 200, content_type = "pdf")]
struct IncludedPdf(Vec<u8>);

#[get("/ingredients_list")]
async fn ingredients_list(db: &State<Database>) -> Template {
    let allergens = db.allergen.get_all().await;
    let recipes = db.recipe.get_all().await;
    let ingredients_list = get_ingredients_list(&recipes, &allergens).await;
    Template::render("ingredients_list", context!{
        allergens: &allergens,
        ingredients_list: &ingredients_list
    })
}

#[post("/ingredients_list/allergens", data = "<allergen>")]
async fn add_new_allergen(db: &State<Database>, allergen: Form<Allergen>) -> Redirect {
    db.allergen.create(&allergen.into_inner()).await;
    Redirect::to(uri!(ingredients_list))
}

#[delete("/ingredients_list/allergens/<id>")]
async fn remove_allergen(db: &State<Database>, id: &str) -> Redirect {
    db.allergen.delete(id).await;

    Redirect::to(uri!(ingredients_list))
}

#[post("/ingredients_list/allergens/ingredient", data = "<allergen_ingredient>")]
fn new_allergen_ingredient(allergen_ingredient: Form<AllergenIngredient>) -> Template {
    Template::render("allergen_ingredient", context!{
        ingredient: allergen_ingredient.ingredient.to_string(),
    })
}

#[put("/ingredients_list/allergens/ingredient/<id>", data = "<allergen_ingredient>")]
async fn add_allergen_ingredient(db: &State<Database>, 
                           allergen_ingredient: Form<AllergenIngredient>, 
                           id: &str) -> Redirect {
    let allergen_with_id = db.allergen.get(id).await.unwrap();
    let mut ingredients = allergen_with_id.ingredients.to_owned();
    ingredients.push(allergen_ingredient.ingredient.to_owned());
    let allergen = Allergen {
        name: allergen_with_id.name,
        ingredients: ingredients
    };

    db.allergen.modify(id, &allergen).await;
    
    Redirect::to(uri!(ingredients_list))

}

#[get("/ingredients_list/pdf")]
async fn get_ingredients_list_pdf(db: &State<Database>) -> IncludedPdf {
    let allergens = db.allergen.get_all().await;
    let recipes = db.recipe.get_all().await;
    let ingredients_list = get_ingredients_list(&recipes, &allergens).await;
    let pdf_bytes = recipes_printer::print_ingredients_lists(&ingredients_list).await;
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
        .mount("/", routes![index, 
                            ingredients_list, add_new_allergen, new_allergen_ingredient, 
                            get_ingredients_list_pdf, remove_allergen, add_allergen_ingredient])
        .mount("/recipes", recipes_routes::get_routes().await)
        .mount("/public", FileServer::from(relative!("static")))
        .manage(db)
        .attach(Template::fairing())
}
