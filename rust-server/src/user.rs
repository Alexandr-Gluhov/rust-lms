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
    pub salt: String,
    pub group_id: i32,
}

impl User {
    pub async fn register(
        client: &tokio_postgres::Client,
        name: String,
        surname: String,
        patronymic: Option<String>,
        email: String,
        password: String,
        group_id: i32,
    ) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        let patronymic = match patronymic {
            Some(patr) => patr,
            None => String::from("NULL"),
        };

        if let Err(Error) = client.execute("INSERT INTO users (surname, name, patronymic, email, password, group_id) VALUES ($1, $2, $3, $4, $5, $6)", &[&surname, &name, &patronymic, &email, &password_hash, &group_id]).await {
            println!("{:?}", Error);
            return Ok(String::from("Error while creating user"));
        }
        Ok(String::from("Success"))
    }

    pub async fn get_by_email(
        client: &tokio_postgres::Client,
        email: &str,
    ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
        client
            .query("Select * from users WHERE email = $1", &[&email])
            .await
    }

    pub async fn login(client: &tokio_postgres::Client, email: &str, password: &str) {
        match Self::get_by_email(client, email).await {
            Ok(res) => {
                println!("I'm in");
                let user_password: String = res[0].get("password");
                let parsed_hash = PasswordHash::new(&user_password).unwrap();
                if Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    println!("Holly molly!");
                };
            },
            Err(E) => {
                println!("{:?}", E);
            },
        };
    }
}
