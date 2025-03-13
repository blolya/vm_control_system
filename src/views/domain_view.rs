use serde::Serialize;
use uuid::Uuid;
use virt::domain::Domain;
use crate::{utils::vm::domain::get_ip_address, repositories::domain_repository};

#[derive(Serialize)]
pub struct DomainView {
    id: u32,
    uuid: Uuid,
    name: String,
    active: bool,
    address: Option<String>,
    vcpu_count: i16,
    memory_size: i16,
    disk_size: i16,
}
impl From<(Domain, domain_repository::Domain)> for DomainView {
    fn from((domain, domain_rep): (Domain, domain_repository::Domain)) -> Self {
        Self {
            id: domain.get_id().unwrap_or(0),
            uuid: domain.get_uuid().unwrap_or(Uuid::new_v4()),
            name: domain_rep.name().to_string(),
            active: domain.is_active().unwrap_or(false),
            address: get_ip_address(&domain).unwrap(),
            vcpu_count: domain_rep.vcpu_count(),
            memory_size: domain_rep.memory_size(),
            disk_size: domain_rep.disk_size(),
        }
    }
}
