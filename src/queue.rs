use crate::lobby::Lobby;

pub trait Queue<L>
where
    L: Lobby,
{
    fn feed(&mut self, lobby: &L);
    fn feed_priority(&mut self, lobby: &L);
    fn take(&mut self, team_sizes: &[u32]) -> Option<Vec<Vec<L>>>;
}
