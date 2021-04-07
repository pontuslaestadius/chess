#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KingStatus {
    Safe,
    Check,
    Mate,
}

impl From<&str> for KingStatus {
    fn from(item: &str) -> Self {
        if item.contains('+') {
            KingStatus::Check
        } else if item.contains('#') {
            KingStatus::Mate
        } else {
            KingStatus::Safe
        }
    }
}
