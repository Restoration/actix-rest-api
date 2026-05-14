#[derive(Debug, Clone, Copy)]
pub struct UserId(pub u32);

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

pub type Users = Vec<User>;

impl User {
    pub fn new(id: UserId, name: impl Into<String>) -> Self {
        User {
            id,
            name: name.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_user_with_str() {
        let user = User::new(UserId(1), "Alice");
        assert_eq!(user.id.0, 1);
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn new_user_with_string() {
        let user = User::new(UserId(42), String::from("Bob"));
        assert_eq!(user.id.0, 42);
        assert_eq!(user.name, "Bob");
    }

    #[test]
    fn user_id_is_copy() {
        let id = UserId(10);
        let id2 = id;
        assert_eq!(id.0, id2.0);
    }

    #[test]
    fn user_is_clone() {
        let user = User::new(UserId(1), "Alice");
        let cloned = user.clone();
        assert_eq!(cloned.id.0, 1);
        assert_eq!(cloned.name, "Alice");
    }
}
