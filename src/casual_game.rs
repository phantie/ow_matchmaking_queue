use crate::prelude::*;
use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    vec,
};

pub struct CasualGame<'lobby, L>
where
    L: Lobby + 'lobby,
{
    queue: VecDeque<&'lobby L>,
    // delimiter for priority part of a queue
    priority_idx: usize,
}

impl<'lobby, L> CasualGame<'lobby, L>
where
    L: Lobby,
{
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            priority_idx: 0,
        }
    }
}

impl<'lobby, L> Queue<'lobby, L> for CasualGame<'lobby, L>
where
    L: Lobby,
{
    fn feed(&mut self, lobby: &'lobby L) {
        assert!(self.valid_lobby(lobby), "invalid lobby for this game type");
        self.queue.push_back(lobby);
    }

    // priority queue as part of a matchmaking queue
    fn feed_priority(&mut self, lobby: &'lobby L) {
        assert!(self.valid_lobby(lobby), "invalid lobby for this game type");
        self.queue.insert(self.priority_idx, lobby);
        self.priority_idx += 1;
    }

    fn take(&mut self, team_sizes: &[u32]) -> Option<Vec<Vec<&'lobby L>>> {
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
            queue: &VecDeque<&impl Lobby>,
            tree_nesting: u32,
            reserved_indeces: &HashSet<usize>,
        ) -> PickOut {
            // try to complete subtree from every element in order
            // return first complete subtree
            // if one subtree fails, move to another
            // if all subtrees fail to complete, return None
            // skip reserved indeces
            fn _pick_out(
                queue: &VecDeque<&impl Lobby>,
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

        // as elements are getting removed, indeces need to be readjusted.
        //
        // implemented, by keeping track of already popped indeces
        // and adjusting an untouched index by a number of popped indeces
        // less in value than the targeted untouched index
        let mut popped_indeces: HashSet<usize> = HashSet::new();

        for indeces in teams {
            let mut team_lobbies = vec![];

            for index in indeces {
                let mov = popped_indeces.iter().filter(|&&v| v < index).count();
                team_lobbies.push(self.queue.remove(index - mov).expect("idx must be present"));
                popped_indeces.insert(index);

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
    assert!(path.len() > 0);

    match path.into_iter().sum::<u32>().cmp(&tree_nesting) {
        Ordering::Greater => PathResolution::Nil,
        Ordering::Less => PathResolution::Incomplete,
        Ordering::Equal => PathResolution::Complete,
    }
}

impl<'lobby, L> ValidLobby<L> for CasualGame<'lobby, L>
where
    L: Lobby,
{
    fn valid_lobby(&self, lobby: &L) -> bool {
        lobby.player_count() > 0
    }
}

mod tests {
    use super::*;

    #[derive(Debug)]
    pub struct TestLobby {
        // len at least one
        player_count: u32,
    }

    impl TestLobby {
        fn new(player_count: u32) -> Self {
            Self { player_count }
        }
    }

    impl Lobby for TestLobby {
        fn player_count(&self) -> u32 {
            self.player_count
        }
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

    fn assert_take_happy_path(game: &mut CasualGame<TestLobby>, team_sizes: &[u32]) {
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

    fn _feed_lobbies(game: &mut CasualGame<TestLobby>, team_sizes: &[u32], priority: bool) {
        let lobbies: Vec<_> = team_sizes
            .into_iter()
            // it's not bad
            .map(|lobby_size| Box::leak(Box::new(TestLobby::new(*lobby_size))))
            .map(|lobby| {
                if priority {
                    game.feed_priority(&*lobby)
                } else {
                    game.feed(&*lobby)
                }
            })
            .collect();
    }

    fn feed_lobbies(game: &mut CasualGame<TestLobby>, team_sizes: &[u32]) {
        _feed_lobbies(game, team_sizes, false);
    }

    fn feed_lobbies_priority(game: &mut CasualGame<TestLobby>, team_sizes: &[u32]) {
        _feed_lobbies(game, team_sizes, true);
    }

    #[test]
    fn test_casual_game_cont_scenario() {
        let mut game = CasualGame::new();

        feed_lobbies(&mut game, &[4, 3, 2, 1]);
        // 4 3 2 1

        assert_take_happy_path(&mut game, &[5, 5]); // -> [4 1] [3 2]
                                                    // empty

        feed_lobbies(&mut game, &[3, 4, 4, 1]);
        // 3 4 4 1

        assert_take_happy_path(&mut game, &[5]); // -> [4 1]
                                                 // 3 4

        feed_lobbies(&mut game, &[2]);
        // 3 4 2

        assert_take_happy_path(&mut game, &[5]); // -> [3 2]
                                                 // 4

        feed_lobbies(&mut game, &[1]);
        // 4 1

        assert_take_happy_path(&mut game, &[5]); // -> [4 1]
                                                 // empty

        assert!(game.queue.is_empty());
    }

    #[test]
    fn test_casual_game_no_panic_on_any_lobby_length() {
        let mut game = CasualGame::new();
        // should not panic when queue has lobbies larger in size than
        // any provided requirement for fulfillment
        feed_lobbies(&mut game, &[6]);
        assert!(game.take(&[5]).is_none());
    }

    #[test]
    fn test_priority_queue() {
        // TODO write more thourough test
        // TODO verify correctness of returned lobbies by its' identities
        let mut game = CasualGame::new();

        feed_lobbies(&mut game, &[1]);
        // 1
        feed_lobbies_priority(&mut game, &[2]);
        // 2 1
        feed_lobbies_priority(&mut game, &[3]);
        // 2 3 1
        game.take(&[5]); // -> [2 3]
                         // 1
        game.take(&[1]); // -> [1]
                         // empty
        assert!(game.queue.is_empty());
    }
}
