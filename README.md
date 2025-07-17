# Basic Web - Modern Rust Portfolio

A professional portfolio website built with Rust, featuring the Axum web framework, Maud templating engine, and progressive enhancement with htmx. This project demonstrates modern web development practices with server-side rendering and type-safe templates.

## ✨ Features

- **Type-safe HTML templating** with Maud for compile-time template validation
- **Progressive enhancement** with htmx 2.0 for seamless SPA-like navigation
- **Dual rendering system**: Full pages for direct access, partial content for htmx requests
- **Development logging system**: Enhanced request tracking with colors and HTMX detection (debug mode only)
- **SNAPPY development workflow**: Ultra-fast cargo-watch with 100ms response time
- **Advanced theme management** with light/dark mode and system preference detection
- **Contact form** with comprehensive server-side validation and error handling
- **SEO optimization** with JSON-LD structured data and complete meta tags
- **Security hardened** with CSP headers and XSS protection
- **Production-ready Docker** with multi-stage builds and scratch-based images
- **Performance optimized** with aggressive caching and release profile tuning
- **Accessibility focused** with ARIA labels and keyboard navigation support
- **Modern JavaScript** with ES6+ features and modular architecture

## 🛠️ Tech Stack

### Core Stack
- **[Rust 2024](https://www.rust-lang.org/)** - Systems programming language with memory safety
- **[Axum 0.8](https://github.com/tokio-rs/axum)** - Modern async web framework with macros and form support
- **[axum-htmx 0.8](https://github.com/robertwayne/axum-htmx)** - HTMX integration with extractors and auto-vary headers
- **[Tokio 1.0](https://tokio.rs/)** - Async runtime with full feature set
- **[Maud 0.26.0](https://maud.lambda.xyz/)** - Type-safe HTML templating with compile-time validation
- **[Tower-HTTP 0.5](https://github.com/tower-rs/tower-http)** - HTTP middleware for static files and headers
- **[Just](https://just.systems/)** - Command runner for streamlined development workflow

### Frontend Technologies
- **[htmx 2.0.3](https://htmx.org/)** - Progressive enhancement library
- **Modern CSS** - Electric Eclipse theme with CSS custom properties
- **Vanilla JavaScript** - ES6+ with modular theme management and accessibility features
- **Web Standards** - Semantic HTML5 with ARIA accessibility

## 📁 Project Structure

```
basic-web/
├── src/
│   ├── main.rs              # Entry point and server configuration
│   ├── routes.rs            # Centralized routing system
│   ├── macros.rs            # Macros for handler generation
│   ├── controllers/         # Route controllers
│   │   ├── home.rs
│   │   ├── about.rs
│   │   ├── experience.rs
│   │   ├── projects.rs
│   │   └── contact.rs       # With form validation
│   └── views/               # HTML templates with Maud
│       ├── layout.rs        # Main layout
│       ├── home_view.rs
│       ├── about_view.rs
│       ├── experience_view.rs
│       ├── projects_view.rs
│       ├── contact_view.rs
│       └── error_view.rs
├── static/                  # Static assets
│   ├── css/electric-eclipse/ # Theme stylesheets (main, light, dark)
│   ├── js/                  # JavaScript (htmx.min.js, main.js, theme-init.js)
│   ├── data/                # JSON-LD structured data (person.json, website.json)
│   ├── *.png, *.ico         # Favicon and app icons
│   ├── robots.txt           # Search engine directives
│   ├── sitemap.xml          # Site structure for SEO
│   └── site.webmanifest     # Progressive Web App manifest
├── Cargo.toml               # Dependencies and configuration
├── justfile                 # Task runner commands
├── Dockerfile               # Multi-stage build
└── CLAUDE.md                # Documentation for Claude Code
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.80 or higher (for Rust 2024 edition support)
- Cargo (included with Rust)
- [just](https://just.systems/) command runner (optional but recommended)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd basic-web
```

2. **Option A: Using Just (Recommended)**
```bash
# Install just command runner if not already installed
cargo install just

# Start SNAPPY development server (100ms response time)
just dev

# Start with ultra-verbose debugging (50ms response)
just dev-verbose

# Check server status
just status

# Stop development server
just stop-dev

# Restart development server (if needed)
just restart-dev
```

3. **Option B: Using Cargo directly**
```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

4. Open your browser at `http://127.0.0.1:3000`

### Production Build

```bash
# Build with optimizations
cargo build --release

# Run the optimized version
cargo run --release
```

## 🔧 Development Tools

### Just Command Runner

This project includes a `justfile` for streamlined development workflow management:

```bash
# Show all available commands
just

# Start development server (cargo watch with auto-reload)
just dev

# Check development server status
just status

# View development server logs in real-time
just logs

# Stop development server safely
just stop-dev

# Restart development server (stop + start)
just restart-dev

# Build the project
just build

# Build for release
just build-release

# Run in release mode
just run

# Clean build artifacts and development files
just clean
```

#### Features of the Just Setup

- **Safe Process Management**: Uses PID files to track and safely terminate development servers
- **Background Execution**: Development server runs in background with logs redirected to `tmp/cargo_watch.log`
- **Status Monitoring**: Real-time status checking and log viewing
- **Automatic Cleanup**: Ensures no orphaned processes when stopping the server
- **Error Handling**: Robust error handling and process verification

#### Why Just?

- **Cross-platform**: Works on Windows, macOS, and Linux
- **Simple syntax**: Easy to read and maintain task definitions
- **No dependencies**: Single binary with no runtime requirements
- **Project-specific**: Commands are contextual to this project
- **Developer experience**: Simplifies common development tasks

### Manual Development

If you prefer not to use Just, you can still use standard Cargo commands:

```bash
# Development with auto-reload
cargo install cargo-watch
cargo watch -x run

# Build and run
cargo run
cargo run --release
```

## 🐳 Docker

### Build Image

```bash
docker build -t portfolio-rust .
```

### Run Container

```bash
docker run -p 3000:3000 portfolio-rust
```

The Docker image uses a multi-stage build that:
1. Compiles a static binary using musl target
2. Creates a minimal image from scratch
3. Includes only the binary and static files

## ⚙️ Configuration

### Runtime Configuration

- **Port**: Fixed at 3000 (configurable for development vs production binding)
- **Bind Address**: `127.0.0.1:3000` in debug mode, `0.0.0.0:3000` in release mode
- **Static Assets**: Served with 1-year immutable cache headers
- **Security Headers**: Automatic CSP, X-Frame-Options, and XSS protection

### Customization

1. **Personal Data**: Update JSON-LD files in `static/data/` (person.json, website.json)
2. **Content**: Edit view templates in `src/views/` (Spanish content supported)
3. **Styling**: Modify CSS in `static/css/electric-eclipse/` (main.css, light.css, dark.css)
4. **Features**: Add new sections following the established pattern

## 🏗️ Architecture

### Routing System

The project uses a centralized enum to manage routes:

```rust
// src/routes.rs
pub enum Route {
    Home,
    About,
    Experience,
    Projects,
    Contact,
}
```

### DRY Handler Generation & Development Logging

The project uses a custom macro system to eliminate code repetition:

```rust
// Automatically generates htmx-aware handlers with development logging
generate_page_handler!(handler, "Section Title", view_module);
```

This macro creates handlers that:
- Use `axum_htmx::HxRequest` extractor for type-safe htmx detection
- Return partial content for htmx requests (SPA-like navigation)
- Return complete pages for direct access (bookmarkable URLs)
- Handle response conversion automatically
- **Development logging**: Enhanced request tracking with timestamp, method, URI, request type (HTMX/FULL), source detection, and referer information (debug mode only)

### Dual Rendering

The application supports two rendering modes:
- **Direct access**: Returns complete HTML page
- **htmx navigation**: Returns only updated content

## 📝 Adding New Sections

1. Create view module in `src/views/new_section.rs`
2. Add declaration in `src/views/mod.rs`
3. Create controller in `src/controllers/new_section.rs`
4. Add declaration in `src/controllers/mod.rs`
5. Add variant to enum in `src/routes.rs`
6. Add navigation link in `src/views/layout.rs`
7. Register route in `main.rs`

## 🛡️ Security Features

### Built-in Security
- **Content Security Policy (CSP)** - Prevents XSS attacks
- **Security Headers** - X-Frame-Options, X-Content-Type-Options, XSS-Protection
- **Form Validation** - Server-side validation with error handling
- **Cache Management** - Automatic Vary headers for proper htmx caching
- **Input Sanitization** - Trim and validate all form inputs
- **No Secrets Exposure** - Template data clearly marked as placeholder

### HTTP Security Headers
- `X-Frame-Options: DENY`
- `X-Content-Type-Options: nosniff`
- `X-XSS-Protection: 1; mode=block`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `Permissions-Policy: geolocation=(), microphone=(), camera=()`

## ⚡ Performance Optimizations

### Frontend Performance
- **Static Asset Caching** - 1-year immutable cache headers
- **CSS Optimization** - Preloaded critical styles, theme-specific loading
- **JavaScript Optimization** - Deferred loading, modular architecture
- **Progressive Enhancement** - Core functionality works without JavaScript

### Backend Performance
- **Release Profile Tuning**:
  ```toml
  opt-level = 3          # Maximum optimization
  lto = true             # Link Time Optimization
  codegen-units = 1      # Single compilation unit
  strip = true           # Remove debug symbols
  ```
- **Async/Await** - Full async request handling with Tokio
- **Zero-copy** - Efficient string handling with Maud
- **Static Binary** - Minimal Docker images using scratch base

## 🤝 Contributing

Contributions are welcome. Please:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -am 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) for the excellent web framework
- [Maud](https://maud.lambda.xyz/) for type-safe templating
- [htmx](https://htmx.org/) for making the web interactive again