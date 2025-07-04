# Functional Programming in Monkey

Monkey is designed with functional programming as a first-class paradigm.
Functions are values that can be passed around, stored in variables, and used to
build powerful abstractions. This chapter explores the functional programming
capabilities of Monkey.

## First-Class Functions

In Monkey, functions are first-class citizens, meaning they can be:

- Assigned to variables
- Passed as arguments to other functions
- Returned from functions
- Stored in data structures

```monkey
// Functions as values
let add = fn(a, b) { a + b };
let subtract = fn(a, b) { a - b };

// Functions in arrays
let operations = [add, subtract];

// Using functions from arrays
let result = operations[0](10, 5); // 15
```

## Higher-Order Functions

Higher-order functions are functions that either take other functions as
parameters or return functions as results.

### Functions that Take Functions

```monkey
let applyTwice = fn(f, x) {
  f(f(x));
};

let double = fn(x) { x * 2 };
let result = applyTwice(double, 5); // 20 (5 * 2 * 2)
```

### Functions that Return Functions

```monkey
let makeAdder = fn(x) {
  fn(y) { x + y };
};

let add5 = makeAdder(5);
let result = add5(10); // 15
```

## Closures

Monkey functions are closures, meaning they capture and remember the environment
in which they were created:

```monkey
// `newAdder` returns a closure that makes use of the free variables `a` and `b`:
let newAdder = fn(a, b) {
    fn(c) { a + b + c };
};
// This constructs a new `adder` function:
let adder = newAdder(1, 2);

adder(8); // => 11
```

## Common Functional Patterns

### Map

The `map` function applies a transformation to every element in an array:

```monkey
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

let numbers = [1, 2, 3, 4];
let double = fn(x) { x * 2 };
let doubled = map(numbers, double); // [2, 4, 6, 8]
```

### Filter

The `filter` function selects elements that satisfy a predicate:

```monkey
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

let numbers = [1, 2, 3, 4, 5, 6];
let isMoreThanTwo = fn(x) { x > 2 };
let evens = filter(numbers, isMoreThanTwo); // [3, 4, 5, 6]
```

### Reduce

The `reduce` function combines all elements in an array into a single value:

```monkey
let reduce = fn(arr, initial, f) {
  let iter = fn(arr, result) {
    if (len(arr) == 0) {
      result;
    } else {
      iter(rest(arr), f(result, first(arr)));
    }
  };
  iter(arr, initial);
};

let sum = fn(arr) {
  reduce(arr, 0, fn(acc, x) { acc + x });
};

let product = fn(arr) {
  reduce(arr, 1, fn(acc, x) { acc * x });
};

sum([1, 2, 3, 4, 5]);     // 15
product([1, 2, 3, 4, 5]); // 120
```

## Function Composition

You can compose functions to create more complex operations:

```monkey
let compose = fn(f, g) {
  fn(x) { f(g(x)) };
};

let add1 = fn(x) { x + 1 };
let multiply2 = fn(x) { x * 2 };

let add1ThenMultiply2 = compose(multiply2, add1);
let result = add1ThenMultiply2(5); // (5 + 1) * 2 = 12
```

## Currying

Currying transforms a function that takes multiple arguments into a series of
functions that each take a single argument:

```monkey
let curry = fn(f) {
  fn(a) {
    fn(b) {
      f(a, b);
    };
  };
};

let add = fn(a, b) { a + b };
let curriedAdd = curry(add);

let add5 = curriedAdd(5);
let result = add5(10); // 15
```

## Practical Examples

### Finding Maximum Element

```monkey
let max = fn(arr) {
  if (len(arr) == 0) {
    null;
  } else {
    reduce(rest(arr), first(arr), fn(acc, x) {
      if (x > acc) { x } else { acc }
    });
  }
};

max([3, 1, 4, 1, 5, 9, 2, 6]); // 9
```

### Functional Fibonacci

```monkey
let fibonacci = fn(n) {
  let fib = fn(a, b, count) {
    if (count == 0) {
      a;
    } else {
      fib(b, a + b, count - 1);
    }
  };
  fib(0, 1, n);
};

fibonacci(10); // 55
```

### Pipeline Processing

```monkey
let pipe = fn(value, functions) {
  reduce(functions, value, fn(acc, f) { f(acc) });
};

let numbers = [1, 2, 3, 4, 5];
let pipeline = [
  fn(arr) { map(arr, fn(x) { x * 2 }) },      // double each
  fn(arr) { filter(arr, fn(x) { x > 5 }) },   // keep > 5
  fn(arr) { reduce(arr, 0, fn(a, b) { a + b }) } // sum
];

let result = pipe(numbers, pipeline); // 18
```

## Advantages of Functional Programming in Monkey

1. **Immutability**: Functions don't modify their inputs, reducing side effects
2. **Composability**: Small functions can be combined to create complex behavior
3. **Reusability**: Generic functions like `map` and `filter` work with any data
4. **Testability**: Pure functions are easier to test and reason about
5. **Expressiveness**: Functional code often reads like a description of what
   you want

## Recursive Patterns

Monkey's functional style naturally leads to recursive solutions:

```monkey
// Recursive list processing
let length = fn(arr) {
  if (len(arr) == 0) {
    0;
  } else {
    1 + length(rest(arr));
  }
};

// Recursive tree traversal (conceptual)
let traverse = fn(tree, visitor) {
  if (tree == null) {
    null;
  } else {
    visitor(tree);
    traverse(tree["left"], visitor);
    traverse(tree["right"], visitor);
  }
};
```

Functional programming in Monkey provides a powerful and elegant way to solve
problems by composing simple, reusable functions. The language's support for
closures, higher-order functions, and immutable data structures makes it
well-suited for functional programming patterns.
