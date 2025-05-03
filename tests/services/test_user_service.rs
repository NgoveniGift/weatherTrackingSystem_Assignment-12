
// tests/services/test_user_service.rs

use crate::services::user_service::UserService;
use crate::repository_interface::{User, Repository, UserRepository};
use std::collections::HashMap;

struct MockUserRepository {
    store: HashMap<String, User>,
}

impl MockUserRepository {
    fn new() -> Self {
        Self { store: HashMap::new() }
    }
}

impl Repository<String, User> for MockUserRepository {
    fn save(&mut self, id: String, entity: User) {
        self.store.insert(id, entity);
    }

    fn find_by_id(&self, id: &String) -> Option<&User> {
        self.store.get(id)
    }

    fn find_all(&self) -> Vec<&User> {
        self.store.values().collect()
    }

    fn delete(&mut self, id: &String) {
        self.store.remove(id);
    }
}

impl UserRepository for MockUserRepository {}

#[test]
fn test_user_service_crud() {
    let mut mock_repo = MockUserRepository::new();
    let mut service = UserService::new(&mut mock_repo);

    let user = User {
        user_id: "u1".into(),
        name: "Alice".into(),
        email: "alice@example.com".into(),
    };

    service.create_user(user.clone());
    assert_eq!(service.get_user_by_id("u1"), Some(&user));

    let all_users = service.get_all_users();
    assert_eq!(all_users.len(), 1);

    service.delete_user("u1");
    assert!(service.get_user_by_id("u1").is_none());
}
