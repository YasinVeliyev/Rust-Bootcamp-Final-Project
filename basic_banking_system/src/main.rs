trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), &str>;
    fn balance(&mut self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), &str> {
        if amount > self.balance {
            return Err("Insufficient balance");
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&mut self) -> f64 {
        self.balance
    }
}

impl BankAccount {
    fn new(account_number: &str, holder_name: &str, balance: f64) -> Self {
        Self {
            account_number: account_number.to_owned(),
            holder_name: holder_name.to_owned(),
            balance,
        }
    }
}

fn main() {
    let mut account1 = BankAccount::new("1111", "Yasin", 5000.0);
    let mut account2 = BankAccount::new("2222", "Asim", 5600.0);
    account1.deposit(560.);
    match account2.withdraw(685.0) {
        Ok(_) => println!("Balance for Account 2 {}", account2.balance()),
        Err(err) => println!("{}", err),
    };

    println!("Balance for Account 1 {}", account1.balance());
}
