use uf_product_macros::OrbitalPermissionManifest;

#[derive(OrbitalPermissionManifest)]
#[permission_manifest(
    domain_key = "spectra",
    domain_name = "Spectra",
    domain_description = "Permissions for Spectra observability query surfaces"
)]
pub enum SpectraPermission {
    #[permission(description = "Query Spectra event and metric tables")]
    QueryTable,
}
