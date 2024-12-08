use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

impl Group {
    pub async fn get_all(client: &tokio_postgres::Client) -> Vec<tokio_postgres::Row> {
        client.query("SELECT * FROM groups", &[]).await.unwrap()
    }
}