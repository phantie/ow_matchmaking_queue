#![allow(unused_variables, dead_code, unused_mut)]
use std::collections::{HashMap, HashSet};
use std::{cmp::Ordering, collections::VecDeque, vec};

pub enum PathResolution {
    Complete,
    Incomplete,
    Nil,
}

pub fn resolved_path(tree_nesting: u32, path: &[u32]) -> PathResolution {
    // TODO remove this requirement for more flexibility
    assert!(path
        .iter()
        .all(|&path_node| path_node >= 1 && path_node <= tree_nesting));
    assert!(path.len() > 0);

    match path.into_iter().sum::<u32>().cmp(&tree_nesting) {
        Ordering::Greater => PathResolution::Nil,
        Ordering::Less => PathResolution::Incomplete,
        Ordering::Equal => PathResolution::Complete,
    }
}

pub trait Queue {
    fn feed(&mut self, lobby: &Lobby);
    fn take(&mut self, team_sizes: &[u32]) -> Option<Vec<Vec<Lobby>>>;
}

impl Queue for CasualGame {
    fn feed(&mut self, lobby: &Lobby) {
        assert!(self.valid_lobby(lobby));
        self.queue.push_back(lobby.clone());
    }

    fn take(&mut self, team_sizes: &[u32]) -> Option<Vec<Vec<Lobby>>> {
        let total_player_amount_in_queue: usize =
            self.queue.iter().map(|lobby| lobby.player_count()).sum();
        let least_req_player_amount = team_sizes.iter().sum::<u32>() as usize;
        if total_player_amount_in_queue < least_req_player_amount {
            return None; // not enough players to form teams, no further checks required
        }

        let mut teams: Vec<Vec<usize>> = vec![];

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

                match resolved_path(tree_nesting, &subtree_path) {
                    PathResolution::Nil => continue,
                    PathResolution::Complete => {
                        return Some((subtree_path, [indeces.as_slice(), &[i]].concat()))
                    }
                    PathResolution::Incomplete => {
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
            }
            None
        }

        let mut reserved_indeces: Vec<usize> = vec![];

        for team_size in team_sizes {
            let result = _pick_out(
                &self.queue,
                &vec![],
                &vec![],
                *team_size,
                0,
                &reserved_indeces,
            );

            match &result {
                None => return None, // cannot form requested teams from existing lobbies
                Some((lobby_lengths, indeces)) => {
                    reserved_indeces.extend(indeces);
                    teams.push(indeces.clone());
                }
            }

            // dbg!(&result);
        }

        assert_eq!(teams.len(), team_sizes.len());

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
}

impl CasualGame {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl Game for CasualGame {
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

    #[test]
    fn no_roles_selected() {
        assert!(Roles::new(Role::NoSelect, Role::NoSelect, Role::NoSelect).is_none());
    }

    #[test]
    fn invalid_rating_value() {
        assert!(Rating::new(5001).is_none());
    }

    #[test]
    fn cmp_rating() {
        assert!(Rating::MAX > Rating::MIN);
        assert!(Rating::MAX == Rating::MAX);
    }

    #[test]
    fn resolve_paths() {
        assert!(matches!(resolved_path(5, &[5]), PathResolution::Complete));
        assert!(matches!(
            resolved_path(5, &[1, 1, 1, 1, 1]),
            PathResolution::Complete
        ));
        assert!(matches!(
            resolved_path(5, &[1, 1, 1]),
            PathResolution::Incomplete
        ));
        assert!(matches!(resolved_path(5, &[3, 3]), PathResolution::Nil));
    }

    fn gen_default_player_lobby(player_number: u32) -> Lobby {
        let players = (0..player_number)
            .map(|_| Player::default())
            .collect::<Vec<_>>();
        Lobby::new(players).unwrap()
    }

    #[test]
    fn test_casual_game() {
        let mut game = CasualGame::new();

        game.feed(&gen_default_player_lobby(4));
        game.feed(&gen_default_player_lobby(3));
        game.feed(&gen_default_player_lobby(2));
        game.feed(&gen_default_player_lobby(1));

        assert!(game.queue.len() == 4);
        assert!(game.take(&[5, 5]).is_some());
        assert!(game.queue.len() == 0);
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
