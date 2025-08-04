// Copyright © 2025 SHAO Liming <lmshao@163.com>
// Licensed under the MIT License
//
// Prototype Pattern Example
//
// The prototype pattern allows creating new objects by cloning existing objects
// instead of using constructors. This is particularly useful for creating
// expensive objects or avoiding repetitive initialization.

use std::collections::HashMap;

/// Document type enumeration
///
/// Using enum to avoid dyn trait compatibility issues in Rust
#[derive(Clone)]
enum Document {
    Resume(Resume),
    Report(Report),
}

impl Document {
    /// Clone document
    fn clone_document(&self) -> Document {
        self.clone()
    }

    /// Display document content
    fn display(&self) {
        match self {
            Document::Resume(resume) => resume.display(),
            Document::Report(report) => report.display(),
        }
    }

    /// Get document title
    fn get_title(&self) -> String {
        match self {
            Document::Resume(resume) => resume.get_title(),
            Document::Report(report) => report.get_title(),
        }
    }

    /// Try to get resume reference
    fn as_resume(&self) -> Option<&Resume> {
        match self {
            Document::Resume(resume) => Some(resume),
            _ => None,
        }
    }

    /// Try to get report reference
    fn as_report(&self) -> Option<&Report> {
        match self {
            Document::Report(report) => Some(report),
            _ => None,
        }
    }
}

/// Resume document
#[derive(Clone)]
struct Resume {
    name: String,
    age: u32,
    experience: Vec<String>,
    skills: Vec<String>,
}

impl Resume {
    fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
            experience: Vec::new(),
            skills: Vec::new(),
        }
    }

    fn add_experience(&mut self, experience: String) {
        self.experience.push(experience);
    }

    fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn display(&self) {
        println!("=== Resume ===");
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Experience:");
        for (i, exp) in self.experience.iter().enumerate() {
            println!("  {}. {}", i + 1, exp);
        }
        println!("Skills:");
        for (i, skill) in self.skills.iter().enumerate() {
            println!("  {}. {}", i + 1, skill);
        }
        println!();
    }

    fn get_title(&self) -> String {
        format!("{}'s Resume", self.name)
    }
}

/// Report document
#[derive(Clone)]
struct Report {
    title: String,
    content: String,
    author: String,
    date: String,
}

impl Report {
    fn new(title: String, author: String) -> Self {
        Self {
            title,
            author,
            content: String::new(),
            date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        }
    }

    fn set_content(&mut self, content: String) {
        self.content = content;
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn display(&self) {
        println!("=== Report ===");
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Date: {}", self.date);
        println!("Content: {}", self.content);
        println!();
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }
}

/// Document manager
///
/// Using prototype pattern to manage different types of document templates
struct DocumentManager {
    templates: HashMap<String, Document>,
}

impl DocumentManager {
    fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// Register document template
    fn register_template(&mut self, name: String, document: Document) {
        self.templates.insert(name, document);
    }

    /// Create new document through prototype
    fn create_document(&self, template_name: &str) -> Option<Document> {
        self.templates
            .get(template_name)
            .map(|template| template.clone_document())
    }

    /// List all available templates
    fn list_templates(&self) {
        println!("Available document templates:");
        for (name, template) in &self.templates {
            println!("  - {}: {}", name, template.get_title());
        }
        println!();
    }
}

fn main() {
    println!("Prototype Pattern Example\n");

    // Create document manager
    let mut manager = DocumentManager::new();

    // Create resume template
    let mut resume_template = Resume::new("John Doe".to_string(), 28);
    resume_template.add_experience("ABC Corp - Software Engineer (2020-2023)".to_string());
    resume_template.add_experience("XYZ Inc - Junior Developer (2018-2020)".to_string());
    resume_template.add_skill("Rust".to_string());
    resume_template.add_skill("Python".to_string());
    resume_template.add_skill("JavaScript".to_string());

    // Create report template
    let mut report_template = Report::new(
        "Q4 Sales Report".to_string(),
        "Sales Department".to_string(),
    );
    report_template.set_content("This quarter's sales have reached the target...".to_string());

    // Register templates
    manager.register_template(
        "Resume Template".to_string(),
        Document::Resume(resume_template),
    );
    manager.register_template(
        "Report Template".to_string(),
        Document::Report(report_template),
    );

    // Display available templates
    manager.list_templates();

    // Use prototype to create new documents
    println!("=== Creating New Documents Using Prototype ===");

    // Create resume copy and modify
    if let Some(new_resume_doc) = manager.create_document("Resume Template") {
        if let Some(resume) = new_resume_doc.as_resume() {
            let mut modified_resume = resume.clone();
            modified_resume.set_name("Jane Smith".to_string());
            modified_resume.add_experience("New Corp - Senior Engineer (2023-Present)".to_string());
            modified_resume.add_skill("Go".to_string());

            println!("Original resume template:");
            new_resume_doc.display();

            println!("Modified resume:");
            modified_resume.display();
        }
    }

    // Create report copy and modify
    if let Some(new_report_doc) = manager.create_document("Report Template") {
        if let Some(report) = new_report_doc.as_report() {
            let mut modified_report = report.clone();
            modified_report.set_title("Annual Technical Report".to_string());
            modified_report.set_content("Annual technical development summary...".to_string());

            println!("Original report template:");
            new_report_doc.display();

            println!("Modified report:");
            modified_report.display();
        }
    }

    println!("=== Prototype Pattern Advantages ===");
    println!("1. Avoid repetitive initialization code");
    println!("2. Quickly create copies of complex objects");
    println!("3. Reduce the number of subclasses");
    println!("4. Provide an alternative to inheritance");
    println!("5. Using enum in Rust avoids dyn trait compatibility issues");
}
