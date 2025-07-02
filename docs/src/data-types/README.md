# Data Types

Monkey supports several built-in data types that cover the most common programming needs. Each data type has its own characteristics and supported operations.

## Integers

Integers in Monkey are 64-bit signed integers. They support standard arithmetic operations:

```monkey
let age = 25;
let negative = -42;
let result = 10 + 5 * 2; // 20
```

### Arithmetic Operations

- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`

### Comparison Operations

- Equal: `==`
- Not equal: `!=`
- Less than: `<`
- Greater than: `>`

## Booleans

Monkey has two boolean values: `true` and `false`.

```monkey
let isReady = true;
let isComplete = false;
let comparison = 5 > 3; // true
```

### Boolean Operations

- Logical NOT: `!true` returns `false`
- Equality: `true == true` returns `true`
- Inequality: `true != false` returns `true`

## Strings

Strings in Monkey are sequences of characters enclosed in double quotes:

```monkey
let name = "Alice";
let greeting = "Hello, World!";
let empty = "";
```

### String Operations

- Concatenation is not directly supported with `+`, but you can use built-in functions
- Strings can be compared for equality: `"hello" == "hello"` returns `true`
- String length can be obtained with the `len()` built-in function

## Arrays

Arrays are ordered collections of elements that can contain different data types:

```monkey
let numbers = [1, 2, 3, 4, 5];
let mixed = [1, "hello", true, [1, 2]];
let empty = [];
```

### Array Operations

- **Indexing**: Access elements with `array[index]` (0-based indexing)
- **Length**: Get array length with `len(array)`
- **First element**: Get first element with `first(array)`
- **Last element**: Get last element with `last(array)`
- **Rest**: Get all elements except first with `rest(array)`
- **Push**: Add element to end with `push(array, element)`

```monkey
let arr = [1, 2, 3];
let first = arr[0];        // 1
let length = len(arr);     // 3
let tail = rest(arr);      // [2, 3]
let extended = push(arr, 4); // [1, 2, 3, 4]
```

## Hash Maps

Hash maps (or dictionaries) store key-value pairs. Keys must be hashable types (integers, booleans, or strings):

```monkey
let person = {
  "name": "Alice",
  "age": 30,
  true: "boolean key",
  42: "integer key"
};
```

### Hash Operations

- **Access**: Get values with `hash[key]`
- **Keys**: Can be strings, integers, or booleans
- **Values**: Can be any data type

```monkey
let config = {"debug": true, "port": 8080};
let debugMode = config["debug"];  // true
let port = config["port"];        // 8080
```

## Functions

Functions are first-class values in Monkey, meaning they can be assigned to variables, passed as arguments, and returned from other functions:

```monkey
let add = fn(a, b) {
  a + b;
};

let result = add(5, 3); // 8
```

### Function Characteristics

- Functions are closures (they capture their environment)
- Functions can be anonymous
- Functions can be higher-order (take or return other functions)
- The last expression in a function body is automatically returned

## Null

Monkey has a `null` value to represent the absence of a value:

```monkey
let nothing = null;
let result = if (false) { 42 }; // result is null
```
