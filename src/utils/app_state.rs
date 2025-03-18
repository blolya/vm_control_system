use virt::connect::Connect;

use crate::repositories::{
    domain_repository::DomainRepository, os_image_repository::OsImageRepository,
    refresh_token_repository::RefreshTokenRepository, ssh_key_repository::SshKeyRepository,
    user_repository::UserRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub user_repository: UserRepository,
    pub refresh_token_repository: RefreshTokenRepository,
    pub domain_repository: DomainRepository,
    pub os_image_repository: OsImageRepository,
    pub ssh_key_repository: SshKeyRepository,
    pub hypervisor_connection: Connect,
}
impl AppState {
    pub fn new(
        user_repository: UserRepository,
        refresh_token_repository: RefreshTokenRepository,
        domain_repository: DomainRepository,
        os_image_repository: OsImageRepository,
        ssh_key_repository: SshKeyRepository,
        hypervisor_connection: Connect,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
            domain_repository,
            os_image_repository,
            ssh_key_repository,
            hypervisor_connection,
        }
    }
}
