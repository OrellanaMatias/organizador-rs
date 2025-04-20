# Organizador-rs

Una aplicación de consola en Rust para organizar archivos automáticamente por tipo.

## Funcionalidades

- Organiza los archivos de una carpeta en subcarpetas según su tipo (extensión)
- Categoriza archivos en: Imagenes, Videos, Documentos y Otros
- Ignora archivos ocultos
- Evita sobrescribir archivos con el mismo nombre
- Informa por consola sobre cada archivo movido

## Categorías de archivos

- **Imagenes**: .jpg, .jpeg, .png, .gif
- **Videos**: .mp4, .avi, .mkv
- **Documentos**: .pdf, .docx, .txt
- **Otros**: Cualquier otro tipo de archivo

## Instalación

### Desde releases

Descarga el archivo ejecutable desde la sección de [releases](https://github.com/OrellanaMatias/organizador-rs/releases).

### Compilar desde el código fuente

1. Clona el repositorio:
```
git clone https://github.com/OrellanaMatias/organizador-rs.git
cd organizador-rs
```

2. Compila el proyecto:
```
cargo build --release
```

3. El ejecutable estará disponible en `target/release/organizador-rs.exe` (Windows) o `target/release/organizador-rs` (Linux/Mac).

## Uso

Para organizar la carpeta actual:
```
organizador-rs
```

Para organizar una carpeta específica:
```
organizador-rs C:\ruta\a\tu\carpeta
```

Ejemplo:
```
organizador-rs C:\Users\TuUsuario\Downloads
```
