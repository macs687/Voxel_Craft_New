use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MenuConfig {
    /// Путь к файлу фоновой текстуры (относительно корня проекта)
    pub background_texture: String,
    /// Скорость скроллинга текстуры по горизонтали
    pub scroll_speed_x: f32,
    /// Скорость скроллинга текстуры по вертикали
    pub scroll_speed_y: f32,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            background_texture: "res/textures/fone.png".to_string(),
            scroll_speed_x: 0.02,
            scroll_speed_y: 0.01,
        }
    }
}

impl MenuConfig {
    /// Загружает конфиг из файла. Если файла нет, создаёт его со значениями по умолчанию.
    pub fn load(path: &str) -> Self {
        let path = std::path::Path::new(path);
        if path.exists() {
            let data = std::fs::read_to_string(path).expect("Failed to read menu config");
            toml::from_str(&data).unwrap_or_default()
        } else {
            let config = MenuConfig::default();
            config.save(path);
            config
        }
    }

    /// Сохраняет конфиг в файл.
    pub fn save(&self, path: &std::path::Path) {
        let data = toml::to_string_pretty(self).expect("Failed to serialize menu config");
        std::fs::write(path, data).expect("Failed to write menu config");
    }
}