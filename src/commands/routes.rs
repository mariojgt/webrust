/// Display all application routes
pub fn list_routes() {
    println!("╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           📍 Application Routes                                ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
    println!();

    let routes = vec![
        ("GET", "/", "HomeController", "index", "Show home page"),
        ("GET", "/users", "UserController", "index", "List all users"),
        ("GET", "/users/create", "UserController", "create", "Show create user form"),
        ("POST", "/users", "UserController", "store", "Store new user"),
        ("GET", "/users/{id}", "UserController", "show", "Show user details"),
        ("GET", "/users/{id}/edit", "UserController", "edit", "Show edit user form"),
        ("PUT", "/users/{id}", "UserController", "update", "Update user"),
        ("DELETE", "/users/{id}", "UserController", "destroy", "Delete user"),
        
        ("GET", "/posts", "PostController", "index", "List all posts"),
        ("GET", "/posts/create", "PostController", "create", "Show create post form"),
        ("POST", "/posts", "PostController", "store", "Store new post"),
        ("GET", "/posts/{id}", "PostController", "show", "Show post details"),
        ("GET", "/posts/{id}/edit", "PostController", "edit", "Show edit post form"),
        ("PUT", "/posts/{id}", "PostController", "update", "Update post"),
        ("DELETE", "/posts/{id}", "PostController", "destroy", "Delete post"),
        
        ("GET", "/api/users", "Api\\UserController", "index", "Get all users (JSON)"),
        ("POST", "/api/users", "Api\\UserController", "store", "Create user (JSON)"),
        ("GET", "/api/users/{id}", "Api\\UserController", "show", "Get user (JSON)"),
        ("PUT", "/api/users/{id}", "Api\\UserController", "update", "Update user (JSON)"),
        ("DELETE", "/api/users/{id}", "Api\\UserController", "destroy", "Delete user (JSON)"),
    ];

    println!("{:<8} {:<25} {:<25} {:<12} {:<30}", "METHOD", "URI", "CONTROLLER", "ACTION", "DESCRIPTION");
    println!("{}", "─".repeat(100));

    for (method, uri, controller, action, description) in routes {
        let method_colored = match method {
            "GET" => "\x1b[32m".to_string() + method + "\x1b[0m",      // Green
            "POST" => "\x1b[33m".to_string() + method + "\x1b[0m",     // Yellow
            "PUT" => "\x1b[34m".to_string() + method + "\x1b[0m",      // Blue
            "DELETE" => "\x1b[31m".to_string() + method + "\x1b[0m",   // Red
            _ => method.to_string(),
        };
        println!(
            "{:<8} {:<25} {:<25} {:<12} {:<30}",
            method, uri, controller, action, description
        );
    }

    println!();
    println!("📊 Summary:");
    println!("  • Total Routes: 20");
    println!("  • GET Routes: 10");
    println!("  • POST Routes: 3");
    println!("  • PUT Routes: 3");
    println!("  • DELETE Routes: 3");
    println!("  • API Routes: 5");
    println!();
    println!("💡 Tip: Use `cargo run -- rune make:resource <name>` to generate resource routes automatically");
}
