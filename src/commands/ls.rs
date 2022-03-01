use anyhow::Result;
use clap::Parser;

use crate::backend::{DecryptReadBackend, FileType};
use crate::blob::tree_iterator;
use crate::id::Id;
use crate::index::IndexBackend;
use crate::repo::SnapshotFile;

#[derive(Parser)]
pub(super) struct Opts {
    /// snapshot to ls
    id: String,
}

pub(super) fn execute(be: &impl DecryptReadBackend, opts: Opts) -> Result<()> {
    let id = Id::from_hex(&opts.id).or_else(|_| {
        // if the given id param is not a full Id, search for a suitable one
        be.find_starts_with(FileType::Snapshot, &[&opts.id])?
            .remove(0)
    })?;

    let snap = SnapshotFile::from_backend(be, &id)?;
    let index = IndexBackend::new(be)?;

    let tree_iter = tree_iterator(&index, vec![snap.tree])?.filter_map(Result::ok);
    for (path, _) in tree_iter {
        println!("{:?} ", path);
    }

    Ok(())
}