# Portfolio Personal en Rust

Un portfolio web profesional y moderno construido con Rust, utilizando el framework Axum, templating Maud y mejora progresiva con htmx.

## 🚀 Características

- **Renderizado del lado del servidor (SSR)** con templates type-safe usando Maud
- **Mejora progresiva** con htmx para navegación SPA sin JavaScript complejo
- **Arquitectura moderna** con async/await, extractores de Axum y middleware de Tower
- **Sistema de temas** con soporte para modo claro/oscuro
- **Formulario de contacto** con validación del lado del servidor
- **SEO optimizado** con datos estructurados JSON-LD
- **Rendimiento optimizado** con caché agresivo y compilación con optimizaciones
- **Containerización** con Docker usando imágenes mínimas (scratch)
- **Código DRY** mediante sistema de macros para reducir repetición

## 🛠️ Stack Tecnológico

### Backend
- **[Rust](https://www.rust-lang.org/)** - Lenguaje de programación de sistemas
- **[Axum 0.8](https://github.com/tokio-rs/axum)** - Framework web async moderno
- **[Tokio](https://tokio.rs/)** - Runtime asíncrono
- **[Maud](https://maud.lambda.xyz/)** - Templating HTML type-safe
- **[Tower](https://github.com/tower-rs/tower)** - Middleware y abstracciones de servicios

### Frontend
- **[htmx 2.0](https://htmx.org/)** - Interactividad sin complejidad
- **CSS3** - Estilos modernos con variables CSS
- **HTML5** - Estructura semántica

## 📁 Estructura del Proyecto

```
prueba/
├── src/
│   ├── main.rs              # Punto de entrada y configuración del servidor
│   ├── routes.rs            # Sistema centralizado de rutas
│   ├── macros.rs            # Macros para generación de handlers
│   ├── controllers/         # Controladores de rutas
│   │   ├── home.rs
│   │   ├── about.rs
│   │   ├── experience.rs
│   │   ├── projects.rs
│   │   └── contact.rs       # Con validación de formularios
│   └── views/               # Templates HTML con Maud
│       ├── layout.rs        # Layout principal
│       ├── home_view.rs
│       ├── about_view.rs
│       ├── experience_view.rs
│       ├── projects_view.rs
│       ├── contact_view.rs
│       └── error_view.rs
├── static/                  # Archivos estáticos
│   ├── css/                 # Hojas de estilo
│   ├── js/                  # JavaScript (htmx, theme)
│   └── data/                # Datos JSON-LD
├── Cargo.toml               # Dependencias y configuración
├── Dockerfile               # Multi-stage build
└── CLAUDE.md                # Documentación para Claude Code
```

## 🚀 Inicio Rápido

### Requisitos Previos

- Rust 1.70 o superior
- Cargo (incluido con Rust)

### Instalación

1. Clona el repositorio:
```bash
git clone <url-del-repositorio>
cd prueba
```

2. Ejecuta la aplicación:
```bash
cargo run
```

3. Abre tu navegador en `http://127.0.0.1:3000`

### Desarrollo con Auto-Recarga

```bash
# Instala cargo-watch
cargo install cargo-watch

# Ejecuta con auto-recarga
cargo watch -x run
```

### Compilación para Producción

```bash
# Compila con optimizaciones
cargo build --release

# Ejecuta la versión optimizada
cargo run --release
```

## 🐳 Docker

### Construcción de la Imagen

```bash
docker build -t portfolio-rust .
```

### Ejecución del Contenedor

```bash
docker run -p 3000:3000 portfolio-rust
```

La imagen Docker utiliza una construcción multi-stage que:
1. Compila un binario estático con musl
2. Crea una imagen mínima desde scratch
3. Incluye solo el binario y los archivos estáticos

## 🔧 Configuración

### Variables de Entorno

- `RUST_LOG`: Nivel de logging (ej: `debug`, `info`)
- El puerto está fijo en 3000

### Personalización

1. **Información personal**: Edita los templates en `src/views/`
2. **Estilos**: Modifica los archivos CSS en `static/css/`
3. **Datos estructurados**: Actualiza los archivos JSON-LD en `static/data/`
4. **Nuevas secciones**: Ver sección "Añadir Nueva Sección"

## 🏗️ Arquitectura

### Sistema de Rutas

El proyecto utiliza un enum centralizado para gestionar las rutas:

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

### Generación de Handlers

Los handlers se generan automáticamente con una macro que:
- Detecta peticiones htmx
- Renderiza contenido parcial o página completa según corresponda
- Gestiona la conversión de respuestas

```rust
generate_page_handler!(home, "Inicio", home_view::render);
```

### Renderizado Dual

La aplicación soporta dos modos de renderizado:
- **Acceso directo**: Devuelve página HTML completa
- **Navegación htmx**: Devuelve solo el contenido actualizado

## 📝 Añadir Nueva Sección

1. Crea el módulo de vista en `src/views/nueva_seccion.rs`
2. Añade la declaración en `src/views/mod.rs`
3. Crea el controlador en `src/controllers/nueva_seccion.rs`
4. Añade la declaración en `src/controllers/mod.rs`
5. Añade la variante al enum en `src/routes.rs`
6. Añade el enlace de navegación en `src/views/layout.rs`
7. Registra la ruta en `main.rs`

## 🔒 Seguridad

- Validación de entrada en formularios
- Headers CSP configurados
- Gestión automática de headers Vary para caché
- Sin exposición de información sensible

## ⚡ Optimizaciones de Rendimiento

- Caché inmutable de 1 año para archivos estáticos
- Preload de CSS crítico
- JavaScript diferido
- Compilación con:
  - Link Time Optimization (LTO)
  - Unidad única de generación de código
  - Stripping de símbolos

## 🤝 Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/nueva-caracteristica`)
3. Commit tus cambios (`git commit -am 'Añade nueva característica'`)
4. Push a la rama (`git push origin feature/nueva-caracteristica`)
5. Abre un Pull Request

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## 🙏 Agradecimientos

- [Axum](https://github.com/tokio-rs/axum) por el excelente framework web
- [Maud](https://maud.lambda.xyz/) por el templating type-safe
- [htmx](https://htmx.org/) por hacer la web interactiva simple de nuevo