use mu_core::{Currency, Ledger, Money};
use rust_decimal_macros::dec;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║     \x1b[32mμBank — CORE v0.1\x1b[0m                ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    let mut ledger = Ledger::new();

    let alice = ledger.create_customer("Alice".into());
    println!("\x1b[32m✓\x1b[0m Customer created: Alice"); // #[ptbr] Contém ANSI escape code. Ver learning notes

    let acc = ledger
        .create_account(alice, Currency::MUB)
        .expect("create account");
    ledger.activate_account(acc).expect("activate account");
    println!("\x1b[32m✓\x1b[0m Account created:     {}", acc);
    println!("  Currency:            MUB");

    let tx = ledger
        .deposit(acc, Money::new(dec!(1000), Currency::MUB), "Initial deposit".into())
        .expect("deposit");
    println!("\x1b[32m✓\x1b[0m Deposit:            +1,000.00 MUB  [{}]", tx);

    let balance = ledger.balance(acc).expect("balance");
    println!("\x1b[32m✓\x1b[0m Balance:            {}", balance);
    println!();

    let tx = ledger
        .deposit(acc, Money::new(dec!(500.50), Currency::MUB), "Freelance payment".into())
        .expect("deposit");
    println!("\x1b[32m✓\x1b[0m Deposit:            +500.50 MUB  [{}]", tx);

    let tx = ledger
        .withdraw(acc, Money::new(dec!(200), Currency::MUB), "ATM withdrawal".into())
        .expect("withdraw");
    println!("\x1b[32m✓\x1b[0m Withdraw:           -200.00 MUB  [{}]", tx);

    let balance = ledger.balance(acc).expect("balance");
    println!("\x1b[32m✓\x1b[0m Final balance:      {}", balance);
    println!();

    println!("── Statement ──────────────────────────");
    for tx in ledger.all_transactions() {
        println!(
            "  {}  {:>12}  {}",
            tx.timestamp().format("%H:%M:%S"),
            tx.amount(),
            tx.description()
        );
    }
    println!("───────────────────────────────────────");
    println!();

    let bob = ledger.create_customer("Bob".into());
    let acc_b = ledger
        .create_account(bob, Currency::MUB)
        .expect("create bob account");
    ledger.activate_account(acc_b).expect("activate bob account");

    let tx = ledger
        .transfer(acc, acc_b, Money::new(dec!(300), Currency::MUB), "Pix to Bob".into())
        .expect("transfer");
    println!("\x1b[32m✓\x1b[0m Transfer:           -300.00 MUB → Bob  [{}]", tx);
    println!("  Alice balance:      {}", ledger.balance(acc).unwrap());
    println!("  Bob balance:        {}", ledger.balance(acc_b).unwrap());
    println!();

    println!("\x1b[32mμBank Core is live. Blindado por design.\x1b[0m");
}
