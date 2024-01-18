#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_debug() {
        let user = User {
            id: Some(42),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };

        assert_eq!(user.id, Some(42));
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
    }
}
