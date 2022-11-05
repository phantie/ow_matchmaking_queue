#![allow(unused_variables, dead_code)]

use ow_role_q::casual_game::*;
use ow_role_q::prelude::*;

fn main() -> () {
    let mut game = CasualGame::new();

    let lobby1 = Lobby::new(vec![
        Player {
            roles: Roles::new(
                Role::RatingNonApplicable,
                Role::NoSelect,
                Role::RatingNonApplicable,
            )
            .unwrap(),
        },
        Player {
            roles: Roles::new(Role::RatingNonApplicable, Role::NoSelect, Role::NoSelect).unwrap(),
        },
        Player {
            roles: Roles::new(Role::NoSelect, Role::RatingNonApplicable, Role::NoSelect).unwrap(),
        },
    ])
    .unwrap();

    let lobby2 = Lobby::new(vec![
        Player {
            roles: Roles::new(
                Role::RatingNonApplicable,
                Role::NoSelect,
                Role::RatingNonApplicable,
            )
            .unwrap(),
        },
        Player {
            roles: Roles::new(Role::RatingNonApplicable, Role::NoSelect, Role::NoSelect).unwrap(),
        },
    ])
    .unwrap();

    let lobby3 = Lobby::new(vec![Player {
        roles: Roles::new(Role::RatingNonApplicable, Role::NoSelect, Role::NoSelect).unwrap(),
    }])
    .unwrap();

    // game.valid_lobby(&lobby1);

    // game.feed(&lobby1);
    game.feed_priority(&lobby2);
    // game.feed(&lobby1);
    game.feed_priority(&lobby2);
    // let r = game.take(&lobby1);
    // let r = game.take(&[5, 5]);
    let r = game.take(&[2]);
    let r = game.take(&[2]);
    assert!(r.is_some());

    dbg!(r);
    // let r = game.take(&lobby3);
    // assert!(r.is_none());
    // let r = game.take(&lobby2);
    // assert!(r.is_some());
    // dbg!(r);
}
