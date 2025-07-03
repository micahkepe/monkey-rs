# Syntax

Monkey has a C-like syntax that should feel familiar to programmers coming from
languages like JavaScript, C, or Go. This section covers the fundamental syntax
elements of the Monkey programming language.

## Identifiers

Identifiers in Monkey must start with a letter, followed by any combination of
letters, digits, or underscores:

```monkey
let myVariable = 10;
let counter1 = 0;
```

## Keywords

Monkey has the following reserved keywords:

- `let` - Variable binding
- `fn` - Function definition
- `if` - Conditional statement
- `else` - Alternative branch for conditionals
- `return` - Return statement
- `true` - Boolean true literal
- `false` - Boolean false literal

## Operators

### Arithmetic Operators

```monkey
let a = 5 + 3;    // Addition
let b = 10 - 4;   // Subtraction
let c = 6 * 7;    // Multiplication
let d = 15 / 3;   // Division
```

### Comparison Operators

```monkey
let equal = 5 == 5;        // Equality
let notEqual = 5 != 3;     // Inequality
let less = 3 < 5;          // Less than
let greater = 7 > 4;       // Greater than
```

### Logical Operators

```monkey
let negation = !true;      // Logical NOT
```

## Statements vs Expressions

Monkey distinguishes between statements and expressions:

### Statements

- `let` statements for variable binding
- `return` statements for returning values
- Expression statements (expressions used as statements)

### Expressions

- Literals (numbers, strings, booleans)
- Identifiers
- Prefix expressions (`!`, `-`)
- Infix expressions (`+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`)
- Function calls
- If expressions
- Function literals

## Semicolons

Semicolons are generally optional in Monkey, but they can be used to explicitly
terminate statements:

```monkey
let x = 5
let y = 10;
```

Both lines above are valid. Semicolons are required when you want to put
multiple statements on the same line:

```monkey
let x = 5; let y = 10;
```
