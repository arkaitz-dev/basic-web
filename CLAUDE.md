# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a professional portfolio website built with Rust, Rocket web framework, and Maud templating. It demonstrates modern web development with server-side rendering and progressive enhancement using htmx.

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
- **Rocket 0.5.1**: Web framework with JSON support
- **Maud 0.26.0**: Type-safe HTML templating with Rocket integration
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
- **htmx Integration**: Progressive enhancement with `HtmxRequest` struct for detecting htmx requests
- **Dual Rendering**: Full page rendering for direct access, partial content for htmx requests
- **Form Handling**: Contact form with server-side validation
- **Static Assets**: Served from `/static` with cache headers via `StaticAssetCacheFairing`
- **JSON-LD**: Structured data from `static/data/` files
- **Security**: Rocket Shield middleware with CSP headers
- **Error Handling**: Custom 404 and 500 error pages

### Route Pattern
Each route handler follows this pattern:
```rust
fn handler(htmx: HtmxRequest) -> Result<Markup, Status> {
    if htmx.0 {
        // Return only content for htmx requests
        Ok(views::module::render())
    } else {
        // Return full page for direct access
        Ok(views::layout::render_page_with_content("section", content))
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
3. Create route handler in `main.rs` following the htmx pattern
4. Add navigation link in `layout.rs`
5. Mount route in the rocket configuration

### Customization
- Update personal information in view templates
- Modify JSON-LD files in `static/data/`
- Adjust styling in CSS files
- The application uses Spanish language by default

### Security Considerations
- CSP headers configured in layout templates
- Rocket Shield middleware for security headers
- Static asset cache headers for performance
- Input validation on contact form