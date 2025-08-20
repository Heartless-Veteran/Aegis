use crate::architect::ast::*;
use crate::engine::ail::{Instruction, InstructionSequence};

/// The Lowerer walks the AST and emits AIL instructions.
pub struct Lowerer {
    label_counter: u32,
}

impl Lowerer {
    pub fn new() -> Self {
        Self { label_counter: 0 }
    }

    fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn lower_program(&mut self, program: &Program) -> Vec<InstructionSequence> {
        let mut sequences = Vec::new();
        // In a real compiler, we'd find the main entry point.
        // For now, we flatten all top-level statements into one sequence.
        let mut main_seq = InstructionSequence::new("main");

        for def in &program.definitions {
            match def {
                Definition::Statement(stmt) => self.lower_statement(stmt, &mut main_seq),
                Definition::Function(func_def) => {
                    // Each function gets its own sequence of instructions.
                    let mut func_seq = InstructionSequence::new(&func_def.name);
                    self.lower_statement(&Statement::Block(func_def.body.clone()), &mut func_seq);
                    sequences.push(func_seq);
                }
                _ => {} // App, Contract definitions are handled by codegen directly for now
            }
        }
        sequences.insert(0, main_seq);
        sequences
    }

    fn lower_statement(&mut self, stmt: &Statement, seq: &mut InstructionSequence) {
        match stmt {
            Statement::Let(let_stmt) => {
                self.lower_expression(&let_stmt.value, seq);
                seq.instructions.push(Instruction::Store(let_stmt.name.clone()));
            }
            Statement::Expression(expr_stmt) => {
                self.lower_expression(&expr_stmt.expression, seq);
            }
            Statement::Return(ret_stmt) => {
                self.lower_expression(&ret_stmt.value, seq);
                seq.instructions.push(Instruction::Return);
            }
            Statement::Block(block_stmt) => {
                for s in &block_stmt.statements {
                    self.lower_statement(s, seq);
                }
            }
            _ => {} // For loops are complex and might be handled directly in codegen
        }
    }

    fn lower_expression(&mut self, expr: &Expression, seq: &mut InstructionSequence) {
        match expr {
            Expression::Literal(Literal::Number(n), _) => {
                seq.instructions.push(Instruction::PushI64(n.parse().unwrap_or(0)));
            }
            Expression::Identifier(name, _) => {
                seq.instructions.push(Instruction::Load(name.clone()));
            }
            Expression::Infix(infix_expr) => {
                self.lower_expression(&infix_expr.left, seq);
                self.lower_expression(&infix_expr.right, seq);
                let op = match infix_expr.operator {
                    // ... map operators to AIL instructions ...
                    _ => Instruction::Add, // Placeholder
                };
                seq.instructions.push(op);
            }
            Expression::If(if_expr) => {
                let else_label = self.new_label();
                let end_label = self.new_label();
                self.lower_expression(&if_expr.condition, seq);
                seq.instructions.push(Instruction::JumpIfFalse(else_label.clone()));
                self.lower_statement(&Statement::Block(if_expr.consequence.clone()), seq);
                seq.instructions.push(Instruction::Jump(end_label.clone()));
                seq.instructions.push(Instruction::Label(else_label));
                if let Some(alt) = &if_expr.alternative {
                    self.lower_statement(alt, seq);
                }
                seq.instructions.push(Instruction::Label(end_label));
            }
            _ => {} // Other expression types
        }
    }
}
