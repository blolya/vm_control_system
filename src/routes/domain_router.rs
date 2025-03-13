use anyhow::anyhow;
use axum::{
    extract::{Json, Path, State},
    routing::{delete, get, post},
    Router,
};
use serde::Deserialize;
use std::{env, fs};
use uuid::Uuid;

use crate::{
    utils::{
        vm::domain,
        auth::claims::Claims,
        jsend::JSend,
        result::Result,
        vm
    },
    AppState,
    errors::Error,
    views::domain_view::DomainView
};

pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/domains",
        Router::new().route("/", get(list).post(create)).nest(
            "/{id}",
            Router::new()
                .route("/", delete(destroy))
                .route("/reboot", post(reboot))
                .route("/resume", post(resume))
                .route("/pause", post(pause)),
        ),
    )
}

#[derive(Deserialize)]
struct CreateDomain {
    name: String,
    vcpu_count: i16,
    memory_size: i16,
    disk_size: i16,
    hostname: String,
    username: String,
    ssh_key_id: Uuid,
    os_image_id: Uuid,
    user_id: Uuid,
}
async fn create(
    claims: Claims,
    State(AppState {
        domain_repository,
        ssh_key_repository,
        os_image_repository,
        hypervisor_connection,
        ..
    }): State<AppState>,
    Json(CreateDomain {
        name,
        vcpu_count,
        memory_size,
        disk_size,
        hostname,
        username,
        ssh_key_id,
        os_image_id,
        user_id,
    }): Json<CreateDomain>,
) -> Result<JSend<Uuid>> {
    let ssh_key = ssh_key_repository
        .find_for_user_by_id(claims.data().user_id, ssh_key_id)
        .await?;
    let os_image = os_image_repository.find_by_id(os_image_id).await?;

    let domain_id = Uuid::new_v4();
    let base_dir = env::var("BASE_PATH").unwrap();
    let vm_directory = format!("{base_dir}/vms/{domain_id}");

    let domain = vm::init_vm(
        &hypervisor_connection,
        &vm_directory,
        &os_image.path,
        &domain_id,
        &domain_id,
        vcpu_count,
        memory_size,
        disk_size,
        &hostname,
        &username,
        &ssh_key.key,
    )
    .await
    .map_err(|e| {
        fs::remove_dir_all(vm_directory).unwrap();
        Error::Anyhow(anyhow!(e))
    })?;

    domain_repository
        .create(
            domain_id,
            user_id,
            os_image_id,
            name,
            vcpu_count,
            memory_size,
            disk_size,
        )
        .await?;

    Ok(JSend::success(Some(domain_id)))
}
async fn list(
    _claims: Claims,
    State(AppState {
        domain_repository, hypervisor_connection, ..
    }): State<AppState>,
) -> Result<JSend<Vec<DomainView>>> {
    let domains = domain_repository.list().await?;
    let domains = domains.into_iter().map(|d| (domain::find_by_id(&hypervisor_connection, *d.id()).unwrap(), d)).map(DomainView::from).collect();
    Ok(JSend::success(Some(domains)))
}
async fn pause(
    _claims: Claims,
    State(AppState {
        hypervisor_connection,
        ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<JSend<()>> {
    vm::suspend_vm(&hypervisor_connection, id)?;
    Ok(JSend::success(Some(())))
}
async fn destroy(
    _claims: Claims,
    State(AppState {
        domain_repository,
        hypervisor_connection,
        ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<JSend<()>> {
    let domain = domain::find_by_id(&hypervisor_connection, id)?;
    if (domain.is_active()?) {
        vm::destroy_vm(&hypervisor_connection, id).map_err(|e| Error::Anyhow(anyhow!(e)))?;
        vm::undefine_vm(&hypervisor_connection, id).map_err(|e| Error::Anyhow(anyhow!(e)))?;
    } else {
        vm::undefine_vm(&hypervisor_connection, id).map_err(|e| Error::Anyhow(anyhow!(e)))?;
    }
    let base_dir = env::var("BASE_PATH").unwrap();
    let vm_directory = format!("{base_dir}/vms/{id}");
    fs::remove_dir_all(&vm_directory).map_err(|e| Error::Anyhow(anyhow!(e)))?;

    domain_repository.delete(id).await?;
    Ok(JSend::success(Some(())))
}
async fn reboot(
    _claims: Claims,
    State(AppState {
        domain_repository,
        hypervisor_connection,
        ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<JSend<()>> {
    vm::reboot_vm(&hypervisor_connection, id)?;
    Ok(JSend::success(Some(())))
}
async fn resume(
    _claims: Claims,
    State(AppState {
        domain_repository,
        hypervisor_connection,
        ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<JSend<()>> {
    vm::resume_vm(&hypervisor_connection, id)?;
    Ok(JSend::success(Some(())))
}
