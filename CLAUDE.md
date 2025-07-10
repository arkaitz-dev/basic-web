# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a professional portfolio website built with Rust, Axum web framework, and Maud templating. It demonstrates modern web development with server-side rendering and progressive enhancement using htmx.

## Development Commands

### Run the application
```bash
cargo run
```
The server will start at `http://127.0.0.1:3000`

### Development with auto-reload
```bash
cargo install cargo-watch
cargo watch -x run
```
Automatically recompiles and restarts when files change.

### Build for production
```bash
cargo build --release
```

### Run in release mode
```bash
cargo run --release
```

## Architecture

### Core Dependencies
- **Axum 0.8**: Modern async web framework built on Tokio and Hyper
- **axum-htmx 0.8**: HTMX extractors and middleware with auto-vary feature
- **Tokio 1.0**: Async runtime with full features
- **Tower 0.4**: Middleware and service abstractions
- **Tower-HTTP 0.5**: HTTP-specific middleware (static files, headers)
- **Maud 0.26.0**: Type-safe HTML templating
- **Chrono 0.4.38**: Date/time handling with serde support

### Application Structure
- **main.rs**: Entry point with route definitions, JSON-LD helpers, and htmx request handling
- **views/**: HTML template modules using Maud
  - `layout.rs`: Main layout with navigation and full page rendering
  - `about_view.rs`: About section template
  - `experience_view.rs`: Experience timeline template
  - `projects_view.rs`: Projects showcase template
  - `contact_view.rs`: Contact form with validation
  - `error_view.rs`: Error pages (404, 500)

### Key Features
- **htmx Integration**: Progressive enhancement using axum-htmx extractors (HxRequest)
- **Auto-Vary Headers**: Automatic Vary header management for proper htmx caching
- **Dual Rendering**: Full page rendering for direct access, partial content for htmx requests
- **Form Handling**: Contact form with server-side validation using Axum Form extractor
- **Static Assets**: Served from `/static` with cache headers via Tower-HTTP middleware
- **JSON-LD**: Structured data from `static/data/` files
- **Error Handling**: Custom 404 fallback handler
- **Async/Await**: Full async support throughout the application

### Route Pattern
Each route handler follows this pattern:
```rust
async fn handler(HxRequest(is_htmx): HxRequest) -> impl IntoResponse {
    if is_htmx {
        // Return only content for htmx requests
        into_html_response(views::module::render())
    } else {
        // Return full page for direct access
        into_html_response(views::layout::render_page_with_content("section", content))
    }
}
```

### Static Assets Organization
- `static/css/`: Stylesheets (main.css, light.css, dark.css)
- `static/js/`: JavaScript files (htmx.min.js, main.js, theme-init.js)
- `static/data/`: JSON-LD structured data files

## Development Notes

### Adding New Sections
1. Create new view module in `src/views/`
2. Add module declaration to `src/views/mod.rs`
3. Create async route handler in `main.rs` following the htmx pattern
4. Add navigation link in `layout.rs`
5. Add route to the Router in main function

### Customization
- Update personal information in view templates
- Modify JSON-LD files in `static/data/`
- Adjust styling in CSS files
- The application uses Spanish language by default

### Technical Details
- **HTMX Detection**: Uses axum-htmx HxRequest extractor for type-safe header detection
- **Auto-Vary Middleware**: AutoVaryLayer automatically adds Vary headers for proper caching
- **Response Type**: All handlers return `impl IntoResponse` for flexibility
- **Form Handling**: Uses Axum's Form extractor with serde Deserialize
- **Static Files**: Tower-HTTP ServeDir with cache control headers
- **Error Handling**: Fallback handler for 404 errors

### Middleware Stack
The application uses the following middleware layers:
```rust
.layer(AutoVaryLayer)  // Automatic Vary headers for htmx caching
```

### Security Considerations
- Input validation on contact form
- Static asset cache headers for performance
- Automatic Vary header management prevents caching issues with htmx
- Type-safe htmx header extraction