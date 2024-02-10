use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Surreal, Result};

pub async fn get_surreal_connection() -> surrealdb::Result<Surreal<Client>> {
   let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

   db.signin(Root {
        username: "root",
        password: "root"
   }).await?;

   db.use_ns("recipes").use_db("recipes").await?;
   Ok(db)
}

