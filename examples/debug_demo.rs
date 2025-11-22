// Example usage of WebRust debugging macros
// This file demonstrates the dd!(), dump!(), and debug!() macros

// ============================================
// Example 1: Using dd!() to stop on error
// ============================================
#[allow(dead_code)]
async fn example_dd() {
    struct User {
        id: i32,
        name: String,
    }

    let user = User {
        id: 1,
        name: "Alice".to_string(),
    };

    // This will dump the user and stop execution
    // Uncomment to try:
    // dd!(user);
}

// ============================================
// Example 2: Using dump!() to continue
// ============================================
#[allow(dead_code)]
async fn example_dump() {
    let data = vec![1, 2, 3, 4, 5];

    // Dump data but continue execution
    let processed = dump!(data);

    println!("Processed: {:?}", processed);
}

// ============================================
// Example 3: Using debug!() with labels
// ============================================
#[allow(dead_code)]
async fn example_debug() {
    struct RequestData {
        method: String,
        path: String,
    }

    let request = RequestData {
        method: "POST".to_string(),
        path: "/api/users".to_string(),
    };

    debug!("incoming_request", request);
}

// ============================================
// Example 4: Debugging in controller
// ============================================
#[allow(dead_code)]
async fn example_in_controller() {
    use axum::{extract::State, response::Html};
    use crate::framework::AppState;
    use tera::Context;

    async fn get_user(State(state): State<AppState>) -> Html<String> {
        debug!("user_endpoint", "fetching user");

        let mut ctx = Context::new();
        ctx.insert("title", "User Profile");

        // Simulate some operation
        let user_id = 1;
        debug!("user_id", user_id);

        let body = state
            .templates
            .render("home/index.rune.html", &ctx)
            .unwrap_or_else(|err| format!("Template error: {err}"));

        Html(body)
    }

    // Note: This is just a demonstration
    // To use, construct AppState and call the function
}

// ============================================
// Example 5: Debugging with multiple values
// ============================================
#[allow(dead_code)]
fn example_multiple() {
    let x = 42;
    let y = "hello";
    let z = vec![1, 2, 3];

    // Dump multiple values at once
    // dd!(x, y, z);  // Uncomment to try

    // Or debug each separately
    debug!("value_x", x);
    debug!("value_y", y);
    debug!("value_z", z);
}

// ============================================
// Example 6: Conditional debugging
// ============================================
#[allow(dead_code)]
fn example_conditional() {
    let result: Result<i32, String> = Ok(42);

    if let Err(e) = &result {
        // Only debug on error
        dd!(e);
    } else {
        // Dump the success case and continue
        let value = dump!(result);
        println!("Result: {:?}", value);
    }
}

// ============================================
// Example 7: Debugging in loops
// ============================================
#[allow(dead_code)]
fn example_loop() {
    let items = vec!["apple", "banana", "cherry"];

    for (index, item) in items.iter().enumerate() {
        debug!(format!("item_{}", index), item);
    }
}

// ============================================
// Example 8: Using dump!() as expression
// ============================================
#[allow(dead_code)]
fn example_dump_expression() {
    fn calculate() -> i32 {
        42
    }

    // Use dump!() to inspect while returning
    let result = dump!(calculate());
    println!("Calculation result: {}", result);
}

/*

OUTPUT EXAMPLES:

1. dd!(user) outputs:
   🐛 DEBUG DUMP:
   User { id: 1, name: "Alice" }
   📍 at: examples/debug_demo.rs:10
   ⏹️  Process exiting...

2. dump!(data) outputs:
   🔍 DEBUG:
   [1, 2, 3, 4, 5]
   📍 at: examples/debug_demo.rs:30

3. debug!("label", value) outputs:
   🔧 [label]
   Value { ... }
   📍 at: examples/debug_demo.rs:50

*/
