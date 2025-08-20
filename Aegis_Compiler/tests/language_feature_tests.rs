//! Language feature tests for the Aegis compiler
//! Tests specific language constructs and their behavior

use aegis_compiler::{Scribe, Architect, Guardian};
use crate::test_utils::{TestFixtures, tokenize_all};

#[cfg(test)]
mod language_feature_tests {
    use super::*;

    fn compile_successfully(input: &str) -> bool {
        let scribe = Scribe::new(input);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        
        if !architect.errors.is_empty() {
            return false;
        }
        
        let mut guardian = Guardian::new();
        guardian.check_program(&program);
        
        guardian.errors.is_empty()
    }

    #[test]
    fn test_variable_declarations() {
        let test_cases = vec![
            "let's x = 42",
            "let's name = \"John\"",
            "let's is_active = true",
            "let's empty = nothing",
            "let's track counter = 0",
            "let's x: number = 42",
            "let's name: string = \"John\"",
            "let's is_active: boolean = true",
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_function_declarations() {
        let test_cases = vec![
            r#"let's greet():
    return "Hello""#,
            
            r#"let's add(a: number, b: number) -> number:
    return a + b"#,
            
            r#"let's greet_user(name: string) -> string:
    return "Hello, " + name"#,
            
            r#"async let's fetch_data() -> string:
    await sleep(1000)
    return "data""#,
            
            r#"let's complex_function(x: number, y: string, z: boolean) -> string:
    if z:
        return y + " " + x
    else:
        return "default""#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_contract_declarations() {
        let test_cases = vec![
            r#"contract User:
    id: number
    name: string"#,
            
            r#"contract Task:
    id: number
    title: string
    completed: boolean
    priority: number"#,
            
            r#"contract Point:
    x: number
    y: number"#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_app_declarations() {
        let test_cases = vec![
            r#"app SimpleApp:
    show:
        text "Hello""#,
            
            r#"app CounterApp:
    let's track counter = 0
    
    show:
        column:
            text "Count: {counter}"
            button "Increment" when_clicked:
                counter = counter + 1"#,
            
            r#"app ComplexApp:
    let's track items = []
    let's track selected_item = nothing
    
    show:
        column:
            for item in items:
                text item"#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_expressions() {
        let test_cases = vec![
            "let's result = 1 + 2",
            "let's result = 10 - 5",
            "let's result = 3 * 4",
            "let's result = 8 / 2",
            "let's result = x == y",
            "let's result = a != b",
            "let's result = p < q",
            "let's result = r > s",
            "let's result = !true",
            "let's result = -42",
            "let's result = (1 + 2) * 3",
            "let's result = user.name",
            "let's result = add(1, 2)",
            "let's result = obj.method(arg)",
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_control_flow() {
        let test_cases = vec![
            r#"let's result = if true: "yes" else: "no""#,
            
            r#"let's check_number(x: number) -> string:
    if x > 0:
        return "positive"
    else:
        return "non-positive""#,
            
            r#"for item in items:
    text item"#,
            
            r#"let's process_items(items: List<string>):
    for item in items:
        show item"#,
            
            r#"let's categorize(value: number) -> string:
    when value:
        is 0 => "zero"
        is 1 => "one"
        is else => "other""#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_async_await() {
        let test_cases = vec![
            r#"async let's fetch_user(id: number) -> User:
    let's response = await http_get("/users/" + id)
    return parse_user(response)"#,
            
            r#"async let's process_data():
    let's data1 = await fetch_data(1)
    let's data2 = await fetch_data(2)
    return combine(data1, data2)"#,
            
            r#"app AsyncApp:
    let's track status = "idle"
    
    show:
        button "Load" when_clicked:
            status = "loading"
            let's result = await fetch_data()
            status = "loaded""#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_ui_components() {
        let test_cases = vec![
            r#"app UIApp:
    show:
        text "Hello World""#,
            
            r#"app UIApp:
    show:
        column:
            text "Title"
            text "Subtitle""#,
            
            r#"app UIApp:
    show:
        row:
            text "Left"
            text "Right""#,
            
            r#"app UIApp:
    let's track message = "Click me"
    
    show:
        button message when_clicked:
            message = "Clicked!""#,
            
            r#"app UIApp:
    let's track input_text = ""
    
    show:
        input "Enter text" on_change:
            input_text = it"#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_ui_styling() {
        let test_cases = vec![
            r#"app StyledApp:
    show:
        text "Styled" style { font_size: 20 }"#,
            
            r#"app StyledApp:
    show:
        column style { padding: 16, background_color: "#FFFFFF" }:
            text "Content""#,
            
            r#"app StyledApp:
    let's track is_active = true
    
    show:
        text "Dynamic" style {
            color: if is_active: "#000000" else: "#CCCCCC"
        }"#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_event_handling() {
        let test_cases = vec![
            r#"app EventApp:
    let's track counter = 0
    
    show:
        button "Click" when_clicked:
            counter = counter + 1"#,
            
            r#"app EventApp:
    let's track text = ""
    
    show:
        input "Type here" on_change:
            text = it"#,
            
            r#"app EventApp:
    let's track selected = false
    
    show:
        row when_clicked:
            selected = not selected
        text "Item""#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_data_structures() {
        let test_cases = vec![
            r#"let's numbers = [1, 2, 3, 4, 5]"#,
            r#"let's names = ["Alice", "Bob", "Charlie"]"#,
            r#"let's mixed = [1, "two", true, nothing]"#,
            r#"let's user = { id: 1, name: "John", active: true }"#,
            r#"let's config = { 
    host: "localhost", 
    port: 8080, 
    ssl: false 
}"#,
            r#"let's tags = set("work", "important", "urgent")"#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_method_calls() {
        let test_cases = vec![
            r#"let's length = items.length()"#,
            r#"let's first = items.get(0)"#,
            r#"let's contains = items.contains("item")"#,
            r#"let's result = items.add("new_item")"#,
            r#"let's result = items.remove(0)"#,
            r#"let's upper = text.toUpperCase()"#,
            r#"let's substring = text.substring(0, 5)"#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_string_interpolation() {
        let test_cases = vec![
            r#"let's message = "Hello, {name}!""#,
            r#"let's info = "User {user.id}: {user.name}""#,
            r#"let's calculation = "Result: {x + y}""#,
            r#"let's conditional = "Status: {if active: "ON" else: "OFF"}""#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_type_annotations() {
        let test_cases = vec![
            r#"let's x: number = 42"#,
            r#"let's name: string = "John""#,
            r#"let's active: boolean = true"#,
            r#"let's items: List<string> = ["a", "b", "c"]"#,
            r#"let's user: User = { id: 1, name: "John" }"#,
            r#"let's config: Map<string, number> = { "port": 8080 }"#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_reactive_variables() {
        let test_cases = vec![
            r#"app ReactiveApp:
    let's track counter = 0
    let's track message = "Hello"
    let's track items = []
    
    show:
        text "{counter}: {message}""#,
            
            r#"app ReactiveApp:
    let's track x = 10
    let's track y = 20
    let's computed_sum = x + y
    
    show:
        text "Sum: {computed_sum}""#,
        ];
        
        for case in test_cases {
            assert!(compile_successfully(case), "Should compile: {}", case);
        }
    }

    #[test]
    fn test_complex_nested_structures() {
        let input = r#"
contract Address:
    street: string
    city: string
    zip: string

contract User:
    id: number
    name: string
    email: string
    address: Address
    tags: List<string>

let's create_user(name: string, email: string) -> User:
    return {
        id: generate_id(),
        name: name,
        email: email,
        address: {
            street: "",
            city: "",
            zip: ""
        },
        tags: []
    }

app UserManager:
    let's track users: List<User> = []
    let's track selected_user: User = nothing
    
    let's add_user(name: string, email: string):
        let's new_user = create_user(name, email)
        users.add(new_user)
    
    show:
        column:
            text "User Manager"
            for user in users:
                row when_clicked:
                    selected_user = user
                column:
                    text user.name
                    text user.email
                    text "Tags: {user.tags.join(", ")}"
"#;
        
        assert!(compile_successfully(input), "Complex nested structure should compile");
    }
}
