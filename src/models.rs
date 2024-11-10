use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub role_id: String,
}

#[derive(Debug, Clone)]
pub struct Role {
    pub id: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Permission {
    pub id: String,
    pub name: String,
}

pub struct AccessControl {
    users: HashMap<String, User>,
    roles: HashMap<String, Role>,
    permissions: HashMap<String, Permission>,
}

impl AccessControl {
    pub fn new() -> Self {
        AccessControl {
            users: HashMap::new(),
            roles: HashMap::new(),
            permissions: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role.id.clone(), role);
    }

    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission.id.clone(), permission);
    }

    pub fn check_permission(&self, user_id: &str, permission_id: &str) -> bool {
        if let Some(user) = self.users.get(user_id) {
            if let Some(role) = self.roles.get(&user.role_id) {
                return role.permissions.contains(&permission_id.to_string());
            }
        }
        false
    }
}
