# pyaskalono

Python bindings for [askalono](https://github.com/jpeddicord/askalono) library

## Requirements

1. Latest stable rust version, you can obtain it [here](https://www.rust-lang.org/tools/install)
2. Python ^3.6 and [poetry](https://python-poetry.org/)

## How to build?

1. Install project dependencies with `poetry install` 
2. Run `poetry maturin develop` to install into current environment or refer to maturin 
   [documentation](https://github.com/PyO3/maturin) for other options.

## Examples

### Predict from file path

```python
import pyaskalono

LICENSE_PATH = "LICENSE"

# IMPORTANT: This initialization takes up to 20 seconds, make sure you not reinitializing 
# it on each license prediction
predictor = pyaskalono.LicensePredictor()

match_1 = predictor.predict_from_file(LICENSE_PATH)
assert match_1.name == "Apache-2.0"
assert match_1.score == 1.0
```

### Predict from text content

```python
import pyaskalono

LICENSE_PATH = "LICENSE"

# IMPORTANT: This initialization takes up to 20 seconds, make sure you don't reinitializing
# it on each license prediction
predictor = pyaskalono.LicensePredictor()

with open(LICENSE_PATH) as file:
   license_text = file.read()

match_2 = predictor.predict_from_text(license_text)
assert match_2.name == "Apache-2.0"
assert match_2.score == 1.0
```