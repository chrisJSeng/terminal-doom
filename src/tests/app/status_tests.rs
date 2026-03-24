use crate::types::BoolLabelParams;

use super::*;

#[test]
fn bool_label_returns_expected_labels() {
    assert_eq!(
        bool_label(BoolLabelParams {
            flag: false,
            false_label: "OFF",
            true_label: "ON",
        }),
        "OFF"
    );
    assert_eq!(
        bool_label(BoolLabelParams {
            flag: true,
            false_label: "OFF",
            true_label: "ON",
        }),
        "ON"
    );
}
