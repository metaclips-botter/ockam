use crate::database::migration_20240111100001_add_authority_tables::AuthorityAttributes;
use crate::database::migration_20240111100002_delete_trust_context::PolicyTrustContextId;
use crate::database::migrations::migration_20231231100000_node_name_identity_attributes::NodeNameIdentityAttributes;
use ockam_core::errcode::{Kind, Origin};
use ockam_core::{async_trait, Error, Result};
use sqlx::SqlitePool;

/// This trait runs migrations on a given database
#[async_trait]
pub trait SqlxMigration {
    /// Migrate the content of a database: schema and or data
    async fn migrate(&self, pool: &SqlitePool) -> Result<()>;
}

/// Map a sqlx migration error into an ockam error
#[track_caller]
pub fn map_migrate_err(err: sqlx::migrate::MigrateError) -> Error {
    Error::new(
        Origin::Application,
        Kind::Io,
        format!("migration error {err}"),
    )
}

/// This struct defines the migration to apply to the nodes database
pub struct NodesMigration;

impl NodesMigration {
    pub(crate) async fn migrate_schema(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::migrate!("./src/storage/database/migrations")
            .run(pool)
            .await
            .map_err(map_migrate_err)
    }

    pub(crate) async fn migrate_data(&self, pool: &SqlitePool) -> Result<()> {
        NodeNameIdentityAttributes::migrate_attributes_node_name(pool).await?;
        AuthorityAttributes::migrate_authority_attributes_to_members(pool).await?;
        PolicyTrustContextId::migrate_update_policies(pool).await?;

        Ok(())
    }
}

#[async_trait]
impl SqlxMigration for NodesMigration {
    async fn migrate(&self, pool: &SqlitePool) -> Result<()> {
        self.migrate_schema(pool).await?;
        self.migrate_data(pool).await?;
        Ok(())
    }
}
