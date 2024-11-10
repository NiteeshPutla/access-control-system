# Access Control System in Rust

This project implements a basic access control system in Rust, providing functionalities to manage users, roles, and permissions. It is designed to handle permission checks efficiently while supporting concurrent access using Rust's synchronization primitives.

## Features

- **User Management**: Create and manage users with associated roles.
- **Role Management**: Define roles with specific permissions.
- **Permission Management**: Add and manage permissions easily.
- **Permission Check**: Verify if a user has a particular permission based on their role.
- **Concurrency**: Safe multi-threaded access using `Arc` and `Mutex`.

## Project Structure

### Modules

1. **models.rs**
   - Defines the core data structures: `User`, `Role`, `Permission`, and `AccessControl`.
   - Implements methods to add users, roles, and permissions.
   - Includes a method `check_permission` to check if a user has a specific permission.

2. **main.rs**
   - Demonstrates usage of the `AccessControl` system.
   - Sets up an example with roles, permissions, and users.
   - Uses Rust's `Arc` and `Mutex` for safe concurrent access and spawns a thread to demonstrate permission checks.

## Getting Started

### Prerequisites

- **Rust**: Ensure you have the Rust toolchain installed. Visit [rustup.rs](https://rustup.rs/) for installation instructions.

