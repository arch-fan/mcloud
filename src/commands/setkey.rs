use dirs;
use tokio::fs;

pub async fn handler(key: &str) {
    let mut path = dirs::cache_dir().expect("Could not retrieve cache directory");

    path.push("mcloud");

    fs::create_dir_all(&path)
        .await
        .expect("Couldn't create cache dir");

    path.push("key");

    fs::write(&path, key)
        .await
        .expect("Couldn't write on cache dir");
}
