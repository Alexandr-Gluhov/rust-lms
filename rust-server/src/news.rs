use tokio_postgres::{Client, Row};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct News {
    pub id: i32,
    pub text: String,
}

impl News {
    pub async fn get_by_user_id(client: &Client, user_id: i32) -> Vec<Row> {
        client.query("
        SELECT n.id, n.text
        FROM news n
        INNER JOIN institutes i
        ON n.institute_id = i.id
        INNER JOIN groups g
        ON g.institute_id = i.id
        INNER JOIN users u
        ON u.group_id = g.id
        WHERE u.id = $1", &[&user_id])
        .await
        .unwrap()
    }
}