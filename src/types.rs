use std::{path::PathBuf, sync::Arc};

use libloading::Library;

use crate::error::Error;

/// Contains the information for a loaded library.
pub struct Lib {
    /// The actual loaded library. Refer to the libloading documentation on how to use this.
    pub lib: Library,

    /// This is the path from where the library was loaded (which may be in a temporary directory)
    pub object_path: PathBuf,

    /// Original location of the file. This is keep so dynamic_reload knows which file to look for
    /// updates in case the library has been changed.
    pub src_path: Option<PathBuf>,
}

pub unsafe fn init_library(
    src_path: Option<PathBuf>,
    object_path: PathBuf,
) -> Result<Arc<Lib>, Error> {
    match Library::new(&object_path) {
        Ok(l) => Ok(Arc::new(Lib {
            src_path,
            object_path,
            lib: l,
        })),
        Err(e) => Err(Error::Load(e)),
    }
}
