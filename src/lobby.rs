#![allow(unused_imports)]

use crate::player::Player;
use crate::rating::Rating;
use crate::roles::{Role, Roles};

#[derive(Debug, Clone)]
pub struct Lobby {
    // len at least one
    pub players: Vec<Player>,
    rating: bool,
}

impl Lobby {
    pub fn player_count(&self) -> u32 {
        self.players.len() as u32
    }

    pub fn empty(&self) -> bool {
        self.player_count() == 0
    }

    pub fn solo(&self) -> bool {
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

    pub fn merge(&self, other: &Self) -> Self {
        // check for same rating, checked again in Self::new
        assert!(!(self.rating ^ other.rating));
        let mut joined_players = self.players.clone();
        joined_players.append(&mut other.players.clone());
        Self::new(joined_players)
            .expect("merge of initially checked lobbies with the same rating must not fail")
    }
}

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
        let ratings = (Rating::MIN.0..=Rating::MAX.0).collect::<Vec<_>>();

        let mut random_rating_value = || pick_random(&mut rng, &ratings);
        let mut random_rating = || {
            Rating::new(random_rating_value())
                .expect("must not fail because random rating value is in valid range")
        };

        let player = Player {
            roles: Roles::new(
                Role::Ranked(random_rating()),
                Role::Ranked(Rating::MAX),
                Role::Ranked(Rating::MIN),
            )
            .expect("all ranked, so valid comb"),
        };

        let _lobby = Lobby::new(vec![player]).unwrap();
    }

    #[test]
    fn empty_lobby() {
        assert!(Lobby::new(vec![]).is_none());
    }
}
