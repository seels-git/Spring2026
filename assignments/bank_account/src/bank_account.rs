#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        let balance = if initial_balance < 0.0 { 0.0 } else { initial_balance };
        BankAccount { balance }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    // ── Creation ──────────────────────────────────────────────────────────────

    #[test]
    fn test_new_account_positive_balance() {
        let account = BankAccount::new(100.0);
        assert!(approx_eq(account.balance(), 100.0),
            "Expected balance 100.0, got {}", account.balance());
    }

    #[test]
    fn test_new_account_zero_balance() {
        let account = BankAccount::new(0.0);
        assert!(approx_eq(account.balance(), 0.0));
    }

    #[test]
    fn test_new_account_negative_initial_balance() {
        // Negative starting balance should be clamped to 0
        let account = BankAccount::new(-50.0);
        assert!(approx_eq(account.balance(), 0.0),
            "Negative initial balance should be clamped to 0.0");
    }

    // ── Deposit ───────────────────────────────────────────────────────────────

    #[test]
    fn test_deposit_normal() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert!(approx_eq(account.balance(), 150.0));
    }

    #[test]
    fn test_deposit_zero() {
        let mut account = BankAccount::new(100.0);
        account.deposit(0.0);
        assert!(approx_eq(account.balance(), 100.0),
            "Depositing 0 should not change the balance");
    }

    #[test]
    fn test_deposit_negative_amount() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-30.0);
        assert!(approx_eq(account.balance(), 100.0),
            "Depositing a negative amount should be ignored");
    }

    #[test]
    fn test_deposit_multiple_times() {
        let mut account = BankAccount::new(0.0);
        account.deposit(25.0);
        account.deposit(75.0);
        account.deposit(0.01);
        assert!(approx_eq(account.balance(), 100.01));
    }

    #[test]
    fn test_deposit_fractional_amount() {
        let mut account = BankAccount::new(0.0);
        account.deposit(0.1);
        account.deposit(0.2);
        // 0.1 + 0.2 in floating-point is not exactly 0.3 — epsilon handles this
        assert!(approx_eq(account.balance(), 0.3),
            "Expected ~0.3, got {}", account.balance());
    }

    // ── Withdraw ──────────────────────────────────────────────────────────────

    #[test]
    fn test_withdraw_normal() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert!(approx_eq(account.balance(), 60.0));
    }

    #[test]
    fn test_withdraw_entire_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(100.0);
        assert!(approx_eq(account.balance(), 0.0));
    }

    #[test]
    fn test_withdraw_more_than_balance() {
        let mut account = BankAccount::new(50.0);
        account.withdraw(100.0);
        assert!(approx_eq(account.balance(), 50.0),
            "Withdrawing more than the balance should be ignored");
    }

    #[test]
    fn test_withdraw_negative_amount() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-20.0);
        assert!(approx_eq(account.balance(), 100.0),
            "Withdrawing a negative amount should be ignored");
    }

    #[test]
    fn test_withdraw_zero() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(0.0);
        assert!(approx_eq(account.balance(), 100.0),
            "Withdrawing 0 should not change the balance");
    }

    #[test]
    fn test_withdraw_from_empty_account() {
        let mut account = BankAccount::new(0.0);
        account.withdraw(10.0);
        assert!(approx_eq(account.balance(), 0.0),
            "Cannot withdraw from a zero balance");
    }

    // ── Combined operations ───────────────────────────────────────────────────

    #[test]
    fn test_deposit_then_withdraw() {
        let mut account = BankAccount::new(200.0);
        account.deposit(50.0);   // 250.0
        account.withdraw(100.0); // 150.0
        assert!(approx_eq(account.balance(), 150.0));
    }

    #[test]
    fn test_balance_never_goes_negative() {
        let mut account = BankAccount::new(10.0);
        account.withdraw(15.0); // ignored
        account.withdraw(10.0); // succeeds → 0.0
        account.withdraw(1.0);  // ignored
        assert!(account.balance() >= 0.0,
            "Balance should never be negative, got {}", account.balance());
    }
}
