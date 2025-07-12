# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a professional portfolio website built with Rust, Axum web framework, and Maud templating. It demonstrates modern web development with server-side rendering and progressive enhancement using htmx.

## Key Technologies

- **Rust** (Edition 2021) - Systems programming language
- **Axum 0.8** - Modern async web framework built on Tokio
- **axum-htmx 0.8** - HTMX integration with extractors and auto-vary headers
- **Maud 0.26.0** - Type-safe HTML templating
- **htmx 2.0.3** - Progressive enhancement library
- **Docker** - Containerization with multi-stage builds and scratch image

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

### Docker commands
```bash
# Build Docker image
docker build -t portfolio-rust .

# Run Docker container
docker run -p 3000:3000 portfolio-rust
```

## Architecture

### Core Dependencies
- **Axum 0.8**: Modern async web framework with macros and form support
- **axum-htmx 0.8**: HTMX extractors and middleware with auto-vary feature
- **Tokio 1.0**: Async runtime with full features
- **Tower 0.4**: Middleware and service abstractions
- **Tower-HTTP 0.5**: HTTP-specific middleware (static files, headers)
- **Maud 0.26.0**: Type-safe HTML templating
- **Chrono 0.4.38**: Date/time handling with serde support
- **Serde 1.0**: Serialization framework with derive support

### Application Structure
- **main.rs**: Entry point with route definitions, JSON-LD helpers, and server configuration
- **routes.rs**: Centralized route enum with path definitions
- **macros.rs**: DRY macro for generating htmx-aware page handlers
- **controllers/**: Request handlers using the page handler macro
  - `home.rs`: Homepage handler
  - `about.rs`: About section handler
  - `experience.rs`: Experience timeline handler
  - `projects.rs`: Projects showcase handler
  - `contact.rs`: Contact form with GET/POST handlers and validation
- **views/**: HTML template modules using Maud
  - `layout.rs`: Main layout with navigation and full page rendering
  - `home_view.rs`: Homepage content
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
- **Theme Support**: Light/dark mode with CSS and JavaScript
- **Type Safety**: Compile-time HTML validation with Maud
- **DRY Code**: Macro-based handler generation to reduce repetition

### Route Pattern
The application uses a macro to generate handlers that follow this pattern:
```rust
generate_page_handler!(handler_name, "Section Title", view_module::render);
```

This automatically creates handlers that:
1. Detect htmx requests using HxRequest extractor
2. Return partial content for htmx requests
3. Return full page for direct access
4. Handle the response conversion

### Static Assets Organization
- `static/css/`: Stylesheets (main.css, light.css, dark.css)
- `static/js/`: JavaScript files (htmx.min.js, main.js, theme-init.js)
- `static/data/`: JSON-LD structured data files

### Docker Deployment
The application includes a multi-stage Dockerfile that:
1. Builds a static binary using musl target
2. Creates a minimal scratch image with only the binary and static files
3. Exposes port 3000 for the web server

## Development Notes

### Adding New Sections
1. Create new view module in `src/views/`
2. Add module declaration to `src/views/mod.rs`
3. Create controller in `src/controllers/` using the generate_page_handler macro
4. Add module declaration to `src/controllers/mod.rs`
5. Add route variant to the Route enum in `src/routes.rs`
6. Add navigation link in `layout.rs`
7. Add route to the Router in main function

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
- **Production Optimization**: Release profile with LTO, single codegen unit, and stripping

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
- Content Security Policy (CSP) headers in HTML

### Performance Optimizations
- Aggressive caching for static assets (1 year immutable)
- CSS preload for critical styles
- Deferred JavaScript loading
- Optimized release builds with:
  - opt-level = 3
  - lto = true
  - codegen-units = 1
  - strip = true