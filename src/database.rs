use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Surreal, Result};
use std::marker::PhantomData;
use crate::models::*;
use std::fmt::Debug;
use rocket::serde::{Serialize, Deserialize};

pub struct Database {
    pub recipe: Resource<Recipe, RecipeWithId>,
    pub allergen: Resource<Allergen, AllergenWithId>
}

pub struct Resource<T, I> {
    db: Surreal<Client>,
    table: String, 
    _resource: PhantomData<T>,
    _resource_with_id: PhantomData<I>

}

impl<T: Debug + for<'de> Deserialize<'de> + Serialize, I: Debug + for<'de> Deserialize<'de>> Resource<T, I> {
    pub async fn new(db: &Surreal<Client>, table: String) -> Self {
        Self{db: db.clone(), table, _resource: PhantomData, _resource_with_id: PhantomData}
    }

    pub async fn get(&self, id: &str) -> Option<I> {
       let resource: Option<I> = self.db.select((&self.table, id)).await.unwrap(); 
       resource
    }

    pub async fn get_all(&self) -> Vec<I> {
        let resources: Vec<I> = self.db.select(&self.table).await.unwrap();
        resources
    }

    pub async fn delete(&self, id: &str) {
        let resource: Option<T> = self.db.delete((&self.table, id)).await.unwrap();
        dbg!(resource);
    }

    pub async fn modify(&self, id: &str, resource: &T) {
        let resource_modified: Option<I>  = self.db.update((&self.table, id)).content(resource).await.unwrap();
        dbg!(resource_modified);

    }

    pub async fn create(&self, resource: &T) {
        let created: Vec<Record> = self.db.create(&self.table).content(resource).await.unwrap();
        dbg!(created);    
    }
}

impl Database {

    pub async fn new() -> Result<Self>{
       let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

       db.signin(Root {
            username: "root",
            password: "root"
       }).await?;

       db.use_ns("recipes").use_db("recipes").await?;
        
       let recipe = Resource::new(&db, "recipes".to_owned()).await;
       let allergen = Resource::new(&db, "allergens".to_owned()).await;

       Ok(Database{ recipe, allergen })
    }
}
