use inexor_rgf_rt_api::RelationTypeExportError;
use inexor_rgf_rt_api::RelationTypeImportError;
use std::sync::Arc;

use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::plugins::RelationTypeImportExportManager;

pub struct RelationTypeImportExportManagerImpl {
    relation_type_import_export_manager: Arc<dyn crate::api::RelationTypeImportExportManager>,
}

impl RelationTypeImportExportManagerImpl {
    pub fn new(relation_type_manager: Arc<dyn crate::api::RelationTypeImportExportManager>) -> Self {
        Self {
            relation_type_import_export_manager: relation_type_manager,
        }
    }
}
impl RelationTypeImportExportManager for RelationTypeImportExportManagerImpl {
    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError> {
        self.relation_type_import_export_manager.import(path)
    }

    fn export(&self, ty: &RelationTypeId, path: &str) -> Result<(), RelationTypeExportError> {
        self.relation_type_import_export_manager.export(ty, path)
    }
}
