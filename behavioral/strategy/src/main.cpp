
/**
 * @file main.cpp
 * @brief Strategy Pattern Example - C++ Implementation
 * @author SHAO Liming <lmshao@163.com>
 * @copyright Copyright © 2025 SHAO Liming <lmshao@163.com>
 * @license MIT
 *
 * The strategy pattern defines a family of algorithms, encapsulates each one,
 * and makes them interchangeable. Strategy lets the algorithm vary independently
 * from clients that use it.
 *
 * SPDX-License-Identifier: MIT
 */

#include <iomanip>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

// PaymentStrategy interface
class PaymentStrategy {
public:
    virtual ~PaymentStrategy() = default;
    virtual bool pay(double amount) const = 0;
    virtual std::string getName() const = 0;
};

// CreditCardPayment
class CreditCardPayment : public PaymentStrategy {
public:
    CreditCardPayment(std::string card_number, std::string card_holder, std::string cvv)
        : card_number_(std::move(card_number)), card_holder_(std::move(card_holder)), cvv_(std::move(cvv))
    {
    }
    bool pay(double amount) const override
    {
        std::cout << "💳 Processing credit card payment:" << std::endl;
        std::cout << "   Card: " << card_number_.substr(0, 4) << "****" << card_number_.substr(card_number_.size() - 4)
                  << std::endl;
        std::cout << "   Holder: " << card_holder_ << std::endl;
        std::cout << "   CVV: " << std::string(cvv_.length(), '*') << std::endl;
        std::cout << "   Amount: $" << std::fixed << std::setprecision(2) << amount << std::endl;
        std::cout << "   ✅ Credit card payment successful!" << std::endl;
        return true;
    }
    std::string getName() const override { return "Credit Card"; }

private:
    std::string card_number_, card_holder_, cvv_;
};

// PayPalPayment
class PayPalPayment : public PaymentStrategy {
public:
    PayPalPayment(std::string email) : email_(std::move(email)) {}
    bool pay(double amount) const override
    {
        std::cout << "📧 Processing PayPal payment:" << std::endl;
        std::cout << "   Email: " << email_ << std::endl;
        std::cout << "   Amount: $" << std::fixed << std::setprecision(2) << amount << std::endl;
        std::cout << "   ✅ PayPal payment successful!" << std::endl;
        return true;
    }
    std::string getName() const override { return "PayPal"; }

private:
    std::string email_;
};

// PaymentContext
class PaymentContext {
public:
    void setPaymentStrategy(std::unique_ptr<PaymentStrategy> strategy) { payment_strategy_ = std::move(strategy); }
    bool processPayment(double amount) const
    {
        if (payment_strategy_) {
            std::cout << "💳 Using " << payment_strategy_->getName() << " payment method" << std::endl;
            return payment_strategy_->pay(amount);
        } else {
            std::cout << "❌ No payment method selected!" << std::endl;
            return false;
        }
    }

private:
    std::unique_ptr<PaymentStrategy> payment_strategy_;
};

int main()
{
    std::cout << "💳 Strategy Pattern Example - Payment System" << std::endl;
    std::cout << std::string(40, '=') << std::endl;

    PaymentContext payment_context;
    double amount = 120.50;

    std::cout << "💰 Processing payment of $" << std::fixed << std::setprecision(2) << amount << std::endl;
    std::cout << std::endl;

    // Test Credit Card payment
    std::cout << "🔄 Using Credit Card:" << std::endl;
    payment_context.setPaymentStrategy(std::make_unique<CreditCardPayment>("1234567890123456", "John Doe", "123"));
    payment_context.processPayment(amount);
    std::cout << std::endl;

    // Test PayPal payment
    std::cout << "🔄 Using PayPal:" << std::endl;
    payment_context.setPaymentStrategy(std::make_unique<PayPalPayment>("john.doe@example.com"));
    payment_context.processPayment(amount);
    std::cout << std::endl;

    std::cout << "✅ Strategy Pattern example completed!" << std::endl;
    std::cout << std::endl;
    std::cout << "💡 Key Points:" << std::endl;
    std::cout << "  - PaymentStrategy defines the algorithm interface" << std::endl;
    std::cout << "  - CreditCard and PayPal are concrete strategies" << std::endl;
    std::cout << "  - PaymentContext uses payment strategies" << std::endl;
    std::cout << "  - Payment algorithms can be swapped at runtime" << std::endl;
}
