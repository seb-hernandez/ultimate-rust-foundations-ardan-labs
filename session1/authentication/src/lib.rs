use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::stdin;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

pub fn get_users_vec() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ]
}

pub fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "bob".to_string(),
        User::new("bob", "password", LoginRole::User),
    );

    users
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load file.
        let users_json = fs::read_to_string(users_path).unwrap();
        serde_json::from_str(&users_json).unwrap()
    } else {
        // Create file.
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        fs::write(users_path, users_json).unwrap();
        users
    }
}

pub fn save_users(users: HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    fs::write(users_path, users_json).unwrap();
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_password(password);

    let users = get_users();

    if let Some(user) = users.get(&username) {
        if user.password == password {
            Some(LoginAction::Granted(user.role.clone()))
        } else {
            Some(LoginAction::Denied)
        }
    } else {
        None
    }

    // match users.iter().find(|user| user.username == username) {
    //     Some(user) => {
    //         if user.password == password {
    //             Some(LoginAction::Granted(user.role.clone()))
    //         } else {
    //             Some(LoginAction::Denied)
    //         }
    //     }
    //     None => None,
    // }

    // With Vec
    // if let Some(user) = users.iter().find(|user| user.username == username) {
    //     if user.password == password {
    //         Some(LoginAction::Granted(user.role.clone()))
    //     } else {
    //         Some(LoginAction::Denied)
    //     }
    // } else {
    //     None
    // }
}

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha224::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}

pub fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user("Lorem"), "Hello Lorem");
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("admin", "not-password"), Some(LoginAction::Denied));
        assert_eq!(login("not-admin", "not-password"), None);
    }

    #[test]
    fn test_vectors() {
        let users = get_users_vec();
        let admins: Vec<_> = users
            .iter()
            .filter(|u| u.role == LoginRole::Admin)
            .collect();

        assert_eq!(users.len(), 2);
        assert_eq!(admins.len(), 1);
    }
}
