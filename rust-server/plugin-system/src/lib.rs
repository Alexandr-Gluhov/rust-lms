use mlua::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio_postgres::Client;

pub struct AppState {
    pub client: Client,
}

pub struct PluginSystem {}

impl PluginSystem {
    pub async fn execute(client: Arc<AppState>, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lua = Lua::new();

        let utils = lua.create_table()?;

        let create_publication = create_publication_builder(client.clone());

        utils.set(
            "create_publication",
            lua.create_async_function(create_publication)?,
        )?;

        lua.globals().set("utils", utils)?;

        println!("Starting Lua script execution");
        lua.load(script).exec_async().await?;
        println!("Lua script execution finished");

        Ok(())
    }
}

fn create_publication_builder(
    client: Arc<AppState>,
) -> impl Fn(Lua, (String, i32)) -> Pin<Box<dyn Future<Output = mlua::Result<()>> + Send>>
       + Send
       + Sync
       + 'static {
    move |_lua, (publication, institute_id)| {
        println!(
            "Calling create_publication with: {}, {}",
            &publication, institute_id
        );
        let client = client.clone();
        Box::pin(async move {
            println!("Executing query...");
            client
                .client
                .query(
                    "INSERT INTO news (text, institute_id) VALUES ($1, $2)",
                    &[&publication, &institute_id],
                )
                .await
                .map_err(|err| {
                    eprintln!("Database error: {}", err);
                    mlua::Error::external(err)
                })?;
            println!("Query executed successfully!");
            Ok(())
        })
    }
}
