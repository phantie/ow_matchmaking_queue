#![allow(unused_variables, dead_code)]

use std::vec;

use itertools::iproduct;

pub struct Queue {}

impl Queue {
    fn enter(&mut self, lobby: &Lobby) {}
}

pub struct NoLimitsGame {}

impl Game for NoLimitsGame {
    fn valid_lobby(&self, lobby: &Lobby) -> bool {
        true
    }

    fn reduced_roles_lobby(&self, lobby: &Lobby) -> Lobby {
        lobby.clone()
    }
}

pub struct OneTwoTwoGame {}

impl Game for OneTwoTwoGame {
    fn valid_lobby(&self, lobby: &Lobby) -> bool {
        let players = &lobby.players;

        if lobby.player_count() == 0 {
            unreachable!()
        } else if lobby.player_count() == 1 {
            return true;
        } else if lobby.player_count() > 5 {
            return false;
        }

        enum Role {
            T,
            D,
            S,
        }

        let mut rows = vec![];

        for player in players {
            let roles = player.roles;

            let (tank, damage, support) = (roles.tank, roles.damage, roles.support);

            let row = [tank, damage, support]
                .into_iter()
                .enumerate()
                .filter(|(i, role)| role.selected())
                .map(|(i, role)| i as u32)
                .collect::<Vec<_>>();

            rows.push(row.into_iter().peekable());
        }

        let mut combs: Vec<Vec<u32>> = vec![];

        let mut indeces = vec![0; lobby.player_count()];

        dbg!(&rows);

        unimplemented!();
    }

    fn reduced_roles_lobby(&self, lobby: &Lobby) -> Lobby {
        unimplemented!()
    }
}

pub trait Game {
    fn valid_lobby(&self, lobby: &Lobby) -> bool;
    fn reduced_roles_lobby(&self, lobby: &Lobby) -> Lobby;
}

#[derive(Clone)]
pub struct Lobby {
    // len at least one
    players: Vec<Player>,
    rating: bool,
}

impl Lobby {
    fn player_count(&self) -> usize {
        self.players.len()
    }

    fn empty(&self) -> bool {
        self.player_count() == 0
    }

    fn solo(&self) -> bool {
        self.player_count() == 1
    }

    pub fn new(players: Vec<Player>) -> Option<Self> {
        // views on players in a lobby must
        // either all or none take into account player ratings

        #[derive(PartialEq, Eq)]
        enum Rating {
            Applicable,
            NotApplicable,
        }

        const APPLICABLE: Option<Rating> = Some(Rating::Applicable);
        const NOT_APPLICABLE: Option<Rating> = Some(Rating::NotApplicable);

        let mut rating: Option<Rating> = None;

        for player in &players {
            for role in &[player.roles.tank, player.roles.damage, player.roles.support] {
                match role {
                    Role::Ranked(_) | Role::Unranked(_) => match rating {
                        None => rating = APPLICABLE,
                        APPLICABLE => (),
                        NOT_APPLICABLE => return None,
                    },
                    Role::RatingNonApplicable => match rating {
                        None => rating = NOT_APPLICABLE,
                        APPLICABLE => return None,
                        NOT_APPLICABLE => (),
                    },
                    Role::NoSelect => (),
                }
            }
        }

        Some(Self {
            players,
            rating: match rating {
                None => return None, // filter out empty lobbies
                APPLICABLE => true,
                NOT_APPLICABLE => false,
            },
        })
    }
}

#[derive(Copy, Clone)]
pub struct Player {
    pub roles: Roles,
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum Role {
    // public rank; ranked game
    Ranked(Rating),
    // hidden rank, when requirements are not met for placements; ranked game
    Unranked(Rating),
    RatingNonApplicable,
    NoSelect,
}

impl Role {
    fn selected(&self) -> bool {
        !matches!(self, Role::NoSelect)
    }
}

#[derive(Copy, Clone)]
pub struct Rating(u32);

impl Rating {
    pub fn new(value: u32) -> Option<Self> {
        if value <= 5000 {
            Some(Self(value))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::ThreadRng;
    use rand::seq::SliceRandom;

    fn pick_random<T>(rng: &mut ThreadRng, value: &[T]) -> T
    where
        T: Copy,
    {
        *value.choose_multiple(rng, 1).next().unwrap()
    }

    #[test]
    fn test_create_composition() {
        let mut rng = rand::thread_rng();
        let ratings = (0..=5000).collect::<Vec<_>>();

        let mut random_rating_value = || pick_random(&mut rng, &ratings);
        let mut random_rating = || {
            Rating::new(random_rating_value())
                .expect("must not fail because random rating value is in valid range")
        };
        let max_rating = Rating::new(5000).unwrap();
        let min_rating = Rating::new(0).unwrap();

        let player = Player {
            roles: Roles::new(
                Role::Ranked(random_rating()),
                Role::Ranked(max_rating),
                Role::Ranked(min_rating),
            )
            .expect("all ranked, so valid comb"),
        };

        let lobby = Lobby::new(vec![player]).unwrap();
    }

    #[test]
    fn empty_lobby() {
        assert!(Lobby::new(vec![]).is_none());
    }

    fn no_roles_selected() {
        assert!(Roles::new(Role::NoSelect, Role::NoSelect, Role::NoSelect).is_none());
    }

    fn invalid_rating_value() {
        assert!(Rating::new(5001).is_none());
    }
}
