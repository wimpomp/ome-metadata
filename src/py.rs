use crate::Ome;
use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "ome_metadata_rs")]
fn ome_metadata_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[pyfn(m)]
    fn ome(text: &str) -> PyResult<Ome> {
        Ok(text.parse()?)
    }

    Ok(())
}
