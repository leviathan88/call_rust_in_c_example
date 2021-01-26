# Rust in C with FFI

This repo is an example of calling Rust from C code.

In this example we have two functions,

_add(a,b)_ - that adds two numbers

_solve_interleave(s1, s2, s3)_ - that solves Leetcode's 97. [Interleaving String problem](https://leetcode.com/problems/interleaving-string/)

The domain logic is clearly separated from FFI code for better reuse.

Also we are careful when using Strings from C code, we do not allocate new memory for Rust code,
but we reuse the same memory that was allocated in C code, and we solve the problem by using lifetimes.

## Steps
```
# navigage to rust_solver
$ cargo build

# create link to the build from the root of the project
$ sudo ln -s $(pwd)/rust_solver/target/debug/librust_solver.so librust_solver.so

# compile C code
$ gcc main.c -o main -L. -lrust_solver

# Run the code
$ ./main
```





