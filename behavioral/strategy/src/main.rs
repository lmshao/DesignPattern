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
        println!("   Amount: ${:.2}", amount);
        println!("   CVV: {}", "*".repeat(self.cvv.len()));
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
}

impl PayPalPayment {
    fn new(email: String) -> Self {
        Self { email }
    }
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) -> bool {
        println!("📧 Processing PayPal payment:");
        println!("   Email: {}", self.email);
        println!("   Amount: ${:.2}", amount);
        println!("   ✅ PayPal payment successful!");
        true
    }

    fn get_name(&self) -> &str {
        "PayPal"
    }
}

/// PaymentContext - Context that uses payment strategies
struct PaymentContext {
    payment_strategy: Option<Box<dyn PaymentStrategy>>,
}

impl PaymentContext {
    fn new() -> Self {
        Self {
            payment_strategy: None,
        }
    }

    fn set_payment_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.payment_strategy = Some(strategy);
    }

    fn process_payment(&self, amount: f64) -> bool {
        if let Some(strategy) = &self.payment_strategy {
            println!("💳 Using {} payment method", strategy.get_name());
            strategy.pay(amount)
        } else {
            println!("❌ No payment method selected!");
            false
        }
    }
}

fn main() {
    println!("💳 Strategy Pattern Example - Payment System");
    println!("{}", "=".repeat(40));

    let mut payment_context = PaymentContext::new();
    let amount = 120.50;

    println!("💰 Processing payment of ${:.2}", amount);
    println!();

    // Test Credit Card payment
    println!("🔄 Using Credit Card:");
    payment_context.set_payment_strategy(Box::new(CreditCardPayment::new(
        "1234567890123456".to_string(),
        "John Doe".to_string(),
        "123".to_string(),
    )));
    payment_context.process_payment(amount);
    println!();

    // Test PayPal payment
    println!("🔄 Using PayPal:");
    payment_context.set_payment_strategy(Box::new(PayPalPayment::new(
        "john.doe@example.com".to_string(),
    )));
    payment_context.process_payment(amount);
    println!();

    println!("✅ Strategy Pattern example completed!");
    println!();
    println!("💡 Key Points:");
    println!("  - PaymentStrategy defines the algorithm interface");
    println!("  - CreditCard and PayPal are concrete strategies");
    println!("  - PaymentContext uses payment strategies");
    println!("  - Payment algorithms can be swapped at runtime");
}
