use crate::prelude::*;

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
