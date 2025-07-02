# Built-in Functions

Monkey provides several built-in functions that are available globally without any imports. These functions provide essential operations for working with the language's data types.

## Array Functions

### `len(array)`

Returns the length of an array or string.

```monkey
let numbers = [1, 2, 3, 4, 5];
let count = len(numbers); // 5

let text = "Hello";
let textLength = len(text); // 5
```

**Parameters:**
- `array` - An array or string

**Returns:**
- Integer representing the length

**Errors:**
- Throws an error if the argument is not an array or string

### `first(array)`

Returns the first element of an array.

```monkey
let numbers = [10, 20, 30];
let firstNum = first(numbers); // 10

let empty = [];
let firstEmpty = first(empty); // null
```

**Parameters:**
- `array` - An array

**Returns:**
- The first element of the array, or `null` if the array is empty

**Errors:**
- Throws an error if the argument is not an array

### `last(array)`

Returns the last element of an array.

```monkey
let numbers = [10, 20, 30];
let lastNum = last(numbers); // 30

let empty = [];
let lastEmpty = last(empty); // null
```

**Parameters:**
- `array` - An array

**Returns:**
- The last element of the array, or `null` if the array is empty

**Errors:**
- Throws an error if the argument is not an array

### `rest(array)`

Returns a new array containing all elements except the first one.

```monkey
let numbers = [1, 2, 3, 4, 5];
let tail = rest(numbers); // [2, 3, 4, 5]

let single = [42];
let restSingle = rest(single); // []

let empty = [];
let restEmpty = rest(empty); // null
```

**Parameters:**
- `array` - An array

**Returns:**
- A new array with all elements except the first, or `null` if the array is empty

**Errors:**
- Throws an error if the argument is not an array

### `push(array, element)`

Returns a new array with the element added to the end. The original array is not modified.

```monkey
let numbers = [1, 2, 3];
let extended = push(numbers, 4); // [1, 2, 3, 4]
// numbers is still [1, 2, 3]

let mixed = push([1, "hello"], true); // [1, "hello", true]
```

**Parameters:**
- `array` - An array
- `element` - Any value to add to the array

**Returns:**
- A new array with the element appended

**Errors:**
- Throws an error if the first argument is not an array

## Output Functions

### `puts(...args)`

Prints the given arguments to standard output, each on a new line.

```monkey
puts("Hello, World!");
puts(42);
puts(true, "multiple", "arguments");

let name = "Alice";
puts("Hello,", name);
```

**Parameters:**
- `...args` - Any number of arguments of any type

**Returns:**
- `null`

**Notes:**
- Each argument is printed on a separate line
- Objects are converted to their string representation
- Always returns `null`

## Usage Examples

Here are some practical examples of using built-in functions:

### Working with Arrays

```monkey
// Create and manipulate arrays
let numbers = [1, 2, 3, 4, 5];

puts("Array length:", len(numbers));
puts("First element:", first(numbers));
puts("Last element:", last(numbers));
puts("All but first:", rest(numbers));

// Build arrays incrementally
let empty = [];
let withOne = push(empty, 1);
let withTwo = push(withOne, 2);
puts("Built array:", withTwo);
```

### Implementing Higher-Order Functions

```monkey
// Map function using built-ins
let map = fn(arr, f) {
  let iter = fn(arr, accumulated) {
    if (len(arr) == 0) {
      accumulated;
    } else {
      iter(rest(arr), push(accumulated, f(first(arr))));
    }
  };
  iter(arr, []);
};

// Filter function using built-ins
let filter = fn(arr, predicate) {
  let iter = fn(arr, accumulated) {
    if (len(arr) == 0) {
      accumulated;
    } else {
      let head = first(arr);
      let tail = rest(arr);
      if (predicate(head)) {
        iter(tail, push(accumulated, head));
      } else {
        iter(tail, accumulated);
      }
    }
  };
  iter(arr, []);
};

// Usage
let numbers = [1, 2, 3, 4, 5];
let doubled = map(numbers, fn(x) { x * 2 });
let evens = filter(numbers, fn(x) { x % 2 == 0 });

puts("Original:", numbers);
puts("Doubled:", doubled);
puts("Evens:", evens);
```

### String Processing

```monkey
let processText = fn(text) {
  puts("Text:", text);
  puts("Length:", len(text));
  
  if (len(text) > 10) {
    puts("This is a long text");
  } else {
    puts("This is a short text");
  }
};

processText("Hello");
processText("This is a longer string");
```
