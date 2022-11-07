use std::collections::HashSet;
use std::{cmp::Ordering, collections::VecDeque, vec};

use crate::prelude::*;

pub struct CasualGame {
    queue: VecDeque<Lobby>,
    // delimiter for priority part of a queue
    priority_idx: usize,
}

impl CasualGame {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            priority_idx: 0,
        }
    }
}

impl Queue for CasualGame {
    fn feed(&mut self, lobby: &Lobby) {
        assert!(self.valid_lobby(lobby));
        self.queue.push_back(lobby.clone());
    }

    // TODO test feature
    // priority queue as part of a matchmaking queue
    fn feed_priority(&mut self, lobby: &Lobby) {
        assert!(self.valid_lobby(lobby));
        self.queue.insert(self.priority_idx, lobby.clone());
        self.priority_idx += 1;
    }

    fn take(&mut self, team_sizes: &[u32]) -> Option<Vec<Vec<Lobby>>> {
        let total_player_amount_in_queue = self
            .queue
            .iter()
            .map(|lobby| lobby.player_count())
            .sum::<u32>();
        let least_req_player_amount = team_sizes.iter().sum::<u32>();
        if total_player_amount_in_queue < least_req_player_amount {
            return None; // not enough players to form teams, no further checks required
        }

        type PickOut = Option<(Vec<u32>, Vec<usize>)>;

        fn pick_out(
            queue: &VecDeque<Lobby>,
            tree_nesting: u32,
            reserved_indeces: &HashSet<usize>,
        ) -> PickOut {
            // try to complete subtree from every element in order
            // return first complete subtree
            // if one subtree fails, move to another
            // if all subtrees fail to complete, return None
            // skip reserved indeces
            fn _pick_out(
                queue: &VecDeque<Lobby>,
                tree_path: &Vec<u32>,
                indeces: &Vec<usize>,
                tree_nesting: u32,
                start_idx: usize,
                reserved_indeces: &HashSet<usize>,
            ) -> PickOut {
                for (i, l) in queue.iter().enumerate() {
                    // cannot pass slice of vector, so depend on start_idx and skip
                    if i < start_idx {
                        continue;
                    }

                    if reserved_indeces.contains(&i) {
                        continue;
                    }

                    let subtree_path = [tree_path.as_slice(), &[l.player_count()]].concat();

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

            _pick_out(queue, &vec![], &vec![], tree_nesting, 0, reserved_indeces)
        }

        let mut teams: Vec<Vec<usize>> = vec![];
        let mut reserved_indeces = HashSet::new();

        for team_size in team_sizes {
            let result = pick_out(&self.queue, *team_size, &reserved_indeces);

            match &result {
                None => return None, // cannot form requested teams from existing lobbies
                Some((_lobby_lengths, indeces)) => {
                    reserved_indeces.extend(indeces);
                    teams.push(indeces.clone());
                }
            }
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

                if index < self.priority_idx {
                    self.priority_idx -= 1;
                }
            }

            lobbies.push(team_lobbies);
        }

        Some(lobbies)
    }
}

enum PathResolution {
    Complete,
    Incomplete,
    Nil,
}

fn resolved_path(tree_nesting: u32, path: &[u32]) -> PathResolution {
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

impl Game for CasualGame {
    fn valid_lobby(&self, _lobby: &Lobby) -> bool {
        true
    }

    fn reduced_roles_lobby(&self, lobby: &Lobby) -> Lobby {
        lobby.clone()
    }
}

mod tests {
    use super::*;

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

    fn assert_take_happy_path(game: &mut CasualGame, team_sizes: &[u32]) {
        let initial_queue_len = game.queue.len();
        let r = game.take(team_sizes);
        assert!(r.is_some());
        let r = r.unwrap();
        let taken_out_lobby_count = r.iter().flatten().count();
        assert_eq!(game.queue.len(), initial_queue_len - taken_out_lobby_count);
        assert_eq!(r.len(), team_sizes.len());
        assert!(r.iter().zip(team_sizes).all(|(team, team_size)| {
            team.iter().map(|lobby| lobby.player_count()).sum::<u32>() == *team_size
        }));
    }

    #[test]
    fn test_casual_game() {
        let mut game = CasualGame::new();

        game.feed(&gen_default_player_lobby(4));
        game.feed(&gen_default_player_lobby(3));
        game.feed(&gen_default_player_lobby(2));
        game.feed(&gen_default_player_lobby(1));
        // 4 3 2 1

        assert_take_happy_path(&mut game, &[5, 5]); // -> [4 1] [3 2]
                                                    // empty

        game.feed(&gen_default_player_lobby(3));
        game.feed(&gen_default_player_lobby(4));
        game.feed(&gen_default_player_lobby(4));
        game.feed(&gen_default_player_lobby(1));
        // 3 4 4 1

        assert_take_happy_path(&mut game, &[5]); // -> [4 1]
                                                 // 3 4

        game.feed(&gen_default_player_lobby(2));
        // 3 4 2

        assert_take_happy_path(&mut game, &[5]); // -> [3 2]
                                                 // 4

        game.feed(&gen_default_player_lobby(1));
        // 4 1

        assert_take_happy_path(&mut game, &[5]); // -> [4 1]
                                                 // empty

        assert!(game.queue.is_empty());
    }
}
