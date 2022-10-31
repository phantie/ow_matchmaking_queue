use ow_role_q::*;

fn main() -> () {
    // let rank = Role::Ranked(Rating::new(1000).unwrap());

    let game = OneTwoTwoGame {};

    let player1 = Player {
        roles: Roles::new(
            Role::RatingNonApplicable,
            Role::NoSelect,
            Role::RatingNonApplicable,
        )
        .unwrap(),
    };

    let player2 = Player {
        roles: Roles::new(Role::RatingNonApplicable, Role::NoSelect, Role::NoSelect).unwrap(),
    };

    let lobby = Lobby::new(vec![player1, player2]).unwrap();

    game.valid_lobby(&lobby);
}
