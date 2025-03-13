pub mod cloud_init;
pub mod domain;
pub mod qemu;

use std::{fs, io::Error};
use tracing::debug;
use uuid::Uuid;
use virt::{connect::Connect, domain::Domain, sys};

pub async fn init_vm(
    connection: &Connect,
    vm_directory: &str,
    os_image_path: &str,
    id: &Uuid,
    name: &Uuid,
    vcpu_count: i16,
    memory_size: i16,
    disk_size: i16,
    hostname: &str,
    username: &str,
    ssh_key: &str,
) -> Result<Domain, String> {
    fs::create_dir(vm_directory).map_err(|e| e.to_string())?;

    let seed_path = cloud_init::init_seed(vm_directory, hostname, username, ssh_key).await.map_err(|e| e.to_string())?;

    let disk_path = format!("{vm_directory}/disk.qcow2");
    qemu::write_disk(&disk_path, os_image_path, disk_size).await.map_err(|e| e.to_string())?;

    let domain_xml = domain::create_xml(
        id,
        &name,
        vcpu_count,
        memory_size,
        &disk_path,
        &os_image_path,
        &seed_path,
    );

    let domain = Domain::define_xml(connection, &domain_xml).map_err(|e| e.to_string())?;
    domain.create().map_err(|e| e.to_string())?;

    Ok(domain)
}

pub fn destroy_vm(connection: &Connect, domain_id: Uuid) -> Result<(), virt::error::Error> {
    debug!("destroying vm {}", domain_id);
    let domain = Domain::lookup_by_uuid(connection, domain_id)?;
    domain.destroy()?;

    Ok(())
}
pub fn shutdown_vm(connection: &Connect, domain_id: Uuid) -> Result<(), virt::error::Error> {
    let domain = Domain::lookup_by_uuid(connection, domain_id)?;
    domain.shutdown()?;

    Ok(())
}
pub fn undefine_vm(connection: &Connect, domain_id: Uuid) -> Result<(), virt::error::Error> {
    let domain = Domain::lookup_by_uuid(connection, domain_id)?;
    domain.undefine()?;

    Ok(())
}
pub fn suspend_vm(connection: &Connect, domain_id: Uuid) -> Result<(), virt::error::Error> {
    let domain = Domain::lookup_by_uuid(connection, domain_id)?;
    domain.suspend()?;

    Ok(())
}
pub fn resume_vm(connection: &Connect, domain_id: Uuid) -> Result<(), virt::error::Error> {
    let domain = Domain::lookup_by_uuid(connection, domain_id)?;
    domain.resume()?;

    Ok(())
}
pub fn reboot_vm(connection: &Connect, domain_id: Uuid) -> Result<(), virt::error::Error> {
    let domain = Domain::lookup_by_uuid(connection, domain_id)?;
    domain.reboot(sys::VIR_DOMAIN_REBOOT_DEFAULT)?;

    Ok(())
}
