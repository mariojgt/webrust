---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "WebRust"
  text: "Laravel-inspired Rust Framework"
  tagline: Build robust web applications with the elegance of Laravel and the power of Rust.
  image:
    src: /webrust/webrust-hero.png
    alt: WebRust Framework
  actions:
    - theme: brand
      text: Get Started
      link: /GUIDE
    - theme: alt
      text: View on GitHub
      link: https://github.com/mariojgt/webrust

features:
  - title: Laravel-like DX
    details: Familiar concepts like Controllers, Middleware, Migrations, and Artisan-style commands (Rune). ✨ Now with Tinker REPL!
  - title: Rust Performance
    details: Built on Axum and Tokio for blazing fast performance and type safety.
  - title: Orbit ORM
    details: An expressive, fluent ORM inspired by Eloquent. Now with 20+ query builder methods!
  - title: Modern Frontend
    details: First-class support for Inertia.js (Vue/React) and Tailwind CSS.
  - title: Event-Driven Architecture
    details: Events, Listeners, and Dispatchers for decoupled, reactive code. Full async support!
  - title: Model Observers
    details: Lifecycle hooks for automatic actions on create, update, delete with Observable trait.
  - title: Authorization Policies
    details: Elegant permission patterns with Policy trait for view, create, update, delete operations.
  - title: Clean Architecture
    details: Repository, Service Patterns, Observers, Policies, and Tinker debugging.
  - title: Testing Tools
    details: Fluent Testing API for integration tests, Factory pattern for test data generation.
  - title: Response Helpers
    details: Laravel-like response helpers for consistent API responses and quick actions.
  - title: Developer Productivity
    details: Interactive Tinker shell, Route:List commands, and Factory patterns for faster development.
---
