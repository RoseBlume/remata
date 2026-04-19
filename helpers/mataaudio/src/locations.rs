use std::sync::LazyLock;

#[cfg(target_os = "windows")]
pub static USERNAME: LazyLock<String> = LazyLock::new(|| {
    std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
});

#[cfg(not(target_os = "windows"))]
pub static USERNAME: LazyLock<String> = LazyLock::new(|| {
    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
});

pub static MUSIC_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Music".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Music", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Music", *USERNAME)
    }
});

pub static DOCUMENTS_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Documents".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Documents", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Documents", *USERNAME)
    }
});

pub static DOWNLOADS_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Downloads".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Downloads", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Downloads", *USERNAME)
    }
});

pub static PICTURES_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Pictures".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Pictures", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Pictures", *USERNAME)
    }
});

pub static VIDEOS_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Movies".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Videos", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Videos", *USERNAME)
    }
});
