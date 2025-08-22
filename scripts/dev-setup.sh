#!/bin/bash
# Development environment setup script

echo "🔧 Setting up Aegis development environment..."

# Build only working components
echo "🏗️  Building working components..."
cargo build --package aegis-compiler

# Run tests to verify setup
echo "🧪 Running tests..."
cargo test --package aegis-compiler

echo "✅ Development environment setup complete!"
echo ""
echo "Quick commands:"
echo "  cargo build --package aegis-compiler  # Build compiler"
echo "  cargo test --package aegis-compiler   # Run tests"
echo "  cargo clippy --package aegis-compiler # Run linter"
echo "  cargo fmt --all                       # Format code"
