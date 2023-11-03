pub struct User {
    name: String,
    password: String,
    role: Role,
}

pub enum Role {
    InventoryManager,
    Other,
}
