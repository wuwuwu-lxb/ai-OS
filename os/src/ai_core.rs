//! AI Core Module
//! Main entry point for AI services in AI-OS

pub mod ai_service {
    pub use super::super::ai_service::*;
}

pub mod vector_db {
    pub use super::super::vector_db::*;
}

pub mod capability {
    pub use super::super::capability::*;
}

pub fn init() {
    capability::init();
    ai_service::init();
    vector_db::init();

    capability::register(
        "file_read",
        "Read file from filesystem",
        capability::CapabilityType::Tool
    );

    capability::register(
        "file_write",
        "Write file to filesystem",
        capability::CapabilityType::Tool
    );

    capability::register(
        "vector_search",
        "Search vector database for similar vectors",
        capability::CapabilityType::Tool
    );

    capability::register(
        "process_create",
        "Create a new AI process",
        capability::CapabilityType::Tool
    );

    capability::register(
        "process_list",
        "List all running AI processes",
        capability::CapabilityType::Tool
    );

    capability::register(
        "memory_store",
        "Store information in AI memory",
        capability::CapabilityType::Skill
    );

    capability::register(
        "memory_retrieve",
        "Retrieve information from AI memory",
        capability::CapabilityType::Skill
    );
}
