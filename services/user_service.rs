// services/user_service.rs

use crate::repository_interface::{User, UserRepository};

pub struct UserService<'a> {
    pub repository: &'a mut dyn UserRepository,
}

impl<'a> UserService<'a> {
    pub fn new(repository: &'a mut dyn UserRepository) -> Self {
        Self { repository }
    }

    pub fn create_user(&mut self, user: User) {
        self.repository.save(user.user_id.clone(), user);
    }

    pub fn get_user_by_id(&self, user_id: &str) -> Option<&User> {
        self.repository.find_by_id(&user_id.to_string())
    }

    pub fn get_all_users(&self) -> Vec<&User> {
        self.repository.find_all()
    }

    pub fn delete_user(&mut self, user_id: &str) {
        self.repository.delete(&user_id.to_string());
    }
}

