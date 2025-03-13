use std::io::Error;

use tokio::process::Command;

pub async fn write_disk(disk_path: &str, source_path: &str, size: i16) -> Result<(), Error> {
    Command::new("qemu-img")
        .args([
            "create",
            "-b",
            source_path,
            "-f",
            "qcow2",
            "-F",
            "qcow2",
            disk_path,
            &format!("{size}G"),
        ])
        .output()
        .await?;
    Ok(())
}

#[cfg(test)]
mod qemu_tests {
    use super::write_disk;
    use std::fs;

    #[tokio::test]
    async fn test_write_disk() {
        let disk_path =
            "/var/lib/libvirt/images/vms/ab46b7a9-98b5-46cb-adde-a30399e23e72/disk.qcow2";
        let source_path = "/var/lib/libvirt/images/base/ubuntu22.img";
        let size = 10;

        write_disk(disk_path, source_path, size).await.unwrap();

        assert!(fs::exists(disk_path).unwrap());
    }
}
