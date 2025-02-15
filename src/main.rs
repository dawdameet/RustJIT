use libc::{mmap, mprotect, munmap, MAP_ANON, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};
use std::collections::VecDeque;
use std::io::{self, Write};
use std::mem;

// Function to convert infix expression to postfix (Shunting Yard Algorithm)
fn infix_to_postfix(expression: &str) -> Vec<String> {
    let mut output = Vec::new();
    let mut operators = VecDeque::new();
    let precedence = |op: &str| match op {
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => 0,
    };

    for token in expression.split_whitespace() {
        match token {
            "(" => operators.push_back(token.to_string()),
            ")" => {
                while let Some(op) = operators.pop_back() {
                    if op == "(" {
                        break;
                    }
                    output.push(op);
                }
            }
            "+" | "-" | "*" | "/" => {
                while let Some(op) = operators.back() {
                    if precedence(op) >= precedence(token) {
                        output.push(operators.pop_back().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push_back(token.to_string());
            }
            _ => output.push(token.to_string()),
        }
    }

    while let Some(op) = operators.pop_back() {
        output.push(op);
    }
    output
}

// Function to generate machine code dynamically
fn generate_machine_code(postfix: &[String]) -> Vec<u8> {
    let mut code = vec![];
    let mut stack_offset = 0;

    for token in postfix {
        match token.as_str() {
            "+" => {
                code.extend(&[0x58, 0x5A, 0x01, 0xD0]); // pop rax, pop rdx, add eax, edx
            }
            "-" => {
                code.extend(&[0x58, 0x5A, 0x29, 0xD0]); // pop rax, pop rdx, sub eax, edx
            }
            "*" => {
                code.extend(&[0x58, 0x5A, 0xF7, 0xEA]); // pop rax, pop rdx, imul eax, edx
            }
            "/" => {
                code.extend(&[0x58, 0x5A, 0x99, 0xF7, 0xFA]); // pop rax, pop rdx, idiv eax, edx
            }
            num => {
                if let Ok(value) = num.parse::<i32>() {
                    code.extend(&[0x68]); // push immediate
                    code.extend(&value.to_le_bytes());
                    stack_offset += 4;
                } else {
                    panic!("Invalid numeric value: {}", num);
                }
            }
        }
    }

    code.push(0xC3); // ret
    code
}

fn main() {
    print!("Enter an arithmetic expression: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let postfix = infix_to_postfix(input);
    let code = generate_machine_code(&postfix);

    // Allocate executable memory
    let mem = unsafe {
        mmap(
            std::ptr::null_mut(),
            code.len(),
            PROT_READ | PROT_WRITE | PROT_EXEC,
            MAP_PRIVATE | MAP_ANON,
            -1,
            0,
        )
    } as *mut u8;

    if mem.is_null() {
        panic!("Failed to allocate memory");
    }

    unsafe {
        std::ptr::copy_nonoverlapping(code.as_ptr(), mem, code.len());
        mprotect(mem as *mut _, code.len(), PROT_EXEC); // Make it executable
    }

    // Cast the memory to a function pointer
    let jit_fn: extern "C" fn() -> i32 = unsafe { mem::transmute(mem) };

    // Execute the JIT-compiled function
    let result = jit_fn();
    println!("JIT-compiled function returned: {}", result);

    // Cleanup
    unsafe { munmap(mem as *mut _, code.len()) };
}
