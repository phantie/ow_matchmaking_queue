use crate::rating::Rating;

pub trait ReduceRoles {
    fn reduced_roles_lobby(&self) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Roles {
    // at least one of the fields is not NoSelect
    pub tank: Role,
    pub damage: Role,
    pub support: Role,
}

impl Roles {
    pub fn new(tank: Role, damage: Role, support: Role) -> Option<Self> {
        // filter out views that don't select any role
        if [tank, damage, support]
            .into_iter()
            .all(|role| !role.selected())
        {
            None
        } else {
            Some(Self {
                tank,
                damage,
                support,
            })
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Role {
    // public rank; ranked game
    Ranked(Rating),
    // hidden rank, when requirements are not met for placements; ranked game
    Unranked(Rating),
    RatingNonApplicable,
    NoSelect,
}

impl Role {
    pub fn selected(&self) -> bool {
        !matches!(self, Role::NoSelect)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn no_roles_selected() {
        assert!(Roles::new(Role::NoSelect, Role::NoSelect, Role::NoSelect).is_none());
    }
}
