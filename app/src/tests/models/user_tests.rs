use ::models::user::User;
use ::tests::truncate_all_tables;
use ::tests::factories::user::*;

#[test]
fn find() {
    truncate_all_tables();
    let user = create();
    let found_user = User::find(user.id).unwrap().unwrap();

    assert_eq!(user.id, found_user.id);
}
