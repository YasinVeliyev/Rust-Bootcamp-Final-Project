pub struct User {
    name: String,
    password: String,
    pub role: Role,
}

#[derive(Debug, PartialEq)]
pub enum Role {
    InventoryManager,
    Other,
}

impl User {
    pub fn new(name: &str, password: &str, role: Role) -> Self {
        Self {
            name: name.to_owned(),
            password: password.to_owned(),
            role: role,
        }
    }

    pub fn create_user() -> Self {
        let mut name = String::new();
        let mut password = String::new();

        println!("Enter User name");
        std::io::stdin().read_line(&mut name);
        println!("Enter User password");

        std::io::stdin().read_line(&mut password);
        Self::new(name.trim(), password.trim(), Role::Other)
    }
}
