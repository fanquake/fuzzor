mod libfuzzer;
mod native_go;
mod semsan;

pub use libfuzzer::LibFuzzerReproducer;
pub use native_go::NativeGoReproducer;
pub use semsan::SemSanReproducer;

use std::error::Error;
use std::path::PathBuf;

use fuzzor_infra::ReproducedSolution;

#[async_trait::async_trait]
pub trait Reproducer<E: Error> {
    async fn reproduce(&self) -> Result<ReproducedSolution, E>;
}

fn create_cloned_files(path: &PathBuf) -> Result<(std::fs::File, std::fs::File), std::io::Error> {
    let stderr = std::fs::File::create(&path)?;
    let stdout = stderr.try_clone()?;
    Ok((stderr, stdout))
}
