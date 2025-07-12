# Rust Portfolio Website

A modern and professional web portfolio built with Rust, using the Axum framework, Maud templating, and progressive enhancement with htmx.

## 🚀 Features

- **Server-side rendering (SSR)** with type-safe templates using Maud
- **Progressive enhancement** with htmx for SPA-like navigation without complex JavaScript
- **Modern architecture** with async/await, Axum extractors, and Tower middleware
- **Theme system** with light/dark mode support
- **Contact form** with server-side validation
- **SEO optimized** with JSON-LD structured data
- **Performance optimized** with aggressive caching and optimized compilation
- **Containerization** with Docker using minimal images (scratch)
- **DRY code** through macro system to reduce repetition

## 🛠️ Tech Stack

### Backend
- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Axum 0.8](https://github.com/tokio-rs/axum)** - Modern async web framework
- **[Tokio](https://tokio.rs/)** - Asynchronous runtime
- **[Maud](https://maud.lambda.xyz/)** - Type-safe HTML templating
- **[Tower](https://github.com/tower-rs/tower)** - Middleware and service abstractions

### Frontend
- **[htmx 2.0](https://htmx.org/)** - Interactivity without complexity
- **CSS3** - Modern styles with CSS variables
- **HTML5** - Semantic structure

## 📁 Project Structure

```
prueba/
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
├── static/                  # Static files
│   ├── css/                 # Stylesheets
│   ├── js/                  # JavaScript (htmx, theme)
│   └── data/                # JSON-LD data
├── Cargo.toml               # Dependencies and configuration
├── Dockerfile               # Multi-stage build
└── CLAUDE.md                # Documentation for Claude Code
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or higher
- Cargo (included with Rust)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd prueba
```

2. Run the application:
```bash
cargo run
```

3. Open your browser at `http://127.0.0.1:3000`

### Development with Auto-Reload

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

### Production Build

```bash
# Build with optimizations
cargo build --release

# Run the optimized version
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

## 🔧 Configuration

### Environment Variables

- `RUST_LOG`: Logging level (e.g., `debug`, `info`)
- Port is fixed at 3000

### Customization

1. **Personal information**: Edit templates in `src/views/`
2. **Styles**: Modify CSS files in `static/css/electric-eclipse/`
3. **Structured data**: Update JSON-LD files in `static/data/`
4. **New sections**: See "Adding New Sections"

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

### Handler Generation

Handlers are automatically generated with a macro that:
- Detects htmx requests
- Renders partial content or full page as appropriate
- Manages response conversion

```rust
generate_page_handler!(home, "Home", home_view::render);
```

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

## 🔒 Security

- Input validation on forms
- Configured CSP headers
- Automatic Vary header management for caching
- No sensitive information exposure

## ⚡ Performance Optimizations

- 1-year immutable cache for static files
- Critical CSS preload
- Deferred JavaScript
- Compilation with:
  - Link Time Optimization (LTO)
  - Single codegen unit
  - Symbol stripping

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