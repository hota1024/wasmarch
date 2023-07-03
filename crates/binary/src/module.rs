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
    pub custom_section: (),
    pub type_section: TypeSection,
    pub import_section: ImportSection,
    pub function_section: FunctionSection,
    pub memory_section: MemorySection,
    pub global_section: GlobalSection,
    pub export_section: ExportSection,
    pub start_section: StartSection,
    pub element_section: ElementSection,
    pub code_section: CodeSection,
}
