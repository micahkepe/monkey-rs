# Features

Monkey is a feature-rich programming language that supports both imperative and functional programming paradigms. This section covers the main features that make Monkey a powerful and expressive language.

## Variable Bindings

Variables in Monkey are created using `let` statements:

```monkey
let name = "Alice";
let age = 30;
let isStudent = false;
```

Variables are immutable once bound - you cannot reassign them:

```monkey
let x = 5;
// x = 10; // This would cause an error
```

## Functions

Functions are first-class citizens in Monkey, supporting both named and anonymous functions.

### Function Definition

```monkey
let add = fn(a, b) {
  a + b;
};

let greet = fn(name) {
  "Hello, " + name + "!";
};
```

### Higher-Order Functions

Functions can take other functions as parameters and return functions:

```monkey
let applyTwice = fn(f, x) {
  f(f(x));
};

let double = fn(x) { x * 2; };
let result = applyTwice(double, 5); // 20
```

### Closures

Functions capture their lexical environment, creating closures:

```monkey
let makeCounter = fn() {
  let count = 0;
  fn() {
    count = count + 1;
    count;
  };
};

let counter = makeCounter();
counter(); // 1
counter(); // 2
```

## Conditional Expressions

Monkey uses `if-else` expressions (not statements) that return values:

```monkey
let max = fn(a, b) {
  if (a > b) {
    a;
  } else {
    b;
  };
};

let status = if (age >= 18) { "adult" } else { "minor" };
```

## Return Statements

Functions can use explicit `return` statements:

```monkey
let factorial = fn(n) {
  if (n <= 1) {
    return 1;
  }
  n * factorial(n - 1);
};
```

## Recursion

Monkey supports recursive function calls:

```monkey
let fibonacci = fn(n) {
  if (n < 2) {
    n;
  } else {
    fibonacci(n - 1) + fibonacci(n - 2);
  }
};
```

## Array Operations

Monkey provides rich array manipulation capabilities:

```monkey
let numbers = [1, 2, 3, 4, 5];

// Functional array operations
let doubled = map(numbers, fn(x) { x * 2 });
let sum = reduce(numbers, 0, fn(acc, x) { acc + x });
let evens = filter(numbers, fn(x) { x % 2 == 0 });
```

## Hash Map Operations

Hash maps provide key-value storage:

```monkey
let person = {
  "name": "Bob",
  "age": 25,
  "city": "New York"
};

let name = person["name"];
let hasAge = "age" in person; // Note: 'in' operator may not be implemented
```

## String Manipulation

While Monkey doesn't have built-in string concatenation with `+`, it provides string operations through built-in functions:

```monkey
let greeting = "Hello";
let name = "World";
// String operations would typically be done through built-in functions
let length = len(greeting); // 5
```

## Error Handling

Monkey handles errors at runtime. Invalid operations will produce error messages:

```monkey
let result = 5 / 0; // Runtime error
let invalid = {}[42]; // Runtime error for invalid hash access
```

## Scoping

Monkey uses lexical scoping with proper variable shadowing:

```monkey
let x = 10;

let outer = fn() {
  let x = 20; // Shadows outer x
  
  let inner = fn() {
    let x = 30; // Shadows both outer x values
    x;
  };
  
  inner(); // Returns 30
};
```

## Expression-Oriented

Most constructs in Monkey are expressions that return values, making the language very composable:

```monkey
let result = if (condition) { 
  fn(x) { x * 2 }(5) 
} else { 
  10 
};
```