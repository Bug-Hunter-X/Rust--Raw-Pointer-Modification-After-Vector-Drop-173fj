# Rust Raw Pointer Bug

This repository demonstrates a common error in Rust involving unsafe code and raw pointers. The `bug.rs` file contains code that attempts to modify a vector's element using a raw pointer after the vector has been dropped. This leads to undefined behavior, likely a crash.

The `bugSolution.rs` file provides a corrected version, emphasizing safe memory management practices.

**Key takeaway:** Always be cautious when using raw pointers in Rust. Ensure proper ownership and lifetime management to prevent undefined behavior and security vulnerabilities.