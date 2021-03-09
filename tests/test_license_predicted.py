from pathlib import Path

import pyaskalono
import pytest


@pytest.fixture(scope="session")
def predictor() -> pyaskalono.LicensePredictor:
    return pyaskalono.LicensePredictor()


@pytest.fixture(scope="session")
def licenses_path() -> Path:
    return Path(__file__).resolve().parent / "licenses"


@pytest.mark.parametrize("license_name", ["Apache-2.0"])
def test_license_predicted_from_file(predictor: pyaskalono.LicensePredictor, licenses_path: Path, license_name: str):
    license_match = predictor.predict_from_file(str(licenses_path / license_name))
    assert license_match.name == license_name
    assert license_match.score == pytest.approx(1.0)


@pytest.mark.parametrize("license_name", ["Apache-2.0"])
def test_license_predicted_from_text(predictor: pyaskalono.LicensePredictor, licenses_path: Path, license_name: str):
    with (licenses_path / license_name).open() as license_file:
        license_text = license_file.read()

    license_match = predictor.predict_from_text(license_text)
    assert license_match.name == license_name
    assert license_match.score == pytest.approx(1.0)
