<pre>
  ___    __    ____  ___   ____ 
 / __)  /__\  (  _ \(  _)/ ___)
( (__  /(__)\  )___/ )_) \___ \
 \___)(__)(__)(__)  (____/____/
</pre>

# Aegis Programming Language

**Aegis is a universal programming language for joyful, safe, and powerful creation.**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/Heartless-Veteran/Aegis/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

---

Aegis is a new programming language designed from the ground up to be **declarative, safe, and ergonomic**. Its primary goal is to make building beautiful, high-performance, native Android applications radically simple and enjoyable.

### Project Status (v0.2.0-alpha)

Aegis is currently in active development with a solid foundation in place. The project has evolved beyond the initial prototype phase and now includes:

#### ✅ Completed Features
- **Complete Compiler Pipeline**: Lexer (Scribe), Parser (Architect), and Semantic Analyzer (Guardian)
- **Language Core**: Variables (`let's`), functions, control flow, and type system
- **Android UI Framework**: Declarative UI components with reactive state management
- **JavaScript Interoperability**: Working bridge for calling JavaScript from Aegis
- **Language Server**: Basic LSP implementation with syntax highlighting and error reporting
- **Comprehensive Test Suite**: 1,179+ lines of tests covering all compiler components

#### 🚧 Current Focus (v0.5)
- Expanding the standard library with essential container types
- Enhancing semantic analysis and type checking
- Improving developer tooling and IDE integration

#### 🎯 Upcoming (v1.0)
- Complete language feature set with all 19 core container types
- Production-ready Android code generation
- Full package manager and testing framework
- Comprehensive documentation and examples

## Quick Start

```bash
# Clone the repository
git clone https://github.com/Heartless-Veteran/Aegis.git
cd Aegis

# Build the project
cargo build --workspace

# Run the comprehensive test suite  
cargo test --workspace

# Try the cross-platform test runner
cd Aegis_Compiler
./run_tests.sh  # Unix/Linux/macOS
# or run_tests.bat on Windows
```
