pub mod cloud_init;
pub mod domain;
pub mod qemu;

use std::{env, fs};

use uuid::Uuid;
use virt::{connect::Connect, domain::Domain};

use crate::{repositories::domain_repository::DomainRepository, views::domain_view::DomainView};

pub async fn init(
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

    let seed_path = cloud_init::init_seed(vm_directory, hostname, username, ssh_key)
        .await
        .map_err(|e| e.to_string())?;

    let disk_path = format!("{vm_directory}/disk.qcow2");
    qemu::write_disk(&disk_path, os_image_path, disk_size)
        .await
        .map_err(|e| e.to_string())?;

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

pub async fn list(
    domain_repository: &DomainRepository,
    hypervisor_connection: &Connect,
) -> Result<Vec<DomainView>, String> {
    let domains = domain_repository.list().await.map_err(|e| e.to_string())?;
    let domains = domains
        .into_iter()
        .map(|d| {
            (
                domain::find_by_id(&hypervisor_connection, *d.id()).unwrap(),
                d,
            )
        })
        .map(DomainView::from)
        .collect();
    Ok(domains)
}

pub async fn destroy(
    id: Uuid,
    domain_repository: &DomainRepository,
    hypervisor_connection: &Connect,
) -> Result<(), String> {
    let domain = domain::find_by_id(&hypervisor_connection, id).map_err(|e| e.to_string())?;
    if domain.is_active().map_err(|e| e.to_string())? {
        domain::destroy(&hypervisor_connection, id).map_err(|e| e.to_string())?;
    }
    domain::undefine(&hypervisor_connection, id).map_err(|e| e.to_string())?;

    let base_dir = env::var("BASE_PATH").unwrap();
    let vm_directory = format!("{base_dir}/vms/{id}");
    fs::remove_dir_all(&vm_directory).map_err(|e| e.to_string())?;

    domain_repository
        .delete(id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn pause(id: Uuid, hypervisor_connection: &Connect) -> Result<(), String> {
    domain::suspend(&hypervisor_connection, id).map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn reboot(id: Uuid, hypervisor_connection: &Connect) -> Result<(), String> {
    domain::reboot(&hypervisor_connection, id).map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn resume(id: Uuid, hypervisor_connection: &Connect) -> Result<(), String> {
    domain::resume(&hypervisor_connection, id).map_err(|e| e.to_string())?;
    Ok(())
}
