use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::Route;


#[derive(FromForm)]
struct TemperaturesFiles<'f> { 
   kitchen_fridge_file: TempFile<'f>,
}

#[post("/temp_files", data = "<temperatures_files>")]
pub async fn upload_temperature_files(temperatures_files: Form<TemperaturesFiles<'_>>) -> &'static str {
    "upload successful"
}

#[get("/")]
pub async fn temp_fridge() -> Template {
    Template::render("temp_fridge/temp_fridge", context!{})
}

pub async fn get_routes() -> Vec<Route> {
   routes![temp_fridge, upload_temperature_files] 
}
