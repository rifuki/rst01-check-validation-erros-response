#[derive(sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub username: String,
    pub password: String
}

lazy_static::lazy_static! {
    static ref RE_USERNAME: regex::Regex = regex::Regex::new(r"^[0-9a-zA-Z]{3,}$").unwrap();
}

#[derive(serde::Deserialize, validator::Validate, Clone)]
pub struct CreateUser {
    #[validate(
        length(
            min = 6,
            max = 20,
            message = "fails validation - must be 3-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String,
    #[validate(
        length(
            min = 8,
            max = 72,
            message = "fails validation - must be 8-72 character long"
        )
    )]
    pub password: String
}