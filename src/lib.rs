use pyo3::prelude::*;
use askalono::{Store, TextData, LicenseType, Match};
use pyo3::wrap_pyfunction;
use std::path::Path;
use std::fs;


#[pyclass]
struct LicenseMatch {
    #[pyo3(get)]
    score: f32,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    license_type: String,
}


impl From<Match<'_>> for LicenseMatch {
    fn from(m: Match) -> Self {
        Self {
            score: m.score,
            name: String::from(m.name),
            license_type: m.license_type.to_string(),
        }
    }
}


#[pyclass]
#[text_signature = "()"]
struct LicensePredictor {
    store: Store,
}


#[pymethods]
impl LicensePredictor {
    #[new]
    fn new() -> Self {
        let mut store = Store::new();
        let spdx_json_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/datasets/spdx-json"));
        store
            .load_spdx(spdx_json_path, false)
            .expect("Failed to load spdx_json");
        Self { store }
    }

    #[text_signature = "($self, text)"]
    fn predict_from_text(&self, text: String) -> LicenseMatch {
        let matched: TextData = text.as_str().into();
        LicenseMatch::from(self.store.analyze(&matched))
    }

    #[text_signature = "($self, path)"]
    fn predict_from_file(&self, path: String) -> LicenseMatch {
        let path = Path::new(path.as_str());
        self.predict_from_text(
            fs::read_to_string(path)
                .unwrap_or(String::new())
        )
    }
}


#[pymodule]
fn pyaskalono(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<LicenseMatch>()?;
    module.add_class::<LicensePredictor>()?;

    Ok(())
}
