use crate::lobby::Lobby;

pub trait Game {
    fn valid_lobby(&self, lobby: &Lobby) -> bool;
    fn reduced_roles_lobby(&self, lobby: &Lobby) -> Lobby;
}
