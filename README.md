# Portfolio Profesional - Desarrollador Full Stack

Una página web de portfolio profesional moderna construida con **Rust**, **Rocket**, **Maud** y **htmx** que demuestra habilidades de desarrollo web moderno con renderizado del lado del servidor y mejora progresiva.

## 🚀 Características

- **Framework Rocket**: Framework web rápido y seguro para Rust
- **Templates Maud**: Sistema de plantillas HTML type-safe
- **Integración htmx**: Navegación suave y carga de contenido dinámico sin JavaScript complejo
- **Diseño Responsivo**: Optimizado para desktop, tablet y móvil
- **Manejo de Errores**: Páginas de error personalizadas (404, 500)
- **Formulario de Contacto**: Validación del lado del servidor con retroalimentación
- **CSS Modular**: Sistema de estilos organizado y mantenible
- **Headers de Seguridad**: Mejoras básicas de seguridad
- **Animaciones Suaves**: Transiciones y efectos visuales modernos

## 📋 Secciones del Portfolio

- **🏠 Inicio**: Presentación principal con llamada a la acción
- **👨‍💻 Sobre Mí**: Información personal y habilidades técnicas
- **💼 Experiencia**: Timeline de experiencia profesional y educación
- **🎯 Proyectos**: Showcase de proyectos destacados con tecnologías utilizadas
- **📞 Contacto**: Formulario funcional y información de contacto

## 🛠️ Tecnologías Utilizadas

### Backend
- **Rust** - Lenguaje de programación principal
- **Rocket 0.5.1** - Framework web
- **Maud** - Motor de plantillas HTML

### Frontend
- **htmx 2.0.3** - Interactividad sin JavaScript complejo
- **CSS3** - Estilos modernos con gradientes y animaciones
- **HTML5** - Estructura semántica

### Herramientas de Desarrollo
- **cargo-watch** - Auto-reload durante desarrollo
- **Cargo** - Gestor de paquetes y build tool

## 🚀 Instalación y Uso

### Requisitos Previos

- Rust 1.70+ (con Cargo)
- Un navegador web moderno

### Ejecutar la Aplicación

1. **Ejecutar básico**:
   ```sh
   cargo run
   ```
   El servidor iniciará en `http://127.0.0.1:3000`

2. **Desarrollo con auto-reload**:
   ```sh
   cargo install cargo-watch
   cargo watch -x run
   ```
   Recompila y reinicia automáticamente cuando los archivos cambian.

### Rutas Disponibles

- `GET /` - Página principal
- `GET /about` - Sección sobre mí
- `GET /experience` - Experiencia profesional
- `GET /projects` - Proyectos destacados
- `GET /contact` - Información de contacto
- `POST /contact` - Envío de formulario de contacto
- Páginas de error personalizadas para códigos 404 y 500

## 📁 Estructura del Proyecto

```
src/
├── main.rs              # Punto de entrada y definición de rutas
├── views/               # Módulos de plantillas HTML
│   ├── mod.rs
│   ├── layout.rs        # Layout principal con navegación
│   ├── about_view.rs    # Sección sobre mí
│   ├── experience_view.rs # Timeline de experiencia
│   ├── projects_view.rs # Showcase de proyectos
│   ├── contact_view.rs  # Formulario de contacto
│   └── error_view.rs    # Páginas de error
└── styles/              # Módulos CSS
    ├── mod.rs
    └── main.rs          # Estilos principales
```

## 🎨 Personalización

### Cambiar Información Personal

1. **Información básica**: Edita `src/views/layout.rs` para cambiar nombre y título
2. **Sobre mí**: Modifica `src/views/about_view.rs` para actualizar biografía y habilidades
3. **Experiencia**: Actualiza `src/views/experience_view.rs` con tu historial profesional
4. **Proyectos**: Personaliza `src/views/projects_view.rs` con tus proyectos
5. **Contacto**: Cambia información en `src/views/contact_view.rs`

### Modificar Estilos

- **Colores principales**: Edita las variables de color en `src/styles/main.rs`
- **Fuentes**: Cambia la familia de fuentes en la sección `body`
- **Animaciones**: Ajusta las transiciones y keyframes según preferencias

### Agregar Nuevas Secciones

1. Crea un nuevo módulo de vista en `src/views/`
2. Define la ruta en `src/main.rs`
3. Agrega navegación en `src/views/layout.rs`
4. Actualiza `src/views/mod.rs`

## 🔧 Funcionalidades Técnicas Destacadas

### Navegación SPA con htmx
- Carga de contenido dinámico sin recargas de página
- Preservación del historial del navegador
- Indicadores de carga suaves

### Formulario de Contacto
- Validación del lado del servidor
- Mensajes de error y éxito dinámicos
- Prevención de ataques comunes

### Diseño Responsivo
- Grid CSS para layouts adaptativos
- Breakpoints para móvil, tablet y desktop
- Navegación optimizada para pantallas pequeñas

### Rendimiento
- CSS inline para evitar requests adicionales
- Lazy loading de secciones
- Optimización de imágenes y recursos

## 🚀 Despliegue

### Desarrollo Local
```sh
cargo run --release
```

### Docker (Opcional)
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/prueba .
EXPOSE 3000
CMD ["./prueba"]
```

### Consideraciones de Producción

- Configurar reverse proxy (nginx/apache)
- Habilitar HTTPS
- Configurar variables de entorno
- Implementar logging y monitoreo
- Configurar rate limiting

## 🤝 Contribuciones

Este es un proyecto de portfolio personal, pero si encuentras bugs o tienes sugerencias de mejora:

1. Crea un issue describiendo el problema/mejora
2. Fork el repositorio
3. Crea una rama para tu feature
4. Envía un pull request

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Ver el archivo `LICENSE` para más detalles.

## 📞 Contacto

- **Email**: tu.email@ejemplo.com
- **LinkedIn**: [linkedin.com/in/tu-perfil](https://linkedin.com/in/tu-perfil)
- **GitHub**: [github.com/tu-usuario](https://github.com/tu-usuario)

---

**Nota**: Este portfolio demuestra el uso de tecnologías web modernas del ecosistema Rust para crear aplicaciones web rápidas, seguras y maintibles. Ideal para desarrolladores que buscan alternativas a frameworks JavaScript tradicionales.