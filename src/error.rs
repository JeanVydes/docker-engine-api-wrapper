use crate::api::ErrorMessage;

pub fn err(bytes: &[u8]) -> Result<Box<dyn std::error::Error + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>  {
    let err_string = String::from_utf8(bytes.to_vec())?;
    if err_string.len() == 0 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Server error: Unknown")).into());
    }

    let err: ErrorMessage = serde_json::from_slice(bytes)?;
    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err.message))).into());
}