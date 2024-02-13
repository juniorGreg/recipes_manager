use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Surreal, Result};

use crate::models::*;

const TABLE: &str = "recipes";

pub struct Database {
    db: Surreal<Client>
}

impl Database {

    pub async fn new() -> Result<Self>{
       let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

       db.signin(Root {
            username: "root",
            password: "root"
       }).await?;

       db.use_ns("recipes").use_db("recipes").await?;
       Ok(Database{ db })
    }
    
    pub async fn get_recipe(&self, id: &str) -> Option<RecipeWithId> {
       let recipe: Option<RecipeWithId> = self.db.select((TABLE, id)).await.unwrap(); 
       recipe
    }

    pub async fn get_all_recipes(&self) -> Vec<RecipeWithId> {
        let recipes: Vec<RecipeWithId> = self.db.select(TABLE).await.unwrap();
        recipes
    }

    pub async fn delete_recipe(&self, id: &str) {
        let recipe: Option<Recipe> = self.db.delete(("recipes", id)).await.unwrap();
        dbg!(recipe);
    }

    pub async fn create_recipe(&self, recipe: &Recipe) {
        let created: Vec<Record> = self.db.create(TABLE).content(recipe).await.unwrap();
        dbg!(created);    
    }
    
}
