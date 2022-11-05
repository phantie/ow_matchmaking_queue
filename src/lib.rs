#![allow(unused_variables, dead_code, unused_mut)]
use std::collections::{HashMap, HashSet};
use std::{collections::VecDeque, vec};

pub fn resolved_path(tree_nesting: u32, path: &[u32]) -> bool {
    assert!(path
        .iter()
        .all(|&path_node| path_node >= 1 && path_node <= tree_nesting));
    assert!(path.len() > 0);
    path.into_iter().sum::<u32>() <= tree_nesting
}

pub trait Queue {
    fn feed(&mut self, lobby: &Lobby);
    fn feed_and_yield(&mut self, lobby: &Lobby) -> Option<Vec<Vec<Lobby>>>;
}

impl Queue for CasualGame {
    fn feed(&mut self, lobby: &Lobby) {
        assert!(self.valid_lobby(lobby));
        self.queue.push_back(lobby.clone());
    }

    fn feed_and_yield(&mut self, lobby: &Lobby) -> Option<Vec<Vec<Lobby>>> {
        self.feed(lobby);

        let total_player_count_in_queue: usize =
            self.queue.iter().map(|lobby| lobby.player_count()).sum();

        if total_player_count_in_queue < self.team_player_count * 2 {
            return None; // not enough players to form teams, no further checks required
        }

        let mut teams: Vec<Vec<usize>> = vec![];

        let mut teams_to_form: u32 = 2;

        fn _pick_out(
            queue: &VecDeque<Lobby>,
            tree_path: &Vec<u32>,
            indeces: &Vec<usize>,
            tree_nesting: u32,
            start_idx: usize,
            reserved_indeces: &Vec<usize>,
        ) -> Option<(Vec<u32>, Vec<usize>)> {
            for (i, l) in queue.iter().enumerate() {
                // cannot pass slice of vector, so depend on start_idx and skip
                if i < start_idx {
                    continue;
                }

                if reserved_indeces.contains(&i) {
                    continue;
                }

                let subtree_path = [tree_path.as_slice(), &[l.player_count() as u32]].concat();

                if !resolved_path(tree_nesting, &subtree_path) {
                    continue;
                }

                if subtree_path.iter().sum::<u32>() == tree_nesting {
                    return Some((subtree_path, [indeces.as_slice(), &[i]].concat()));
                } else {
                    let result = _pick_out(
                        queue,
                        &subtree_path,
                        &[indeces.as_slice(), &[i]].concat(),
                        tree_nesting,
                        start_idx + i + 1,
                        reserved_indeces,
                    );

                    if result != None {
                        return result;
                    }
                }
            }
            None
        }

        let mut reserved_indeces: Vec<usize> = vec![];

        loop {
            if teams_to_form == 0 {
                break;
            }

            let result = _pick_out(&self.queue, &vec![], &vec![], 5, 0, &reserved_indeces);

            match &result {
                None => return None,
                Some((lobby_lengths, indeces)) => {
                    reserved_indeces.extend(indeces);
                    teams.push(indeces.clone());
                    teams_to_form -= 1;
                }
            }

            // dbg!(&result);
        }

        if teams_to_form > 0 {
            return None; // cannot form a team from existing lobbies
        }

        assert_eq!(teams.len(), 2);

        let mut lobbies = vec![];

        let mut s: HashSet<usize> = HashSet::new();

        for indeces in teams {
            let mut team_lobbies = vec![];

            for index in indeces {
                let mov = s.iter().filter(|&&v| v < index).count();
                team_lobbies.push(self.queue.remove(index - mov).expect("idx must be present"));
                s.insert(index);
            }

            lobbies.push(team_lobbies);
        }

        Some(lobbies)
    }
}

pub struct CasualGame {
    queue: VecDeque<Lobby>,
    team_player_count: usize,
}

impl CasualGame {
    pub fn new(team_player_count: usize) -> Self {
        Self {
            team_player_count,
            queue: VecDeque::new(),
        }
    }
}

impl Game for CasualGame {
    fn valid_lobby(&self, lobby: &Lobby) -> bool {
        lobby.player_count() <= self.team_player_count
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

        let mut rows = vec![];

        for player in players {
            let roles = player.roles;

            let (tank, damage, support) = (roles.tank, roles.damage, roles.support);

            let row = [tank, damage, support]
                .into_iter()
                .enumerate()
                .filter(|(_i, role)| role.selected())
                .map(|(i, _role)| i as u32)
                .collect::<Vec<_>>();

            rows.push(row);
        }

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

#[derive(Debug, Clone)]
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

    fn merge(&self, other: &Self) -> Self {
        // check for same rating, checked again in Self::new
        assert!(!(self.rating ^ other.rating));
        let mut joined_players = self.players.clone();
        joined_players.append(&mut other.players.clone());
        Self::new(joined_players)
            .expect("merge of initially checked lobbies with the same rating must not fail")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub roles: Roles,
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
    fn selected(&self) -> bool {
        !matches!(self, Role::NoSelect)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rating(u32);

impl Rating {
    pub const MAX: Self = Self(5000);
    pub const MIN: Self = Self(0);

    pub fn new(value: u32) -> Option<Self> {
        if value <= Self::MAX.0 && value >= Self::MIN.0 {
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

    fn cmp_rating() {
        assert!(Rating::MAX > Rating::MIN);
        assert!(Rating::MAX == Rating::MAX);
    }

    fn resolve_paths() {
        assert!(resolved_path(5, &[5]));
        assert!(resolved_path(5, &[1, 1, 1, 1, 1]));
        assert!(resolved_path(5, &[1, 1, 1]));
        assert!(!resolved_path(5, &[3, 3]));
    }
}

#[derive(Debug)]
pub enum Tree {
    Child(HashMap<u32, Box<Tree>>),
}

pub fn build_tree(n: u32) -> Tree {
    let mut h: HashMap<u32, Box<Tree>>;

    if n == 0 {
        h = HashMap::with_capacity(0);
    } else {
        h = HashMap::new();
        for i in 1..=n {
            h.insert(i, Box::new(build_tree(n - i)));
        }
    }
    Tree::Child(h)
}
