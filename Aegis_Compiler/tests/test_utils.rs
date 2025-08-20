//! Test utilities and common fixtures for the Aegis compiler test suite

use aegis_compiler::{Scribe, Token, Span};
use std::time::{Duration, Instant};

/// Test fixture for common Aegis code samples
pub struct TestFixtures;

impl TestFixtures {
    /// Simple variable declaration
    pub fn simple_let() -> &'static str {
        "let's x = 42"
    }

    /// Tracked variable declaration
    pub fn tracked_variable() -> &'static str {
        "let's track counter = 0"
    }

    /// Function definition
    pub fn simple_function() -> &'static str {
        r#"let's add(a: number, b: number) -> number:
    return a + b"#
    }

    /// Async function definition
    pub fn async_function() -> &'static str {
        r#"async let's fetch_data(id: number) -> string:
    await sleep(1000)
    return "data""#
    }

    /// Contract definition
    pub fn simple_contract() -> &'static str {
        r#"contract User:
    id: number
    name: string
    active: boolean"#
    }

    /// App definition with UI
    pub fn simple_app() -> &'static str {
        r#"app MyApp:
    let's track message = "Hello"
    
    show:
        column:
            text message
            button "Click me" when_clicked:
                message = "Clicked!""#
    }

    /// Complex app with multiple features
    pub fn complex_app() -> &'static str {
        r#"contract Task:
    id: number
    title: string
    completed: boolean

app TaskManager:
    let's track tasks: List<Task> = []
    let's track input_text = ""
    
    let's add_task(title: string):
        let's new_task: Task = {
            id: tasks.length() + 1,
            title: title,
            completed: false
        }
        tasks.add(new_task)
    
    show:
        column:
            text "Task Manager"
            for task in tasks:
                row when_clicked:
                    task.completed = not task.completed
                text task.title"#
    }
}

/// Performance measurement utilities
pub struct PerformanceTimer {
    start: Instant,
}

impl PerformanceTimer {
    pub fn new() -> Self {
        Self { start: Instant::now() }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.elapsed().as_millis()
    }
}

/// Helper function to tokenize input and collect all tokens
pub fn tokenize_all(input: &str) -> Vec<Token> {
    let mut scribe = Scribe::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = scribe.next_token();
        let is_eof = matches!(token, Token::Eof(_));
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    
    tokens
}

/// Helper function to count specific token types
pub fn count_token_type(tokens: &[Token], target_type: fn(&Token) -> bool) -> usize {
    tokens.iter().filter(|token| target_type(token)).count()
}

/// Helper to create a span for testing
pub fn test_span(start: usize, end: usize) -> Span {
    Span { start, end }
}
