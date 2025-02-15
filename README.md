# Jitmind - A JIT-Compiled Expression Evaluator

## Overview
Jitmind is a Just-In-Time (JIT) compiled arithmetic expression evaluator written in Rust. It converts arithmetic expressions from infix notation to postfix using the **Shunting Yard Algorithm** and dynamically generates x86-64 machine code for execution.

## Features
- Supports **basic arithmetic operations**: `+`, `-`, `*`, `/`
- **JIT compilation**: Converts expressions into executable machine code at runtime
- **Uses mmap and mprotect** for **dynamic code execution**
- **Efficient stack-based execution model**

## Installation
Ensure you have Rust installed. Clone the repository and build the project:

```sh
git clone https://github.com/your-repo/jitmind.git
cd jitmind
cargo build --release
```

## Usage
Run the program and input an arithmetic expression:

```sh
cargo run
```

Example:

```
Enter an arithmetic expression: (3 + 4) * 2
JIT-compiled function returned: 14
```

## How It Works
1. **Parsing**: The input is parsed and converted into postfix notation (Reverse Polish Notation) using the **Shunting Yard Algorithm**.
2. **Code Generation**: x86-64 machine code is generated dynamically based on the postfix expression.
3. **Memory Allocation**: `mmap` is used to allocate executable memory.
4. **Execution**: The generated machine code is executed as a function using Rust's `mem::transmute`.
5. **Cleanup**: The allocated memory is deallocated using `munmap`.

## Example Expressions Supported
| Expression       | Result |
|-----------------|--------|
| `(4 + 2) * 3`  | `18`   |
| `10 - 3 * 2`   | `4`    |
| `8 / 2 + 5`    | `9`    |

## Future Improvements
- Support for floating-point operations
- Additional operators (`%`, `^`)
- Variable assignment and function support
- Enhanced error handling

## License
MIT License

## Author
Meet (Jitmind Project Creator)

