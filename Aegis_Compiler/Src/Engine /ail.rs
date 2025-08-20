/// Aegis Intermediate Language Instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // --- Stack Manipulation ---
    PushI64(i64),
    PushBool(bool),
    PushString(String),

    // --- Variable Handling ---
    Store(String), // Store top of stack into a local variable
    Load(String),  // Load a local variable onto the stack

    // --- Arithmetic & Logic---
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    Equals,

    // --- Control Flow ---
    Label(String),       // Marks a destination for jumps, e.g., L0
    Jump(String),        // Unconditional jump to a label
    JumpIfFalse(String), // Jump to a label if the top of the stack is 0 (false)
    
    // --- Function Calls ---
    Call(String), // Call a function by name
    Return,
}

/// A sequence of instructions representing a function or block.
#[derive(Debug, Clone, Default)]
pub struct InstructionSequence {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

impl InstructionSequence {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), instructions: Vec::new() }
    }
}
