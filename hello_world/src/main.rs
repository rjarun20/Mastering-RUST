#![allow(dead_code, unused_imports, unused)]

use std::mem;

fn main() {
    println!("Hello, world!");

    // core data types
    let x: u8 = 42; // unsigned 8-bit integer (0-255)
    println!("x = {}", x); // immutable variable (cannot be reassigned) 
    let mut x: i8 = -42; // signed 8-bit integer (-128-127) (mutable variable)
    println!("x = {}", x);
    x = 43;
    println!("x = {}", x); // reassigning a mutable variable

    let y: f64 = 13.37; // 64-bit floating point number (double precision)

    let z: bool = true; // boolean (true/false)

    let a: char = 'a'; // character (single unicode character)

    let b: isize = 123; // signed integer (platform dependent) (32/64 bit) (default)
    let c: usize = 123; // unsigned integer (platform dependent) (32/64 bit) (default)
    //print value and bytes
    println!("{} {} {}", a, b, c);
    println!("bytes taken {} {} {}", mem::size_of_val(&a), mem::size_of_val(&b), mem::size_of_val(&c));
    //print os bits
    println!("{}-bit OS", mem::size_of::<isize>() * 8);

    // compound data types
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 integers (type signature is optional)
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple of an integer, float, and unsigned integer
    let (d, e, f) = tup; // destructuring a tuple into individual variables

    // string types
    let str1: &str = "Hello, world!"; // string slice (immutable) (UTF-8)
    let str2: String = str1.to_string(); // string (heap allocated) (UTF-8)


    //deep dive into operators
    let g = 2 + 3 * 4; // *, /, % have higher precedence than + and -, and all are left-associative.
    let h = 2 + 3;
    let i = 2 - 3;
    let j = 2 * 3;
    let k = 2 / 3;
    let l = 2 % 3;
    let m = 2 ^ 3; // bitwise XOR (exclusive OR) example: 10 ^ 11 = 01 (2 ^ 3 = 1) 
    let n = 1 << 10; // bitwise shift left (1 * 2^10) example: 1 << 10 = 1024 
    let o = 1 >> 10; // bitwise shift right (1 / 2^10) example: 1 >> 10 = 0
    let p = true && false; // logical AND (short-circuiting) example: true && false = false
    let q = true || false; // logical OR (short-circuiting)  example: true || false = true
    let r = !true; // logical NOT (unary operator) example: !true = false

    // squrared and cubed
    let squared = i32::pow(2, 2);
    let cubed = i32::pow(2, 3);
    //powi and powf
    let powi = 2_i32.pow(2); // 2^2 = 4 
    let powf = 2.0_f64.powf(2.0); // 2.0^2.0 = 4.0


    // bitwise
    let bitwise_and = 1 & 2; // bitwise AND (1 & 2 = 0) (01 & 10 = 00) 
    let bitwise_or = 1 | 2; // bitwise OR (1 | 2 = 3) (01 | 10 = 11)
    let bitwise_xor = 1 ^ 2;// bitwise XOR (1 ^ 2 = 3) (01 ^ 10 = 11)
    let bitwise_not = !1; // bitwise NOT (!1 = -2) (!01 = 10) 
    let bitwise_shift_left = 1 << 10; // bitwise shift left (1 << 10 = 1024) (1 * 2^10)
    let bitwise_shift_right = 1 >> 10; // bitwise shift right (1 >> 10 = 0) (1 / 2^10) 

    // logical
    let logical_and = true && false; // logical AND (true && false = false)
    let logical_or = true || false; // logical OR (true || false = true)
    let logical_not = !true; // logical NOT (!true = false)

    // comparison
    let is_equal = 1 == 2; // is equal to (1 == 2 = false)
    let is_not_equal = 1 != 2; // is not equal to (1 != 2 = true)
    let is_greater_than = 1 > 2; // is greater than (1 > 2 = false)
    let is_less_than = 1 < 2; // is less than (1 < 2 = true)
    let is_greater_than_or_equal = 1 >= 2; // is greater than or equal to (1 >= 2 = false)
    let is_less_than_or_equal = 1 <= 2; // is less than or equal to (1 <= 2 = true)


    // NOT SUPPORTED IN RUST
    /*
    RUST does not have ternary operator (condition ? true : false)\
    //RUST does not have increment or decrement operators (++ --) (use += -= *= /= %=)
    //RUST does not have pointer arithmetic (unsafe rust has it) (use references) (use raw pointers) (use smart pointers) (use iterators) (use slices) (use arrays) (use vectors) (use hashmaps) (use sets) (use queues) (use stacks) (use linked lists) (use trees) (use graphs) (use heaps) (use tries (prefix trees))
    //RUST does not have null (use Option<T> or Result<T, E>)
    //RUST does not have undefined (use Option<T> or Result<T, E>)
    //RUST does not have void (use () or empty tuple)
    //RUST does not have function overloading (use traits)
    //RUST does not have default arguments (use Option<T> or Result<T, E>)
    //RUST does not have named arguments (use structs)
    //RUST does not have function pointers (use closures)
    //RUST does not have goto (use loop)
    //RUST does not have exceptions (use Result<T, E>)
    //RUST does not have checked exceptions (use Result<T, E>)
    //RUST does not have unchecked exceptions (use Result<T, E>)
    //RUST does not have checked exceptions (use Result<T, E>)

    */

    //scope and shadowing
    //define: a scope is the range within a program for which a variable is valid (in Rust, a scope is determined by the {} braces)
    //define: shadowing is the process of reusing the same variable name within a scope (the inner variable "shadows" the outer variable)

    let a = 123;
    {
        let b = 456;
        println!("inside, b = {}", b);
        let a = 789; // shadowing
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);

    //declaring and using constants
    //define: a constant is an immutable variable that is always valid for the entire duration of a program
    //define: a constant is declared using the const keyword
    //define: a constant must have an explicit type annotation
    //define: a constant can be declared in any scope, including the global scope
    //define: a constant is valid for the entire duration of a program
    //define: a constant is not stored on the stack or heap, but is directly hard-coded into the program
    //code
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    //static variable
    //define: a static variable is a global variable that is always valid for the entire duration of a program
    //code
    static HELLO: &str = "Hello, world!";
    println!("{}", HELLO);

    //n fixed address vs fixed address
    //define: a fixed address is a memory address that is known at compile time
    //define: a fixed address is a memory address that is hard-coded into the program
    //define: a fixed address is a memory address that is not stored on the stack or heap
    //define: a fixed address is a memory address that is directly accessed by the program
    //define: a fixed address is a memory address that is not subject to change
    //define: a fixed address is a memory address that is not subject to relocation
    //define: a fixed address is a memory address that is not subject to garbage collection
    //define: a fixed address is a memory address that is not subject to memory leaks
    //define: a fixed address is a memory address that is not subject to segmentation faults
    //define: a fixed address is a memory address that is not subject to buffer overflows
    //define: a fixed address is a memory address that is not subject to stack smashing
    //define: a fixed address is a memory address that is not
    //code
    // let address = 0x01234567 as *const i32; // fixed address (hard-coded into the program)
    // let value = unsafe { *address }; // dereference a raw pointer (unsafe) (dereference a fixed address) 
    // println!("value = {}", value);

    let heap_value = Box::new(42);

    // Get a raw pointer to the heap-allocated value
    let address = &*heap_value as *const i32; // This address is properly aligned

    // Dereference the raw pointer within an unsafe block
    let value = unsafe { *address };
    println!("value = {}", value);

    //stack and heap
    //definition: the stack is a region of memory that is used for static memory allocation (fixed size) (LIFO). 
    //definition: the heap is a region of memory that is used for dynamic memory allocation (variable size) (FIFO).
    //definition: the stack is fast (constant time) (automatic memory management).
    //definition: the heap is slow (linear time) (manual memory management).
    //definition: the stack is limited in size (1MB-8MB).
    //definition: the heap is unlimited in size (limited by physical memory).
    //definition: the stack is thread-safe (shared memory).
    //definition: the heap is not thread-safe (exclusive memory).
    //definition: the stack is contiguous (adjacent memory).
    //definition: the heap is fragmented (non-adjacent memory).
    //definition: the stack is allocated at compile time (static memory allocation).
    //definition: the heap is allocated at runtime (dynamic memory allocation).
    //definition: the stack is deallocated at the end of a function (automatic memory management).
    //definition: the heap is deallocated manually (manual memory management).
    //definition: the stack is used for local variables (automatic memory management).
    //definition: the heap is used for global variables (manual memory management).
    //definition: the stack is used for function calls (automatic memory management).
    //definition: the heap is used for data structures (manual memory management).
    //definition: the stack is used for recursion (automatic memory management).
    //definition: the heap is used for linked lists (manual memory management).
    //definition: the stack is used for arrays (automatic memory management).
    //definition: the heap is used for trees (manual memory management).
    //code
    let stack = 42; // stack (static memory allocation)
    let heap = Box::new(42); // heap (dynamic memory allocation)
    println!("stack = {}", stack);
    println!("heap = {}", heap);

    struct Point {
        x: i32,
        y: i32,
    }
    //stack and heap store
    let p1 = Point { x: 0, y: 0 };
    let p2 = Box::new(Point { x: 0, y: 0 });
    println!("p1 = ({}, {})", p1.x, p1.y);
    println!("p2 = ({}, {})", p2.x, p2.y);

    let p3 = *p2;
    println!("p3 = ({}, {})", p3.x, p3.y);
    









}
