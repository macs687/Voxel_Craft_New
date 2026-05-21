use core::error;

use thiserror::Error;


#[derive(Error, Debug)]
pub enum ProjectErrors {
    #[error("Ошибка инициализации GLFW: {0}")]
    GlfwInitError(String),
    
    #[error("Ошибка создания окна: {0}")]
    WindowCreateError(String),

    #[error("Ошибка ввода-вывода: {0}")]
    Io(#[from] std::io::Error),

    #[error("Ошибка открытия файла '{path}': {source}")] 
    FileOpen {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Ошибка чтения файла '{path}': {source}")]
    FileRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Ошибка преобразования в CString для {context}: {source}")]
    CStringConvert {
        context: String,
        #[source]
        source: std::ffi::NulError,
    },

    #[error("Ошибка компиляции {stage} шейдера:\n{log}")]
    ShaderCompilation {
        stage: &'static str,   // "Vertex" или "Fragment"
        log: String,
    },

    #[error("Ошибка линковки шейдерной программы:\n{log}")]
    ShaderLinking {
        log: String,
    },

    #[error("Ошибка загрузки текстуры '{path}': {source}")]
    TextureLoadError {
        path: String,
        #[source]
        source: image::ImageError,
    },

    #[error("Ошибка создания изображения из RgbaImage: {log}")]
    TextureCreationError {
        //#[source]
        log: String
    }
}


impl ProjectErrors {
    
}