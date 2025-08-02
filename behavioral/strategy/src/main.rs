// Copyright © 2025 SHAO Liming <lmshao@163.com>
// Licensed under the MIT License
//
// Strategy Pattern Example
//
// The strategy pattern defines a family of algorithms, encapsulates each one,
// and makes them interchangeable. Strategy lets the algorithm vary independently
// from clients that use it.

/// PaymentStrategy trait - defines the interface for payment algorithms
trait PaymentStrategy {
    fn pay(&self, amount: f64) -> bool;
    fn get_name(&self) -> &str;
}

/// CreditCardPayment - Concrete Strategy
struct CreditCardPayment {
    card_number: String,
    card_holder: String,
    cvv: String,
}

impl CreditCardPayment {
    fn new(card_number: String, card_holder: String, cvv: String) -> Self {
        Self {
            card_number,
            card_holder,
            cvv,
        }
    }
}

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) -> bool {
        println!("💳 Processing credit card payment:");
        println!(
            "   Card: {}****{}",
            &self.card_number[..4],
            &self.card_number[self.card_number.len() - 4..]
        );
        println!("   Holder: {}", self.card_holder);
        println!("   CVV: {} (verified)", "*".repeat(self.cvv.len()));
        println!("   Amount: ${:.2}", amount);

        // Simulate payment processing
        println!("   ✅ Credit card payment successful!");
        true
    }

    fn get_name(&self) -> &str {
        "Credit Card"
    }
}

/// PayPalPayment - Concrete Strategy
struct PayPalPayment {
    email: String,
    password: String,
}

impl PayPalPayment {
    fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) -> bool {
        println!("📧 Processing PayPal payment:");
        println!("   Email: {}", self.email);
        println!("   Password: {} (verified)", "*".repeat(self.password.len()));
        println!("   Amount: ${:.2}", amount);

        // Simulate payment processing
        println!("   ✅ PayPal payment successful!");
        true
    }

    fn get_name(&self) -> &str {
        "PayPal"
    }
}

/// CryptoPayment - Concrete Strategy
struct CryptoPayment {
    wallet_address: String,
    crypto_type: String,
}

impl CryptoPayment {
    fn new(wallet_address: String, crypto_type: String) -> Self {
        Self {
            wallet_address,
            crypto_type,
        }
    }
}

impl PaymentStrategy for CryptoPayment {
    fn pay(&self, amount: f64) -> bool {
        println!("₿ Processing {} payment:", self.crypto_type);
        println!(
            "   Wallet: {}...{}",
            &self.wallet_address[..8],
            &self.wallet_address[self.wallet_address.len() - 8..]
        );
        println!("   Amount: {} {}", amount, self.crypto_type);

        // Simulate payment processing
        println!("   ✅ {} payment successful!", self.crypto_type);
        true
    }

    fn get_name(&self) -> &str {
        &self.crypto_type
    }
}

/// BankTransferPayment - Concrete Strategy
struct BankTransferPayment {
    account_number: String,
    bank_name: String,
}

impl BankTransferPayment {
    fn new(account_number: String, bank_name: String) -> Self {
        Self {
            account_number,
            bank_name,
        }
    }
}

impl PaymentStrategy for BankTransferPayment {
    fn pay(&self, amount: f64) -> bool {
        println!("🏦 Processing bank transfer:");
        println!("   Bank: {}", self.bank_name);
        println!(
            "   Account: {}****{}",
            &self.account_number[..4],
            &self.account_number[self.account_number.len() - 4..]
        );
        println!("   Amount: ${:.2}", amount);

        // Simulate payment processing
        println!("   ✅ Bank transfer successful!");
        true
    }

    fn get_name(&self) -> &str {
        "Bank Transfer"
    }
}

/// ShoppingCart - Context that uses payment strategies
struct ShoppingCart {
    items: Vec<(String, f64)>,
    payment_strategy: Option<Box<dyn PaymentStrategy>>,
}

impl ShoppingCart {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            payment_strategy: None,
        }
    }

    fn add_item(&mut self, name: String, price: f64) {
        self.items.push((name.clone(), price));
        println!("🛒 Added {} - ${:.2}", name, price);
    }

    fn set_payment_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.payment_strategy = Some(strategy);
    }

    fn checkout(&self) -> bool {
        let total = self.items.iter().map(|(_, price)| price).sum::<f64>();

        println!("\n🛒 Shopping Cart Summary:");
        println!("{}", "=".repeat(30));
        for (name, price) in &self.items {
            println!("   {} - ${:.2}", name, price);
        }
        println!("   Total: ${:.2}", total);
        println!();

        if let Some(strategy) = &self.payment_strategy {
            println!("💳 Using {} payment method", strategy.get_name());
            strategy.pay(total)
        } else {
            println!("❌ No payment method selected!");
            false
        }
    }

    fn clear(&mut self) {
        self.items.clear();
        println!("🛒 Cart cleared");
    }
}

fn main() {
    println!("💳 Strategy Pattern Example - Payment System");
    println!("{}", "=".repeat(50));

    // Create shopping cart
    let mut cart = ShoppingCart::new();

    // Add items to cart
    cart.add_item("Laptop".to_string(), 999.99);
    cart.add_item("Mouse".to_string(), 29.99);
    cart.add_item("Keyboard".to_string(), 89.99);

    // Test different payment strategies
    let payment_scenarios: Vec<(&str, Box<dyn PaymentStrategy>)> = vec![
        (
            "Credit Card",
            Box::new(CreditCardPayment::new(
                "1234567890123456".to_string(),
                "John Doe".to_string(),
                "123".to_string(),
            )),
        ),
        (
            "PayPal",
            Box::new(PayPalPayment::new(
                "john.doe@example.com".to_string(),
                "password123".to_string(),
            )),
        ),
        (
            "Bitcoin",
            Box::new(CryptoPayment::new(
                "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
                "Bitcoin".to_string(),
            )),
        ),
        (
            "Bank Transfer",
            Box::new(BankTransferPayment::new(
                "1234567890".to_string(),
                "Chase Bank".to_string(),
            )),
        ),
    ];

    for (scenario_name, strategy) in payment_scenarios {
        println!("\n🔄 Testing {} payment scenario:", scenario_name);
        println!("{}", "=".repeat(40));

        cart.set_payment_strategy(strategy);
        cart.checkout();

        println!();
    }

    // Demonstrate cart clearing functionality
    println!("\n🧹 Demonstrating cart management:");
    println!("{}", "=".repeat(40));
    cart.clear();
    
    // Add new items after clearing
    cart.add_item("Book".to_string(), 19.99);
    cart.add_item("Pen".to_string(), 2.99);
    
    // Set a payment strategy and checkout again
    cart.set_payment_strategy(Box::new(CreditCardPayment::new(
        "9876543210987654".to_string(),
        "Jane Smith".to_string(),
        "456".to_string(),
    )));
    cart.checkout();

    println!("\n✅ Strategy Pattern example completed!");
    println!();
    println!("💡 Design Pattern Key Points:");
    println!("  - PaymentStrategy trait defines the algorithm interface");
    println!("  - CreditCard, PayPal, Crypto, BankTransfer are concrete strategies");
    println!("  - ShoppingCart is the context that uses payment strategies");
    println!("  - Payment algorithms can be swapped at runtime");
    println!("  - Each strategy encapsulates its own payment logic");
}
