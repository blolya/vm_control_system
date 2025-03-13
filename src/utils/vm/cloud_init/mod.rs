use std::{fs, include_str, io::Error};
use tokio::process::Command;

pub async fn init_seed(
    vm_directory: &str,
    hostname: &str,
    username: &str,
    ssh_key: &str,
) -> Result<String, Error> {
    let meta_data_path = init_meta_data(vm_directory, hostname)?;
    let user_data_path = init_user_data(vm_directory, username, ssh_key)?;

    let seed_path = format!("{vm_directory}/seed.iso");
    write_seed(&seed_path, &user_data_path, &meta_data_path).await?;

    fs::remove_file(&meta_data_path)?;
    fs::remove_file(&user_data_path)?;

    Ok(seed_path)
}

async fn write_seed(
    seed_path: &str,
    user_data_path: &str,
    meta_data_path: &str,
) -> Result<(), Error> {
    Command::new("cloud-localds")
        .args([seed_path, user_data_path, meta_data_path])
        .output()
        .await?;

    Ok(())
}

fn create_user_data(username: &str, ssh_key: &str) -> String {
    format!(
        include_str!("user_data.yaml"),
        username = username,
        ssh_key = ssh_key
    )
}
fn init_user_data(vm_directory: &str, username: &str, ssh_key: &str) -> Result<String, Error> {
    let user_data_path = format!("{vm_directory}/user-data");
    let user_data = create_user_data(username, ssh_key);
    fs::write(&user_data_path, user_data)?;

    Ok(user_data_path)
}
fn create_meta_data(hostname: &str) -> String {
    format!(include_str!("meta_data.yaml"), hostname = hostname)
}

fn init_meta_data(vm_directory: &str, hostname: &str) -> Result<String, Error> {
    let meta_data_path = format!("{vm_directory}/meta-data");
    let meta_data = create_meta_data(hostname);
    fs::write(&meta_data_path, meta_data)?;

    Ok(meta_data_path)
}
