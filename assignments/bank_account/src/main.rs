mod bank_account;
use bank_account::BankAccount;

fn main() {
    println!("=== BankAccount Demo ===\n");

    // Create a new account
    let mut account = BankAccount::new(500.0);
    println!("New account created.");
    println!("Starting balance:  ${:.2}", account.balance());

    // Deposit money
    account.deposit(250.0);
    println!("\nDeposited $250.00");
    println!("Balance after deposit:  ${:.2}", account.balance());

    // Withdraw money
    account.withdraw(100.0);
    println!("\nWithdrew $100.00");
    println!("Balance after withdrawal:  ${:.2}", account.balance());

    // Attempt overdraft
    println!("\nAttempting to withdraw $1000.00 (more than balance)...");
    account.withdraw(1000.0);
    println!("Balance unchanged:  ${:.2}", account.balance());

    // Attempt negative deposit
    println!("\nAttempting to deposit -$50.00 (negative amount)...");
    account.deposit(-50.0);
    println!("Balance unchanged:  ${:.2}", account.balance());

    // Final state
    println!("\nFinal account state:  {:?}", account);
}