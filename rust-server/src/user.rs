use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub patronymic: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub group_id: i32,
}

impl User {
    pub async fn register(
        client: &tokio_postgres::Client,
        name: &str,
        surname: &str,
        patronymic: Option<&str>,
        email: &str,
        password: &str,
        group_id: i32,
    ) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        let patronymic = patronymic.unwrap_or_else(|| "NULL");

        if let Err(e) = client.execute("INSERT INTO users (surname, name, patronymic, email, password, group_id) VALUES ($1, $2, $3, $4, $5, $6)", &[&surname, &name, &patronymic, &email, &password_hash, &group_id]).await {
            println!("{:?}", e);
            return Ok(String::from("Error while creating user"));
        }
        Ok(String::from("Success"))
    }

    pub async fn get_by_email(
        client: &tokio_postgres::Client,
        email: &str,
    ) -> Option<tokio_postgres::Row> {
        client
            .query("Select * from users WHERE email = $1", &[&email])
            .await
            .ok()?
            .into_iter()
            .next()
    }

    pub async fn get_by_id(
        client: &tokio_postgres::Client,
        id: i32,
    ) -> Option<tokio_postgres::Row> {
        client
            .query("select * from users where id = $1", &[&id])
            .await
            .ok()?
            .into_iter()
            .next()
    }
    pub async fn approve_login(
        client: &tokio_postgres::Client,
        email: &str,
        password: &str,
    ) -> bool {
        if let Some(res) = Self::get_by_email(client, email).await {
            if let Ok(user_password) = res.try_get::<_, String>("password") {
                if let Ok(parsed_hash) = PasswordHash::new(&user_password) {
                    return Argon2::default()
                        .verify_password(password.as_bytes(), &parsed_hash)
                        .is_ok();
                }
            }
        }
        false
    }
}
