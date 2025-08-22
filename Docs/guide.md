
# The Aegis Programming Language
### The Official Guide (v0.1.0-alpha)

Welcome to Aegis! This guide will walk you through the core principles and features of the Aegis language, from basic syntax to building a complete, reactive Android application.

> **A Note on Versioning**
> You are reading the guide for Aegis `v0.1.0-alpha`. The language is in its infancy. Many features described here have a working prototype, but some are part of the grand vision and not yet fully implemented. We will mark these "vision features" and "alpha implementations" clearly.

---
## Part I: Getting Started

### Installation

Getting the Aegis toolchain, **The Forge**, is designed to be a one-line command.

```bash
# Installation via package managers is a future goal.
# For now, you would build the `aegis` CLI from source.
```

### Your First Project
The Forge makes starting a new project simple.
```bash
# 1. Create a new project directory
aegis new MyFirstApp

# 2. Change into the new directory
cd MyFirstApp

# 3. Build the project for the Android target
aegis build --target android
```

This command runs the full compiler pipeline and generates a complete, runnable Android project in the build/ directory.
## Part II: The Aegis Language Tour
### Variables and State
You give data a name using the let's keyword. For building user interfaces, you use let's track to create a reactive state variable. Any part of your UI that uses a tracked variable will automatically and efficiently update when its value changes.

```aegis
# An immutable variable
let's version = 0.1

# A reactive state variable
let's track counter = 0
```

### Data Structures
Aegis comes with a rich set of built-in containers. List, Map, and Set are prototyped.

* **List<T>**: An ordered collection.
```aegis
let's fruits = ["Apple", "Orange"]
fruits.add("Banana")
let's count = fruits.length() // 3
let's first = fruits.get(0) // "Apple"
```

* **Map<K, V>**: A collection of key-value pairs.
```aegis
let's user = { "name": "Manny", "score": 100 }
let's name = user.name // Access with dot notation
let's score = user.get("score") // Access with get method
```

* **Set<T>**: A collection of unique items.
```aegis
let's tags = set("work", "project")
tags.add("work") // No change
let's has_work = tags.contains("work") // true
```

### Control Flow

* **if/else**: Used for conditional logic. It's an expression, so it returns a value.
```aegis
let's age = 20
let's status = if age >= 18: "Adult" else: "Minor"
```

* **for Loops**: Used to iterate over collections. Inside a show block, the compiler intelligently transforms a for loop into a high-performance, native Android RecyclerView.
```aegis
for fruit in fruits:
    text fruit // In UI, creates a TextView for each fruit
```

* **when Expressions**: A powerful pattern matching construct for handling multiple conditions cleanly.
```aegis
let's message = when http_code:
    is 200 => "OK"
    is 404 => "Not Found"
    is else => "Error"
```

### Functions
Define reusable, type-safe logic. The compiler validates parameter and return types.

```aegis
let's add(a: number, b: number) -> number:
    return a + b

let's result = add(5, 10) // 15
```

### Asynchronous Programming
Aegis has first-class support for async/await to handle long-running tasks without blocking the UI. It compiles directly to high-performance Kotlin Coroutines.

```aegis
async let's fetch_user(id: number) -> User:
    # ... makes a network request
    
button "Load" when_clicked:
    # The await call is non-blocking
    let's user = await fetch_user(1)
    status_text = "Welcome, {user.name}"
```

## Part III: Building Android Apps (Project Nexus)
You define your entire application—state, UI, and event handling—in a single, cohesive flow.

```aegis
# 1. Define custom data types with `contract`.
contract Task:
    id: number
    title: string
    is_done: boolean

app TaskManager:
    # 2. Define the app's reactive state.
    let's track tasks: List<Task> = []
    
    # 3. Describe the UI in a `show` block.
    show:
        column:
            text "My Tasks" style HeaderText
            
            for task in tasks:
                row when_clicked:
                    # 4. Mutate state directly in event handlers.
                    task.is_done = not task.is_done
                
                # 5. Styles can be dynamic and react to state.
                text task.title style {
                    text_decoration: if task.is_done: "line-through" else: "none"
                }
```

## Part IV: The Aegis Ecosystem

### The Forge (CLI)
A complete toolchain for managing your project: `aegis new`, `aegis build`, `aegis test`.

### The Bridge (Interop)
Seamlessly call into other language ecosystems. The JavaScript bridge is currently prototyped.

```aegis
let's result = ask_javascript "
    return {my_aegis_variable} * 2;
"
```

### The Language Server
Provides a modern IDE experience with live error-checking, autocompletion, and hover information in editors like VS Code.

