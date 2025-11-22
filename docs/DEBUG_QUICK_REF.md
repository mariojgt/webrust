# 🐛 Debugging in WebRust

WebRust comes with Laravel-inspired debugging helpers available anywhere in your application.

## Available Macros

### 1. `dd!` (Dump and Die)
Dumps the value and **stops execution** immediately (exits the process).
Useful for checking values and stopping the script.

```rust
use crate::dd;

// Dump a string
dd!("Reached this point!");

// Dump a variable
let user_id = 42;
dd!(user_id);

// Dump multiple values
dd!(user, posts, "check");
```

### 2. `dump!` (Dump and Continue)
Dumps the value but **continues execution**.
Useful for logging values without stopping the server.

```rust
use crate::dump;

let result = calculate_something();
dump!(result); // Prints to terminal, code continues
```

It also returns the value, so you can wrap expressions:
```rust
let x = dump!(1 + 1); // x is 2, prints "2"
```

### 3. `debug!` (Labeled Debug)
Dumps a value with a custom label.

```rust
use crate::debug;

debug!("User Data", user);
debug!("SQL Query", query_string);
```

## How to use

Just import them at the top of your file:

```rust
use crate::{dd, dump};
```

Or use the prelude:

```rust
use crate::prelude::*;
```
