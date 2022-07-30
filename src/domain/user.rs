#[derive(Debug, Clone, Copy)]
pub struct UserId(pub u32);

pub type UserIds = Vec<UserId>;

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
