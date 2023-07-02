use crate::{
    sections::{
        element_section::ElementSection, export_section::ExportSection,
        function_section::FunctionSection, global_section::GlobalSection,
        import_section::ImportSection, memory_section::MemorySection, start_section::StartSection,
        type_section::TypeSection,
    },
    CodeSection,
};

#[derive(Debug, Default, PartialEq)]
pub struct Module {
    pub custom_section: Option<()>,
    pub type_section: Option<TypeSection>,
    pub import_section: Option<ImportSection>,
    pub function_section: Option<FunctionSection>,
    pub memory_section: Option<MemorySection>,
    pub global_section: Option<GlobalSection>,
    pub export_section: Option<ExportSection>,
    pub start_section: Option<StartSection>,
    pub element_section: Option<ElementSection>,
    pub code_section: Option<CodeSection>,
}
