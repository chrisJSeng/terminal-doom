use crate::constants::{WAD_CANDIDATES, WAD_NOT_FOUND_MESSAGE};

use super::wad_not_found_error;

#[test]
fn wad_not_found_error_reports_candidates_and_context() {
    let err = wad_not_found_error();
    let message = err.to_string();

    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    assert!(message.contains(WAD_NOT_FOUND_MESSAGE));
    assert!(message.contains("Diretorio atual:"));
    assert!(message.contains("Candidatos verificados:"));
    for candidate in WAD_CANDIDATES {
        assert!(message.contains(candidate));
    }
}
