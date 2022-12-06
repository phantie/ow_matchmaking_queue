use crate::lobby::Lobby;

pub trait Queue<'lobby, L>
where
    L: Lobby,
{
    fn feed(&mut self, lobby: &'lobby L);
    fn feed_priority(&mut self, lobby: &'lobby L);
    fn take(&mut self, team_sizes: &[u32]) -> Option<Vec<Vec<&'lobby L>>>;
}
