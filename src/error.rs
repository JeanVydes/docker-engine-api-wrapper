use crate::api::ErrorMessage;

pub fn err(bytes: &[u8]) -> Result<Box<dyn std::error::Error + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>  {
    let err: ErrorMessage = serde_json::from_slice(bytes)?;
    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err.message))).into());
}