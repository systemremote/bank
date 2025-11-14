use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
enum AccountType {
    Checking,
    Savings,
    Credit,
}

struct Account {
    balance: f64,
    account_type: AccountType,
    transactions: Vec<Transaction>,
    is_active: bool,
}

impl Account {
    fn new(account_type: AccountType) -> Account {
        Account {
            balance: 0.0,
            account_type,
            transactions: Vec::new(),
            is_active: true,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if self.is_active {
            self.balance += amount;
            self.transactions.push(Transaction::Deposit(amount));
        } else {
            println!("Account is inactive!");
        }
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.is_active {
            if amount > self.balance {
                false
            } else {
                self.balance -= amount;
                self.transactions.push(Transaction::Withdrawal(amount));
                true
            }
        } else {
            println!("Account is inactive!");
            false
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }

    fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    fn activate(&mut self) {
        self.is_active = true;
    }

    fn deactivate(&mut self) {
        self.is_active = false;
    }
}

#[derive(Debug, Clone)]
enum Transaction {
    Deposit(f64),
    Withdrawal(f64),
    Transfer(f64, String),
}

struct Bank {
    accounts: HashMap<String, Account>,
}

impl Bank {
    fn new() -> Bank {
        Bank {
            accounts: HashMap::new(),
        }
    }

    fn create_account(&mut self, account_number: String, account_type: AccountType) {
        self.accounts.insert(account_number, Account::new(account_type));
    }

    fn deposit(&mut self, account_number: String, amount: f64) -> bool {
        if let Some(account) = self.accounts.get_mut(&account_number) {
            account.deposit(amount);
            true
        } else {
            false
        }
    }

    fn withdraw(&mut self, account_number: String, amount: f64) -> bool {
        if let Some(account) = self.accounts.get_mut(&account_number) {
            account.withdraw(amount)
        } else {
            false
        }
    }

    fn balance(&self, account_number: String) -> Option<f64> {
        if let Some(account) = self.accounts.get(&account_number) {
            Some(account.balance())
        } else {
            None
        }
    }

    fn transfer(&mut self, from_account: String, to_account: String, amount: f64) -> bool {
        if let (Some(from), Some(to)) = (self.accounts.get_mut(&from_account), self.accounts.get_mut(&to_account)) {
            if from.withdraw(amount) {
                to.deposit(amount);
                from.transactions.push(Transaction::Transfer(amount, to_account.clone()));
                to.transactions.push(Transaction::Transfer(amount, from_account.clone()));
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_account_type(&self, account_number: String) -> Option<AccountType> {
        if let Some(account) = self.accounts.get(&account_number) {
            Some(account.account_type.clone())
        } else {
            None
        }
    }

    fn get_transactions(&self, account_number: String) -> Option<&Vec<Transaction>> {
        if let Some(account) = self.accounts.get(&account_number) {
            Some(account.transactions())
        } else {
            None
        }
    }

    fn activate_account(&mut self, account_number: String) -> bool {
        if let Some(account) = self.accounts.get_mut(&account_number) {
            account.activate();
            true
        } else {
            false
        }
    }

    fn deactivate_account(&mut self, account_number: String) -> bool {
        if let Some(account) = self.accounts.get_mut(&account_number) {
            account.deactivate();
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut bank = Bank::new();

    loop {
        println!("1. Create Account");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("4. Check Balance");
        println!("5. Transfer");
        println!("6. Get Account Type");
        println!("7. Get Transactions");
        println!("8. Activate Account");
        println!("9. Deactivate Account");
        println!("10. Exit");

        match menu::select("Enter your choice: ") {
            1 => create_account(&mut bank),
            2 => deposit(&mut bank),
            3 => withdraw(&mut bank),
            4 => check_balance(&bank),
            5 => transfer(&mut bank),
            6 => get_account_type(&bank),
            7 => get_transactions(&bank),
            8 => activate_account(&mut bank),
            9 => deactivate_account(&mut bank),
            10 => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn create_account(bank: &mut Bank) {
    let account_number = menu::input("Enter account number: ");
    let account_type = match menu::select("Enter account type (1. Checking, 2. Savings, 3. Credit): ") {
        1 => AccountType::Checking,
        2 => AccountType::Savings,
        3 => AccountType::Credit,
        _ => {
            println!("Invalid account type!");
            return;
        }
    };

    bank.create_account(account_number, account_type);
    println!("Account created successfully!");
}

fn deposit(bank: &mut Bank) {
    let account_number = menu::input("Enter account number: ");
    let amount = match menu::float("Enter amount to deposit: ") {
        Ok(amount) => amount,
        Err(_) => {
            println!("Invalid amount!");
            return;
        }
    };

    if bank.deposit(account_number, amount) {
        println!("Deposit successful!");
    } else {
        println!("Account not found!");
    }
}

fn withdraw(bank: &mut Bank) {
    let account_number = menu::input("Enter account number: ");
    let amount = match menu::float("Enter amount to withdraw: ") {
        Ok(amount) => amount,
        Err(_) => {
            println!("Invalid amount!");
            return;
        }
    };

    if bank.withdraw(account_number, amount) {
        println!("Withdrawal successful!");
    } else {
        println!("Insufficient balance or account not found!");
    }
}

fn check_balance(bank: &Bank) {
    let account_number = menu::input("Enter account number: ");
    if let Some(balance) = bank.balance(account_number) {
        println!("Balance: {}", balance);
    } else {
        println!("Account not found!");
    }
}

fn transfer(bank: &mut Bank) {
    let from_account = menu::input("Enter account number to transfer from: ");
    let to_account = menu::input("Enter account number to transfer to: ");
    let amount = match menu::float("Enter amount to transfer: ") {
        Ok(amount) => amount,
        Err(_) => {
            println!("Invalid amount!");
            return;
        }
    };

    if bank.transfer(from_account, to_account, amount) {
        println!("Transfer successful!");
    } else {
        println!("Insufficient balance or account not found!");
    }
}

fn get_account_type(bank: &Bank) {
    let account_number = menu::input("Enter account number: ");
    if let Some(account_type) = bank.get_account_type(account_number) {
        println!("Account Type: {:?}", account_type);
    } else {
        println!("Account not found!");
    }
}

fn get_transactions(bank: &Bank) {
    let account_number = menu::input("Enter account number: ");
    if let Some(transactions) = bank.get_transactions(account_number) {
        println!("Transactions:");
        for (i, transaction) in transactions.iter().enumerate() {
            println!("{}: {:?}", i + 1, transaction);
        }
    } else {
        println!("Account not found!");
    }
}

fn activate_account(bank: &mut Bank) {
    let account_number = menu::input("Enter account number: ");
    if bank.activate_account(account_number) {
        println!("Account activated successfully!");
    } else {
        println!("Account not found!");
    }
}

fn deactivate_account(bank: &mut Bank) {
    let account_number = menu::input("Enter account number: ");
    if bank.deactivate_account(account_number) {
        println!("Account deactivated successfully!");
    } else {
        println!("Account not found!");
    }
}

mod menu {
    use std::io;

    pub fn select(prompt: &str) -> u8 {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().parse().expect("Invalid input")
    }

    pub fn input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    pub fn float(prompt: &str) -> Result<f64, ()> {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read
