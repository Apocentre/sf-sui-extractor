use move_core_types::resolver::ModuleResolver;
use eyre::Report;

/// Dumy implementation of the ModuleResolver 
pub struct SuiModuleResolver;

impl ModuleResolver for SuiModuleResolver {
  type Error = Report;

  fn get_module(&self, id: &move_core_types::language_storage::ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
    Ok(None)
  }
}
