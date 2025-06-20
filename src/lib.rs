// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() <= 2 {
        return Err("Transaction data too short".to_string());
    }

    let ch = raw_tx_hex
        .chars()
        .nth(1)
        .ok_or_else(|| "Input string is too short to extract the second character".to_string())?;

    let val = ch
        .to_digit(16)
        .ok_or_else(|| format!("Hex decode error: invalid digit '{}'", ch))?;

    if val > 9 {
        return Err(format!(
            "Hex decode error: '{}' is not a decimal number (0-9)",
            ch
        ));
    }

    Ok(val)
}
