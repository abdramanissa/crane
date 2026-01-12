use std::{fs, path::PathBuf};

mod assembly;
mod c;

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Languages {
    Assembly,
    C,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub language: Languages,
}

impl Project {
    pub fn new(name: String, language: Languages) -> Self {
        Self { name, language }
    }
    pub fn create(&self) -> std::io::Result<()> {
        match self.language {
            Languages::Assembly => assembly::create_assembly_project(&self.name),
            Languages::C => c::create_c_project(&self.name),
        }
        Ok(())
    }
}
