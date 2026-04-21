use leimu_mem::slot_map;

#[derive(Debug)]
pub enum LogError {
    SlotMapError(slot_map::IndexError),
    IoError(std::io::Error),
}

impl core::fmt::Display for LogError {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SlotMapError(_) => write!(f, "slot map error"),
            Self::IoError(_) => write!(f, "IO error"),
        }
    }
}

impl core::error::Error for LogError {

    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::SlotMapError(err) => Some(err),
            Self::IoError(err) => Some(err),
        }
    }
}

impl From<slot_map::IndexError> for LogError {

    fn from(value: slot_map::IndexError) -> Self {
        Self::SlotMapError(value)
    }
}

impl From<std::io::Error> for LogError {

    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
