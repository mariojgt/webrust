# Inertia.js Integration

WebRust includes built-in support for [Inertia.js](https://inertiajs.com/), allowing you to build modern single-page apps using classic server-side routing and controllers.

## Setup

### 1. Dependencies (`package.json`)

The project is configured with Vue 3, Vite, and Tailwind CSS.

```json
{
  "scripts": {
    "dev": "vite",
    "build": "vite build"
  },
  "dependencies": {
    "@inertiajs/vue3": "^1.0.0",
    "vue": "^3.4.0"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.0.0",
    "tailwindcss": "^3.4.3",
    "vite": "^5.0.0"
  }
}
```

### 2. Vite Configuration (`vite.config.js`)

Vite is configured to compile assets from `resources/js` and `resources/css` into `public/build`.

```javascript
export default defineConfig({
    plugins: [vue()],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, './resources/js'),
        },
    },
    build: {
        outDir: 'public/build',
        emptyOutDir: true,
        manifest: true,
        rollupOptions: {
            input: 'resources/js/app.js',
            output: {
                entryFileNames: 'app.js',
                assetFileNames: 'app.[ext]',
            },
        },
    },
});
```

### 3. Frontend Structure

- `resources/js/app.js`: Main entry point.
- `resources/js/Pages/`: Vue components (Pages).
- `resources/css/app.css`: Tailwind CSS entry.
- `templates/root.rune.html`: The app shell (loads the built assets).

### 4. Running the App

1.  **Install Dependencies**: `npm install`
2.  **Start Dev Server**: `npm run dev` (or `npm run build` for production)
3.  **Run Rust Server**: `cargo run`

The Rust server is configured to serve the `public/build` directory, so the browser can load the compiled assets.

## Usage in Controllers

To render an Inertia page from a controller, use the `Inertia` struct.

```rust
use crate::prelude::*;

pub async fn index(inertia: Inertia) -> impl IntoResponse {
    inertia.render("Home/Index", json!({
        "user": "Mario",
        "framework": "WebRust"
    }))
}
```

### Shared Data

The `share_inertia_data` middleware automatically shares common data with every Inertia response:
- `auth.user`: The currently authenticated user (if any).
- `flash`: Flash messages from the session.

You can access these in your frontend components as props.

**Example (Vue):**

```vue
<script setup>
defineProps({ user: Object })
</script>

<template>
  <div>Hello, {{ $page.props.auth.user.name }}</div>
</template>
```

## Architecture

1.  **Routing**: Defined in `src/routes/web.rs` (standard Rust routes).
2.  **Controllers**: Return `Inertia::render()`.
3.  **Views**: Vue/React components in your frontend folder.
4.  **Middleware**: `src/http/middleware/inertia.rs` handles the protocol details.
