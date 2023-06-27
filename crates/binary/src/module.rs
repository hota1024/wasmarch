use crate::sections::{
    element_section::ElementSection, export_section::ExportSection,
    function_section::FunctionSection, global_section::GlobalSection,
    import_section::ImportSection, memory_section::MemorySection, start_section::StartSection,
    type_section::TypeSection,
};

#[derive(Debug, Default, PartialEq)]
pub struct Module {
    custom_section: Option<()>,
    type_section: Option<TypeSection>,
    import_section: Option<ImportSection>,
    function_section: Option<FunctionSection>,
    memory_section: Option<MemorySection>,
    global_section: Option<GlobalSection>,
    export_section: Option<ExportSection>,
    start_section: Option<StartSection>,
    element_section: Option<ElementSection>,
}
