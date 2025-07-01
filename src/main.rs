#[allow(unused)]
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        assert!(
            self.balance >= amount,
            "You cannot withdraw more than you have!"
        );
        self.balance -= amount;
        self.balance
    }
}

#[derive(Debug)]
struct Bank {
    // every bank has a list of accounts associated with it
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();

    let mut emeka_account = Account::new(1, String::from("emeka"));
    let mut salinha_account = Account::new(1, String::from("salinha"));
    let mut kamsi_account = Account::new(1, String::from("kamsi"));
    let mut chelsea_account = Account::new(1, String::from("chelsea"));

    // make deposits
    emeka_account.deposit(500);
    salinha_account.deposit(430);
    kamsi_account.deposit(990);
    chelsea_account.deposit(400);

    // withdraw
    emeka_account.withdraw(12);
    salinha_account.withdraw(32);
    chelsea_account.withdraw(321);
    kamsi_account.withdraw(122);

    // all these accounts are moved and essentially useless after adding them to a bank!
    bank.add_account(emeka_account);
    bank.add_account(salinha_account);
    bank.add_account(kamsi_account);
    bank.add_account(chelsea_account);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
