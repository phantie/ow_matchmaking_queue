#![allow(unused_variables, dead_code)]

use ow_role_q::*;

fn main() -> () {
    let mut game = CasualGame::new(5);

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

    let r = game.feed_and_yield(&lobby1);
    let r = game.feed_and_yield(&lobby2);
    let r = game.feed_and_yield(&lobby1);
    // let r = game.feed_and_yield(&lobby3);
    assert!(r.is_none());
    let r = game.feed_and_yield(&lobby2);
    assert!(r.is_some());
    // dbg!(r);

    let t = build_tree(3);
    dbg!(t);
}
