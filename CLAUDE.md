# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a professional portfolio website built with Rust, demonstrating modern web development practices. The application uses Axum 0.8 web framework, Maud 0.26.0 for type-safe HTML templating, and progressive enhancement with htmx 2.0.3 for seamless user experience.

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
- **axum-htmx 0.8**: HTMX integration with extractors and auto-vary headers for proper caching
- **Tokio 1.0**: Async runtime with full feature set (fs, net, time, macros)
- **Tower 0.4**: Service abstractions and middleware composition
- **Tower-HTTP 0.5**: HTTP-specific middleware (static file serving, response headers)
- **Maud 0.26.0**: Type-safe HTML templating with compile-time validation
- **Chrono 0.4.38**: Date/time handling with serde serialization support
- **Serde 1.0**: Serialization framework with derive macros for forms

### Application Structure
- **main.rs**: Application entry point with:
  - Route configuration and handler registration
  - JSON-LD helper functions for structured data
  - Security middleware setup (CSP, XSS protection, headers)
  - Static file serving with cache headers
  - Server binding configuration (dev vs production)
- **routes.rs**: Centralized Route enum with type-safe path definitions
- **macros.rs**: DRY macro system for generating htmx-aware page handlers
- **controllers/**: Request handlers leveraging the macro system:
  - `home.rs`: Homepage handler with dual rendering
  - `about.rs`: About section handler
  - `experience.rs`: Experience timeline handler  
  - `projects.rs`: Projects showcase handler
  - `contact.rs`: Contact form with comprehensive validation (GET/POST)
- **views/**: Maud template modules with type-safe HTML:
  - `layout.rs`: Main layout with navigation, meta tags, and security headers
  - `home_view.rs`: Homepage content and hero section
  - `about_view.rs`: About section template
  - `experience_view.rs`: Experience timeline template
  - `projects_view.rs`: Projects showcase template
  - `contact_view.rs`: Contact form with validation states
  - `error_view.rs`: Error pages (404 fallback)

### Key Features
- **Progressive Enhancement**: htmx 2.0.3 integration with axum-htmx extractors for SPA-like navigation
- **Auto-Vary Headers**: AutoVaryLayer middleware ensures proper cache behavior with htmx
- **Dual Rendering System**: 
  - Full page rendering for direct URL access (bookmarkable)
  - Partial content for htmx requests (seamless navigation)
- **Advanced Form Handling**: Contact form with comprehensive server-side validation
- **Performance Optimization**: Static assets with 1-year immutable cache headers
- **SEO & Structured Data**: JSON-LD from `static/data/` with API endpoints
- **Security Hardening**: 
  - CSP headers via meta tags and HTTP headers
  - XSS protection, frame options, content type validation
  - Input sanitization and form validation
- **Theme Management**: Advanced light/dark mode with system preference detection
- **Type Safety**: Compile-time HTML validation and route safety with Rust type system
- **Architecture**: DRY macro system eliminates handler boilerplate

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
- `static/css/electric-eclipse/`: Electric Eclipse theme system
  - `main.css`: Core styles and CSS custom properties
  - `light.css`: Light theme variables and overrides
  - `dark.css`: Dark theme variables and overrides
- `static/js/`: Modern JavaScript modules
  - `htmx.min.js`: HTMX 2.0.3 library for progressive enhancement
  - `main.js`: Theme management, accessibility features, and UX enhancements
  - `theme-init.js`: Theme initialization to prevent flash of unstyled content
- `static/data/`: JSON-LD structured data
  - `person.json`: Personal/professional information (Schema.org Person)
  - `website.json`: Website metadata (Schema.org WebSite)
- `static/`: Additional assets
  - Favicon files (ico, png) and app icons
  - `robots.txt`, `sitemap.xml` for SEO
  - `site.webmanifest` for PWA capabilities

### Docker Deployment
Production-ready containerization with multi-stage build:
1. **Builder stage**: Compiles static binary using `x86_64-unknown-linux-musl` target
2. **Runtime stage**: Minimal scratch-based image containing:
   - Static binary (`/basic-web`)
   - Static assets (`/static`)
   - No unnecessary dependencies or OS layers
3. **Configuration**: Exposes port 3000, optimized for container environments

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
Layered middleware architecture for security and performance:
```rust
// Security headers middleware
.layer(SetResponseHeaderLayer::overriding("x-frame-options", "DENY"))
.layer(SetResponseHeaderLayer::overriding("x-content-type-options", "nosniff"))
.layer(SetResponseHeaderLayer::overriding("x-xss-protection", "1; mode=block"))
.layer(SetResponseHeaderLayer::overriding("referrer-policy", "strict-origin-when-cross-origin"))
.layer(SetResponseHeaderLayer::overriding("permissions-policy", "geolocation=(), microphone=(), camera=()"))
// HTMX cache management
.layer(AutoVaryLayer)
```

### Security Considerations
- Input validation on contact form
- Static asset cache headers for performance
- Automatic Vary header management prevents caching issues with htmx
- Type-safe htmx header extraction
- Content Security Policy (CSP) headers in HTML

### Performance Optimizations
**Frontend Performance:**
- Static assets: 1-year immutable cache headers (`public, max-age=31536000, immutable`)
- CSS: Preloaded critical styles, theme-specific loading
- JavaScript: Deferred loading, modular ES6+ architecture
- Progressive enhancement: Core functionality without JavaScript

**Backend Performance:**
- Release profile optimization:
  ```toml
  opt-level = 3        # Maximum optimization level
  lto = true          # Link Time Optimization
  codegen-units = 1   # Single compilation unit for better optimization
  strip = true        # Remove debug symbols for smaller binary
  ```
- Async/await throughout with Tokio runtime
- Zero-copy string handling with Maud templates
- Efficient static file serving with Tower-HTTP