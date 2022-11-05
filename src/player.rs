use crate::roles::{Role, Roles};

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub roles: Roles,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            roles: Roles::new(
                Role::RatingNonApplicable,
                Role::RatingNonApplicable,
                Role::RatingNonApplicable,
            )
            .unwrap(),
        }
    }
}
