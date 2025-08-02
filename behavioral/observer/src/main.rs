// Copyright © 2025 SHAO Liming <lmshao@163.com>
// Licensed under the MIT License
//
// Observer Pattern Example
//
// The observer pattern defines a one-to-many dependency between objects
// so that when one object changes state, all its dependents are notified
// and updated automatically.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

/// Observer trait - defines the interface for objects that should be notified
trait Observer {
    fn update(&self, news: &str);
    fn get_id(&self) -> &str;
}

/// NewsAgency - Subject that notifies observers
struct NewsAgency {
    observers: Arc<Mutex<HashMap<String, Box<dyn Observer + Send + Sync>>>>,
    latest_news: String,
}

impl NewsAgency {
    fn new() -> Self {
        Self {
            observers: Arc::new(Mutex::new(HashMap::new())),
            latest_news: String::new(),
        }
    }

    fn attach(&mut self, observer: Box<dyn Observer + Send + Sync>) {
        let id = observer.get_id().to_string();
        if let Ok(mut observers) = self.observers.lock() {
            observers.insert(id, observer);
        }
    }

    fn detach(&mut self, observer_id: &str) {
        if let Ok(mut observers) = self.observers.lock() {
            observers.remove(observer_id);
        }
    }

    fn notify(&self) {
        if let Ok(observers) = self.observers.lock() {
            for observer in observers.values() {
                observer.update(&self.latest_news);
            }
        }
    }

    fn publish_news(&mut self, news: String) {
        println!("📰 News Agency publishing: {}", news);
        self.latest_news = news;
        self.notify();
    }
}

/// NewsChannel - Concrete Observer
struct NewsChannel {
    id: String,
    name: String,
    received_news: RwLock<Vec<String>>,
}

impl NewsChannel {
    fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            received_news: RwLock::new(Vec::new()),
        }
    }

    fn display_news(&self) {
        if let Ok(news) = self.received_news.read() {
            println!("📺 {} - Latest news: {:?}", self.name, *news);
        }
    }
}

impl Observer for NewsChannel {
    fn update(&self, news: &str) {
        println!("📺 {} received news: {}", self.name, news);
        if let Ok(mut news_vec) = self.received_news.write() {
            news_vec.push(news.to_string());
        }
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

/// NewsWebsite - Concrete Observer
struct NewsWebsite {
    id: String,
    name: String,
    url: String,
}

impl NewsWebsite {
    fn new(id: String, name: String, url: String) -> Self {
        Self { id, name, url }
    }
}

impl Observer for NewsWebsite {
    fn update(&self, news: &str) {
        println!("🌐 {} ({}): Breaking news - {}", self.name, self.url, news);
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

/// MobileApp - Concrete Observer
struct MobileApp {
    id: String,
    name: String,
    user_count: u32,
}

impl MobileApp {
    fn new(id: String, name: String, user_count: u32) -> Self {
        Self {
            id,
            name,
            user_count,
        }
    }
}

impl Observer for MobileApp {
    fn update(&self, news: &str) {
        println!(
            "📱 {} ({} users): Push notification - {}",
            self.name, self.user_count, news
        );
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

fn main() {
    println!("📰 Observer Pattern Example - News Publishing System");
    println!("{}", "=".repeat(50));

    // Create news agency (subject)
    let mut news_agency = NewsAgency::new();

    // Create observers
    let cnn = Box::new(NewsChannel::new("cnn".to_string(), "CNN".to_string()));
    let bbc = Box::new(NewsChannel::new("bbc".to_string(), "BBC".to_string()));
    let reuters_website = Box::new(NewsWebsite::new(
        "reuters".to_string(),
        "Reuters".to_string(),
        "https://reuters.com".to_string(),
    ));
    let news_app = Box::new(MobileApp::new(
        "news_app".to_string(),
        "Breaking News App".to_string(),
        1000000,
    ));

    // Attach observers to subject
    println!("🔗 Attaching observers to news agency...");
    news_agency.attach(cnn);
    news_agency.attach(bbc);
    news_agency.attach(reuters_website);
    news_agency.attach(news_app);

    println!();

    // Publish news and notify all observers
    let news_items = vec![
        "Global tech conference announces breakthrough in AI technology".to_string(),
        "New environmental policy aims to reduce carbon emissions by 50%".to_string(),
        "SpaceX successfully launches new satellite constellation".to_string(),
    ];

    for news in news_items {
        news_agency.publish_news(news);
        println!();
    }

    // Detach an observer
    println!("🔗 Detaching BBC from news agency...");
    news_agency.detach("bbc");

    // Publish another news (BBC won't receive it)
    news_agency.publish_news("Breaking: Major sports event postponed due to weather".to_string());

    println!();

    // Create a new CNN instance to demonstrate the stored news
    let cnn_demo = NewsChannel::new("cnn_demo".to_string(), "CNN Demo".to_string());
    cnn_demo.display_news();

    println!();
    println!("✅ Observer Pattern example completed!");
    println!();
    println!("💡 Design Pattern Key Points:");
    println!("  - NewsAgency is the Subject that maintains observers");
    println!("  - Observer trait defines the notification interface");
    println!("  - NewsChannel, NewsWebsite, MobileApp are concrete observers");
    println!("  - When news is published, all observers are automatically notified");
    println!("  - Observers can be dynamically attached and detached");
    println!(
        "  - NewsChannel uses RwLock for thread-safe interior mutability to store received news"
    );
}
