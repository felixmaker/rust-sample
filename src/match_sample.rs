enum Account {
    User(String),
    Tester(i32),
    Other
}

fn echo_account(account: &Account) {
    match account {
        Account::User(username) => println!("User {}", username),
        Account::Tester(id) => println!("ID {}", id),
        _ => println!("Other account")
    }
}

fn main() {
    let account_list = vec![
        Account::User("Felixmaker".to_owned()),
        Account::Tester(52),
        Account::Other
    ];

    for account in account_list {
        echo_account(&account)
    }
}
