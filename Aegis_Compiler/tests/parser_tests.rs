//! Parser (Architect) tests for the Aegis compiler

use aegis_compiler::{Scribe, Architect, ast::*};
use crate::test_utils::TestFixtures;

#[cfg(test)]
mod parser_tests {
    use super::*;

    fn parse_program(input: &str) -> Program {
        let scribe = Scribe::new(input);
        let mut architect = Architect::new(scribe);
        architect.parse_program()
    }

    #[test]
    fn test_parse_simple_let_statement() {
        let program = parse_program(TestFixtures::simple_let());
        
        assert_eq!(program.definitions.len(), 1);
        
        if let Definition::Statement(Statement::Let(let_stmt)) = &program.definitions[0] {
            assert_eq!(let_stmt.name, "x");
            assert!(!let_stmt.is_tracked);
            assert!(let_stmt.type_annotation.is_none());
            
            if let Expression::Literal(Literal::Number(ref num), _) = let_stmt.value {
                assert_eq!(num, "42");
            } else {
                panic!("Expected number literal");
            }
        } else {
            panic!("Expected let statement");
        }
    }

    #[test]
    fn test_parse_tracked_variable() {
        let program = parse_program(TestFixtures::tracked_variable());
        
        assert_eq!(program.definitions.len(), 1);
        
        if let Definition::Statement(Statement::Let(let_stmt)) = &program.definitions[0] {
            assert_eq!(let_stmt.name, "counter");
            assert!(let_stmt.is_tracked);
            
            if let Expression::Literal(Literal::Number(ref num), _) = let_stmt.value {
                assert_eq!(num, "0");
            } else {
                panic!("Expected number literal");
            }
        } else {
            panic!("Expected let statement");
        }
    }

    #[test]
    fn test_parse_function_definition() {
        let program = parse_program(TestFixtures::simple_function());
        
        assert_eq!(program.definitions.len(), 1);
        
        if let Definition::Function(func_def) = &program.definitions[0] {
            assert_eq!(func_def.name, "add");
            assert!(!func_def.is_async);
            assert_eq!(func_def.parameters.len(), 2);
            assert_eq!(func_def.return_type, Some("number".to_string()));
            
            // Check parameters
            assert_eq!(func_def.parameters[0].name, "a");
            assert_eq!(func_def.parameters[0].type_annotation, "number");
            assert_eq!(func_def.parameters[1].name, "b");
            assert_eq!(func_def.parameters[1].type_annotation, "number");
            
            // Check function body
            assert_eq!(func_def.body.statements.len(), 1);
            if let Statement::Return(return_stmt) = &func_def.body.statements[0] {
                if let Expression::Infix(infix_expr) = &return_stmt.value {
                    // Should be a + b
                    assert!(matches!(infix_expr.operator, InfixOperator::Plus));
                }
            }
        } else {
            panic!("Expected function definition");
        }
    }

    #[test]
    fn test_parse_async_function() {
        let program = parse_program(TestFixtures::async_function());
        
        assert_eq!(program.definitions.len(), 1);
        
        if let Definition::Function(func_def) = &program.definitions[0] {
            assert_eq!(func_def.name, "fetch_data");
            assert!(func_def.is_async);
            assert_eq!(func_def.parameters.len(), 1);
            assert_eq!(func_def.return_type, Some("string".to_string()));
            
            // Check parameter
            assert_eq!(func_def.parameters[0].name, "id");
            assert_eq!(func_def.parameters[0].type_annotation, "number");
            
            // Function body should contain await expression
            let has_await = func_def.body.statements.iter().any(|stmt| {
                matches!(stmt, Statement::Expression(ExpressionStatement { expression: Expression::Await(_), .. }))
            });
            assert!(has_await);
        } else {
            panic!("Expected async function definition");
        }
    }

    #[test]
    fn test_parse_contract_definition() {
        let program = parse_program(TestFixtures::simple_contract());
        
        assert_eq!(program.definitions.len(), 1);
        
        if let Definition::Contract(contract_def) = &program.definitions[0] {
            assert_eq!(contract_def.name, "User");
            assert_eq!(contract_def.fields.len(), 3);
            
            // Check fields
            assert_eq!(contract_def.fields[0].name, "id");
            assert_eq!(contract_def.fields[0].type_annotation, "number");
            
            assert_eq!(contract_def.fields[1].name, "name");
            assert_eq!(contract_def.fields[1].type_annotation, "string");
            
            assert_eq!(contract_def.fields[2].name, "active");
            assert_eq!(contract_def.fields[2].type_annotation, "boolean");
        } else {
            panic!("Expected contract definition");
        }
    }

    #[test]
    fn test_parse_app_definition() {
        let program = parse_program(TestFixtures::simple_app());
        
        assert_eq!(program.definitions.len(), 1);
        
        if let Definition::App(app_def) = &program.definitions[0] {
            assert_eq!(app_def.name, "MyApp");
            
            // Should have tracked variable in body
            assert_eq!(app_def.body.statements.len(), 1);
            if let Statement::Let(let_stmt) = &app_def.body.statements[0] {
                assert_eq!(let_stmt.name, "message");
                assert!(let_stmt.is_tracked);
            }
            
            // Should have show block
            assert!(app_def.body.show_block.is_some());
            if let Some(show_block) = &app_def.body.show_block {
                if let UiNode::Element(ui_element) = &show_block.root_node {
                    assert_eq!(ui_element.name, "column");
                    assert_eq!(ui_element.children.len(), 2); // text and button
                }
            }
        } else {
            panic!("Expected app definition");
        }
    }

    #[test]
    fn test_parse_expressions() {
        let test_cases = vec![
            ("42", |expr| matches!(expr, Expression::Literal(Literal::Number(ref n), _) if n == "42")),
            ("\"hello\"", |expr| matches!(expr, Expression::Literal(Literal::String(ref s), _) if s == "hello")),
            ("true", |expr| matches!(expr, Expression::Literal(Literal::Boolean(true), _))),
            ("false", |expr| matches!(expr, Expression::Literal(Literal::Boolean(false), _))),
            ("nothing", |expr| matches!(expr, Expression::Literal(Literal::Nothing, _))),
            ("my_var", |expr| matches!(expr, Expression::Identifier(ref s, _) if s == "my_var")),
        ];
        
        for (input, validator) in test_cases {
            let full_input = format!("let's x = {}", input);
            let program = parse_program(&full_input);
            
            if let Definition::Statement(Statement::Let(let_stmt)) = &program.definitions[0] {
                assert!(validator(&let_stmt.value), "Failed for input: {}", input);
            } else {
                panic!("Expected let statement for input: {}", input);
            }
        }
    }

    #[test]
    fn test_parse_infix_expressions() {
        let test_cases = vec![
            ("1 + 2", InfixOperator::Plus),
            ("5 - 3", InfixOperator::Minus),
            ("4 * 6", InfixOperator::Multiply),
            ("8 / 2", InfixOperator::Divide),
            ("x == y", InfixOperator::Equal),
            ("a != b", InfixOperator::NotEqual),
            ("p < q", InfixOperator::LessThan),
            ("r > s", InfixOperator::GreaterThan),
        ];
        
        for (input, expected_op) in test_cases {
            let full_input = format!("let's result = {}", input);
            let program = parse_program(&full_input);
            
            if let Definition::Statement(Statement::Let(let_stmt)) = &program.definitions[0] {
                if let Expression::Infix(infix_expr) = &let_stmt.value {
                    assert_eq!(infix_expr.operator, expected_op, "Failed for input: {}", input);
                } else {
                    panic!("Expected infix expression for input: {}", input);
                }
            } else {
                panic!("Expected let statement for input: {}", input);
            }
        }
    }

    #[test]
    fn test_parse_prefix_expressions() {
        let test_cases = vec![
            ("-5", PrefixOperator::Minus),
            ("!true", PrefixOperator::Not),
        ];
        
        for (input, expected_op) in test_cases {
            let full_input = format!("let's result = {}", input);
            let program = parse_program(&full_input);
            
            if let Definition::Statement(Statement::Let(let_stmt)) = &program.definitions[0] {
                if let Expression::Prefix(prefix_expr) = &let_stmt.value {
                    assert_eq!(prefix_expr.operator, expected_op, "Failed for input: {}", input);
                } else {
                    panic!("Expected prefix expression for input: {}", input);
                }
            } else {
                panic!("Expected let statement for input: {}", input);
            }
        }
    }

    #[test]
    fn test_parse_call_expression() {
        let input = "let's result = add(1, 2)";
        let program = parse_program(input);
        
        if let Definition::Statement(Statement::Let(let_stmt)) = &program.definitions[0] {
            if let Expression::Call(call_expr) = &let_stmt.value {
                if let Expression::Identifier(ref func_name, _) = *call_expr.function {
                    assert_eq!(func_name, "add");
                    assert_eq!(call_expr.arguments.len(), 2);
                } else {
                    panic!("Expected identifier in call expression");
                }
            } else {
                panic!("Expected call expression");
            }
        } else {
            panic!("Expected let statement");
        }
    }

    #[test]
    fn test_parse_member_access() {
        let input = "let's name = user.name";
        let program = parse_program(input);
        
        if let Definition::Statement(Statement::Let(let_stmt)) = &program.definitions[0] {
            if let Expression::MemberAccess(member_expr) = &let_stmt.value {
                if let Expression::Identifier(ref obj_name, _) = *member_expr.object {
                    assert_eq!(obj_name, "user");
                    assert_eq!(member_expr.property, "name");
                } else {
                    panic!("Expected identifier in member access");
                }
            } else {
                panic!("Expected member access expression");
            }
        } else {
            panic!("Expected let statement");
        }
    }

    #[test]
    fn test_parse_if_expression() {
        let input = "let's result = if x > 0: \"positive\" else: \"non-positive\"";
        let program = parse_program(input);
        
        if let Definition::Statement(Statement::Let(let_stmt)) = &program.definitions[0] {
            if let Expression::If(if_expr) = &let_stmt.value {
                // Should have condition, then_branch, and else_branch
                assert!(matches!(*if_expr.condition, Expression::Infix(_)));
                assert!(matches!(*if_expr.then_branch, Expression::Literal(Literal::String(_), _)));
                assert!(if_expr.else_branch.is_some());
            } else {
                panic!("Expected if expression");
            }
        } else {
            panic!("Expected let statement");
        }
    }

    #[test]
    fn test_parse_for_statement() {
        let input = r#"for item in items:
    text item"#;
        let program = parse_program(input);
        
        if let Definition::Statement(Statement::For(for_stmt)) = &program.definitions[0] {
            assert_eq!(for_stmt.variable_name, "item");
            if let Expression::Identifier(ref collection_name, _) = for_stmt.collection {
                assert_eq!(collection_name, "items");
            } else {
                panic!("Expected identifier for collection");
            }
            
            // Body should be a block statement
            assert!(matches!(*for_stmt.body, Statement::Block(_)));
        } else {
            panic!("Expected for statement");
        }
    }
}
