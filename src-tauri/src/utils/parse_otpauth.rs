// Helpers for parsing otpauth URIs (minimal implementation)
#[derive(Debug, Clone)]
pub struct ParsedOtp {
    pub issuer: String,
    pub label: String,
    pub r#type: String,
    pub algorithm: String,
    pub digits: i32,
    pub period: Option<i32>,
    pub counter: Option<i32>,
    pub secret: Vec<u8>,
}

pub fn percent_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = &input[i + 1..i + 3];
            if let Ok(val) = u8::from_str_radix(hex, 16) {
                result.push(val as char);
                i += 3;
                continue;
            }
        }
        let ch = if bytes[i] == b'+' {
            ' '
        } else {
            bytes[i] as char
        };
        result.push(ch);
        i += 1;
    }
    result
}

pub fn parse_otpauth(auth_url: &str) -> Option<ParsedOtp> {
    let lower = auth_url.to_lowercase();
    if !lower.starts_with("otpauth://") {
        return None;
    }
    let rest = &auth_url[10..];
    let slash_idx = rest.find('/')?;
    let typ_raw = &rest[..slash_idx];
    let remainder = &rest[slash_idx + 1..];

    let mut label_raw = remainder;
    let mut query = "";
    if let Some(qpos) = remainder.find('?') {
        label_raw = &remainder[..qpos];
        query = &remainder[qpos + 1..];
    }

    let label_decoded = percent_decode(label_raw);

    let mut issuer_from_label: Option<String> = None;
    let label_final: String;
    if let Some(pos) = label_decoded.find(':') {
        issuer_from_label = Some(label_decoded[..pos].trim().to_string());
        label_final = label_decoded[pos + 1..].trim().to_string();
    } else {
        label_final = label_decoded.trim().to_string();
    }

    // Parse query params
    let mut params: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut it = pair.splitn(2, '=');
        let k = it.next().unwrap_or("");
        let v = percent_decode(it.next().unwrap_or(""));
        params.insert(k.to_lowercase(), v);
    }

    let secret = params.get("secret")?.as_bytes().to_vec();
    let issuer = if let Some(iss) = params.get("issuer") {
        if iss.is_empty() {
            issuer_from_label.clone().unwrap_or_default()
        } else {
            iss.clone()
        }
    } else {
        issuer_from_label.clone().unwrap_or_default()
    };

    let algorithm = params
        .get("algorithm")
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "SHA1".to_string());

    let digits = params
        .get("digits")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(6);

    let typ = if typ_raw.eq_ignore_ascii_case("hotp") {
        "HOTP".to_string()
    } else {
        // default to TOTP
        "TOTP".to_string()
    };

    let period = if typ == "TOTP" {
        Some(
            params
                .get("period")
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(30),
        )
    } else {
        None
    };

    let counter = if typ == "HOTP" {
        Some(
            params
                .get("counter")
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0),
        )
    } else {
        None
    };

    Some(ParsedOtp {
        issuer,
        label: label_final,
        r#type: typ,
        algorithm,
        digits,
        period,
        counter,
        secret,
    })
}
