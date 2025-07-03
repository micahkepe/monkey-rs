# REPL

The Monkey REPL (Read-Eval-Print Loop) provides an interactive environment for
experimenting with the Monkey programming language. It's perfect for learning,
testing code snippets, and exploring language features.

## Starting the REPL

To start the Monkey REPL, run:

```bash
cargo run
```

You'll be greeted with the Monkey ASCII art and a prompt:

```
       __  ___          __
      /  |/  /__  ___  / /_____ __ __
     / /|_/ / _ \/ _ \/  '_/ -_) // /
    /_/  /_/\___/_//_/_/\_\\__/\_, /
                              /___/

Welcome to the Monkey programming language!
Feel free to type in commands

>>
```

## Basic Usage

The REPL evaluates Monkey expressions and statements as you type them:

```monkey
>> 5 + 3
8
>> let x = 10
>> x * 2
20
>> "Hello, " + "World!"
Hello, World!
```

## Features

### Persistent Environment

Variables and functions defined in the REPL persist throughout your session:

```monkey
>> let name = "Alice"
>> let greet = fn(n) { "Hello, " + n + "!" }
>> greet(name)
Hello, Alice!
```

### Command History

The REPL maintains a command history that persists between sessions. You can:

- Use **Up/Down arrow keys** to navigate through previous commands
- History is saved to `/tmp/.monkey-history.txt`
- History is automatically loaded when you start a new REPL session

### Line Editing

The REPL supports basic line editing features:

- **Left/Right arrows**: Move cursor within the current line
- **Home/End**: Jump to beginning/end of line
- **Backspace/Delete**: Remove characters
- **Ctrl+C**: Exit the REPL
- **Ctrl+D**: Exit the REPL (EOF)

### Multi-line Input

You can enter multi-line expressions and function definitions by using the "\\"
character and pressing Enter. You will then be re-prompted with ".. " to
continue entering your input program:

```monkey
>> let factorial = fn(n) { \
..   if (n <= 1) { \
..     1 \
..   } else { \
..     n * factorial(n - 1) \
..   } \
.. } \
>> factorial(5)
120
```

## Example Session

Here's a complete example session showing various Monkey features:

```monkey
// Define some variables
>> let numbers = [1, 2, 3, 4, 5]
>> let double = fn(x) { x * 2 }

// Use built-in functions
>> len(numbers)
5
>> first(numbers)
1
>> last(numbers)
5

// Higher-order function example
>> let map = fn(arr, f) {
     let iter = fn(arr, accumulated) {
       if (len(arr) == 0) {
         accumulated
       } else {
         iter(rest(arr), push(accumulated, f(first(arr))))
       }
     }
     iter(arr, [])
   }

>> map(numbers, double)
[2, 4, 6, 8, 10]

// Hash map example
>> let person = {"name": "Bob", "age": 30}
>> person["name"]
Bob

// Conditional expressions
>> let status = if (person["age"] >= 18) { "adult" } else { "minor" }
>> status
adult

// Recursive function
>> let fibonacci = fn(n) {
     if (n < 2) {
       n
     } else {
       fibonacci(n - 1) + fibonacci(n - 2)
     }
   }

>> fibonacci(10)
55

>> puts("Session complete!")
Session complete!
```

## Error Handling

The REPL gracefully handles errors and continues running:

```monkey
>> 5 / 0
division by zero

>> let x = [1, 2, 3]
>> x[10]
null
>> unknownFunction()
identifier not found: unknownFunction

>> 2 + 2
4
```

## Tips for Using the REPL

1. **Experiment freely**: The REPL is perfect for trying out language features
2. **Build incrementally**: Define helper functions and test them step by step
3. **Use puts() for debugging**: Print intermediate values to understand your code
4. **Save important code**: Copy useful functions to files for later use
5. **Use history**: Navigate through previous commands with arrow keys

## Exiting the REPL

To exit the REPL, you can:

- Press **Ctrl+C**
- Press **Ctrl+D**
- Type the EOF character

The REPL will save your command history and display:

```
Exiting...
```

## Technical Details

The REPL is implemented using the `rustyline` crate, which provides:

- Line editing capabilities
- Command history management
- Cross-platform terminal handling

The REPL maintains a shared environment across all evaluations, allowing for
persistent state throughout your session. Each input is parsed into an AST and
evaluated using the same interpreter engine that would process Monkey source
files.
