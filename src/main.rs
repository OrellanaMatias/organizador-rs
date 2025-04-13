use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let carpeta = if args.len() < 2 {
        match env::current_dir() {
            Ok(path) => path,
            Err(_) => {
                println!("Error: No se pudo obtener el directorio actual");
                return Ok(());
            }
        }
    } else {
        Path::new(&args[1]).to_path_buf()
    };
    
    if !carpeta.exists() || !carpeta.is_dir() {
        println!("Error: La ruta proporcionada no existe o no es un directorio");
        return Ok(());
    }
    
    println!("Organizando archivos en: {}", carpeta.display());
    
    let mut tipos_archivo: HashMap<&str, &str> = HashMap::new();
    tipos_archivo.insert("jpg", "Imagenes");
    tipos_archivo.insert("jpeg", "Imagenes");
    tipos_archivo.insert("png", "Imagenes");
    tipos_archivo.insert("gif", "Imagenes");
    tipos_archivo.insert("mp4", "Videos");
    tipos_archivo.insert("avi", "Videos");
    tipos_archivo.insert("mkv", "Videos");
    tipos_archivo.insert("pdf", "Documentos");
    tipos_archivo.insert("docx", "Documentos");
    tipos_archivo.insert("txt", "Documentos");
    
    let mut archivos_movidos = 0;
    
    for entrada in WalkDir::new(&carpeta)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entrada.path();
        
        if es_archivo_oculto(path) {
            continue;
        }
        
        if path.is_file() {
            if let Ok(()) = mover_archivo(path, &carpeta, &tipos_archivo) {
                archivos_movidos += 1;
            }
        }
    }
    
    println!("¡Completado! Se movieron {} archivos.", archivos_movidos);
    println!("Presiona Enter para salir...");
    let mut entrada = String::new();
    let _ = io::stdin().read_line(&mut entrada);
    
    Ok(())
}

fn es_archivo_oculto(path: &Path) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if let Ok(metadata) = fs::metadata(path) {
            return metadata.file_attributes() & 2 == 2;
        }
    }
    
    #[cfg(not(windows))]
    {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with("."))
            .unwrap_or(false)
    }
    
    false
}

fn obtener_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

fn mover_archivo(path: &Path, carpeta_base: &Path, tipos_archivo: &HashMap<&str, &str>) -> io::Result<()> {
    let extension = obtener_extension(path);
    
    let subcarpeta = match &extension {
        Some(ext) => tipos_archivo.get(ext.as_str()).unwrap_or(&"Otros"),
        None => "Otros",
    };
    
    let subcarpeta_path = carpeta_base.join(subcarpeta);
    
    if !subcarpeta_path.exists() {
        fs::create_dir(&subcarpeta_path)?;
        println!("Creada carpeta: {}", subcarpeta_path.display());
    }
    
    let nombre_archivo = path.file_name().unwrap();
    let mut destino = subcarpeta_path.join(nombre_archivo);
    
    let mut contador = 1;
    while destino.exists() {
        let nuevo_nombre = format!(
            "{}_{}.{}",
            path.file_stem().unwrap().to_string_lossy(),
            contador,
            extension.as_deref().unwrap_or("")
        );
        destino = subcarpeta_path.join(nuevo_nombre);
        contador += 1;
    }
    
    fs::rename(path, &destino)?;
    println!("Movido: {} → {}", path.display(), destino.display());
    
    Ok(())
}
