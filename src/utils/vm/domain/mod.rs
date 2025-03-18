use std::include_str;

use uuid::Uuid;
use virt::{connect::Connect, domain::Domain, error::Error, sys};

pub fn create_xml(
    id: &Uuid,
    name: &Uuid,
    vcpu_count: i16,
    memory_size: i16,
    disk_path: &str,
    image_path: &str,
    seed_path: &str,
) -> String {
    format!(
        include_str!("domain.xml"),
        uuid = id,
        name = name,
        vcpu_count = vcpu_count,
        memory_size = memory_size,
        disk_path = disk_path,
        image_path = image_path,
        seed_path = seed_path
    )
}

pub fn get_ip_address(domain: &Domain) -> Result<Option<String>, Error> {
    if let Some(interface) = domain.interface_addresses(0, 0)?.iter().next() {
        if let Some(address) = interface.addrs.iter().next() {
            return Ok(Some(address.addr.to_string()));
        }
    }
    Ok(None)
}

pub fn list(connection: &Connect) -> Result<Vec<Domain>, Error> {
    let flags = sys::VIR_CONNECT_LIST_DOMAINS_INACTIVE | sys::VIR_CONNECT_LIST_DOMAINS_ACTIVE;
    let domains = connection.list_all_domains(flags)?;
    Ok(domains)
}
pub fn find_by_id(connection: &Connect, uuid: Uuid) -> Result<Domain, Error> {
    let domain = Domain::lookup_by_uuid(connection, uuid)?;
    Ok(domain)
}

pub fn destroy(connection: &Connect, domain_id: Uuid) -> Result<(), Error> {
    Domain::lookup_by_uuid(connection, domain_id)?.destroy()?;
    Ok(())
}
pub fn shutdown(connection: &Connect, domain_id: Uuid) -> Result<(), Error> {
    Domain::lookup_by_uuid(connection, domain_id)?.shutdown()?;
    Ok(())
}
pub fn undefine(connection: &Connect, domain_id: Uuid) -> Result<(), Error> {
    Domain::lookup_by_uuid(connection, domain_id)?.undefine()?;
    Ok(())
}
pub fn suspend(connection: &Connect, domain_id: Uuid) -> Result<(), Error> {
    Domain::lookup_by_uuid(connection, domain_id)?.suspend()?;
    Ok(())
}
pub fn resume(connection: &Connect, domain_id: Uuid) -> Result<(), Error> {
    Domain::lookup_by_uuid(connection, domain_id)?.resume()?;
    Ok(())
}
pub fn reboot(connection: &Connect, domain_id: Uuid) -> Result<(), Error> {
    Domain::lookup_by_uuid(connection, domain_id)?.reboot(sys::VIR_DOMAIN_REBOOT_DEFAULT)?;
    Ok(())
}
