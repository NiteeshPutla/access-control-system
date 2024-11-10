
mod models;
use models::{AccessControl, User, Role, Permission};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let access_control = Arc::new(Mutex::new(AccessControl::new()));

    let admin_role = Role {
        id: "admin".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    
    let read_permission = Permission {
        id: "read".to_string(),
        name: "Read Access".to_string(),
    };
    
    let user = User {
        id: "user1".to_string(),
        role_id: "admin".to_string(),
    };

    {
        let mut ac = access_control.lock().unwrap();
        ac.add_role(admin_role);
        ac.add_permission(read_permission);
        ac.add_user(user);
    }

    let ac_clone = Arc::clone(&access_control);
    thread::spawn(move || {
        let ac = ac_clone.lock().unwrap();
        let has_access = ac.check_permission("user1", "read");
        println!("User has read access: {}", has_access);
    }).join().unwrap();
}