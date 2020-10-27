use std::{fs::File, path::Path};

#[derive(Debug)]
pub struct FileInfo {
    file_handle: File,
    path: Path,
}

// impl Clone for FileInfo {
//     fn clone(&self) -> Self {
//         *self
//     }

//     fn clone_from(&mut self, source: &Self) {
//         *std::clone = source.clone()
//     }
// }
