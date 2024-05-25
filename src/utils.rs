use tokio::fs;

pub async fn get_key() -> String {
    let mut path = dirs::cache_dir().expect("Could not retrieve cache directory");

    path.push("mcloud/key");

    fs::read_to_string(path)
        .await
        .expect("Couldn't read key file.")
}
