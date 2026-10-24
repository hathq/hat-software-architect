//! Explicit offline proposal evaluation; never registers or claims a worker.
use crate::worker_io::{canonical_directory, message, write_document};
use hat_software_architect::{
    document,
    review::{self, ReviewInput, SemanticBinding},
};
use std::path::Path;

pub(super) fn run(args: &[String]) -> Result<(), String> {
    let [input, selection, output] = args else {
        return Err("review-v2 requires input.json selected-binding.json output.json".into());
    };
    for name in [input, selection, output] {
        let path = Path::new(name);
        canonical_directory(
            path.parent()
                .and_then(Path::to_str)
                .ok_or("absolute path required")?,
        )?;
        if !path.is_absolute() {
            return Err("absolute path required".into());
        }
    }
    let input: ReviewInput =
        serde_json::from_slice(&document::read(Path::new(input))?).map_err(message)?;
    let selected: SemanticBinding =
        serde_json::from_slice(&document::read(Path::new(selection))?).map_err(message)?;
    let revision = input.revision;
    let output_value = review::review(input, revision, &selected).map_err(message)?;
    write_document(
        Path::new(output),
        &serde_json::to_vec(&output_value).map_err(message)?,
    )?;
    println!("{{\"proposalWritten\":true,\"authorityGranted\":false}}");
    Ok(())
}
