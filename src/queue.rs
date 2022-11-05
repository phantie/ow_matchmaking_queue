use crate::lobby::Lobby;

pub trait Queue {
    fn feed(&mut self, lobby: &Lobby);
    fn feed_priority(&mut self, lobby: &Lobby);
    fn take(&mut self, team_sizes: &[u32]) -> Option<Vec<Vec<Lobby>>>;
}
