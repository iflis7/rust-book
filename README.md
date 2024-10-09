# Rust Book Projects

This repository contains my projects and exercises from working through the official [Rust Programming Language Book](https://doc.rust-lang.org/book/). The book covers essential Rust programming concepts and builds several hands-on projects that help in solidifying the learning.

## Table of Contents

The book is divided into the following chapters, each of which introduces new Rust programming concepts and exercises.

### 1. Getting Started
- **1.1 Installation**: Instructions on how to install Rust.
- **1.2 Hello, World!**: The classic first program to get familiar with Rust syntax.
- **1.3 Hello, Cargo!**: Introduction to Cargo, Rust's package manager and build system.

### 2. Programming a Guessing Game
- **Guessing Game**: Building a simple command-line game where a user guesses a randomly generated number between 1 and 100.

### 3. Common Programming Concepts
- **3.1 Variables and Mutability**: Learning about mutable and immutable variables.
- **3.2 Data Types**: Overview of Rust's built-in data types.
- **3.3 Functions**: Defining and calling functions in Rust.
- **3.4 Comments**: How to write comments in Rust.
- **3.5 Control Flow**: `if`, `else`, `match`, and other control flow constructs.

### 4. Understanding Ownership
- **4.1 What is Ownership?**: The ownership system, Rust’s most unique feature for memory management.
- **4.2 References and Borrowing**: How references work in Rust.
- **4.3 The Slice Type**: Introduction to the slice type in Rust.

### 5. Using Structs to Structure Related Data
- **5.1 Defining and Instantiating Structs**: Creating and using structs in Rust.
- **5.2 An Example Program Using Structs**: Example program using structs.
- **5.3 Method Syntax**: Defining methods for structs.

### 6. Enums and Pattern Matching
- **6.1 Defining an Enum**: Creating and using enums.
- **6.2 The `match` Control Flow Construct**: Pattern matching with `match`.
- **6.3 Concise Control Flow with `if let`**: Using `if let` for simple pattern matching.

### 7. Managing Growing Projects with Packages, Crates, and Modules
- **7.1 Packages and Crates**: Organizing Rust projects into packages and crates.
- **7.2 Defining Modules**: Using modules to control scope and privacy.
- **7.3 Paths in the Module Tree**: Referencing items in the module tree.
- **7.4 Using the `use` Keyword**: Bringing paths into scope.
- **7.5 Separating Modules into Files**: Organizing code by splitting modules into separate files.

### 8. Common Collections
- **8.1 Vectors**: Dynamic arrays in Rust.
- **8.2 Strings**: UTF-8 encoded strings in Rust.
- **8.3 Hash Maps**: Key-value storage in Rust.

### 9. Error Handling
- **9.1 Unrecoverable Errors with `panic!`**: Handling unrecoverable errors.
- **9.2 Recoverable Errors with `Result`**: Using `Result` for error handling.
- **9.3 To `panic!` or Not to `panic!`**: Deciding between `panic!` and `Result`.

### 10. Generic Types, Traits, and Lifetimes
- **10.1 Generic Data Types**: Using generics in Rust.
- **10.2 Traits**: Defining shared behavior with traits.
- **10.3 Lifetimes**: Ensuring references are valid.

### 11. Writing Automated Tests
- **11.1 Writing Tests**: How to write tests in Rust.
- **11.2 Controlling Test Runs**: Using Cargo to control tests.
- **11.3 Test Organization**: Organizing your tests.

### 12. An I/O Project: Building a Command Line Program
- **12.1 Accepting Command Line Arguments**: Handling command-line arguments.
- **12.2 Reading a File**: Reading from files.
- **12.3 Refactoring for Modularity and Error Handling**: Improving code organization.
- **12.4 Test-Driven Development**: Developing functionality with TDD.
- **12.5 Environment Variables**: Using environment variables.
- **12.6 Error Messages**: Writing errors to standard error.

### 13. Functional Language Features: Iterators and Closures
- **13.1 Closures**: Anonymous functions that capture their environment.
- **13.2 Iterators**: Processing a series of items.
- **13.3 Improving I/O Project**: Enhancing the I/O project using iterators.
- **13.4 Comparing Performance**: Comparing loops and iterators.

### 14. More about Cargo and Crates.io
- **14.1 Customizing Builds**: Build profiles.
- **14.2 Publishing a Crate**: How to publish a crate to Crates.io.
- **14.3 Workspaces**: Managing multiple packages.
- **14.4 Installing Binaries**: Using `cargo install`.
- **14.5 Extending Cargo**: Custom Cargo commands.

### 15. Smart Pointers
- **15.1 Box<T>**: Using heap-allocated data.
- **15.2 Deref Trait**: Treating smart pointers like references.
- **15.3 Drop Trait**: Running code on cleanup.
- **15.4 Rc<T>**: Reference-counted smart pointers.
- **15.5 RefCell<T>**: Interior mutability pattern.
- **15.6 Reference Cycles**: Avoiding memory leaks.

### 16. Fearless Concurrency
- **16.1 Threads**: Running code simultaneously.
- **16.2 Message Passing**: Transferring data between threads.
- **16.3 Shared-State Concurrency**: Sharing data between threads.
- **16.4 Sync and Send Traits**: Concurrency traits.

### 17. Object-Oriented Programming Features of Rust
- **17.1 Characteristics of Object-Oriented Languages**: OOP in Rust.
- **17.2 Trait Objects**: Values of different types using trait objects.
- **17.3 OOP Design Patterns**: Implementing OOP patterns.

### 18. Patterns and Matching
- **18.1 Pattern Matching**: Using patterns in different places.
- **18.2 Refutability**: Whether patterns may fail.
- **18.3 Pattern Syntax**: Syntax for patterns.

### 19. Advanced Features
- **19.1 Unsafe Rust**: Opting out of Rust’s safety guarantees.
- **19.2 Advanced Traits**: More advanced uses of traits.
- **19.3 Advanced Types**: Using advanced types.
- **19.4 Advanced Functions and Closures**: More advanced function features.
- **19.5 Macros**: Writing code that writes code.

### 20. Final Project: Building a Multithreaded Web Server
- **20.1 Single-Threaded Web Server**: Building a basic web server.
- **20.2 Multithreaded Web Server**: Adding multithreading.
- **20.3 Graceful Shutdown**: Handling server shutdown.

### 21. Appendix
- **A - Keywords**: List of Rust keywords.
- **B - Operators and Symbols**: Operators and symbols used in Rust.
- **C - Derivable Traits**: Traits that can be derived automatically.
- **D - Development Tools**: Useful tools for Rust development.
- **E - Editions**: Rust editions.
- **F - Book Translations**: Translations of the Rust book.
- **G - How Rust is Made**: Nightly Rust and the Rust process.

## Running the Projects

To run any project in this repository, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rust-book-projects.git
