use crate::models::{ParseWarning, WarningSeverity};

pub fn warning(
    code: &str,
    message: String,
    line_start: usize,
    line_end: usize,
    severity: WarningSeverity,
) -> ParseWarning {
    ParseWarning {
        code: code.to_string(),
        message,
        line_start,
        line_end,
        severity,
    }
}
