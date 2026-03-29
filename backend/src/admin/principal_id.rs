use crate::error::{AppError, AppResult};
use crate::models::Principal;
use serde::{Deserialize, Serialize};

/// Canonical JSON structure for principal identity encoding.
///
/// `principalId` in admin routes is the URL-safe base64url encoding of:
/// `{"agentId":"...","userId":"..."}`   (keys sorted alphabetically)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrincipalPayload {
    agent_id: String,
    user_id: String,
}

/// Canonical JSON structure for transcript identity encoding.
///
/// `transcriptId` in admin routes is the URL-safe base64url encoding of:
/// `{"sessionId":"...","sessionKey":"..."}`   (keys sorted alphabetically)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranscriptPayload {
    session_id: String,
    session_key: String,
}

/// Encode a `Principal` into an opaque admin-route `principalId`.
pub fn encode_principal_id(principal: &Principal) -> String {
    let payload = PrincipalPayload {
        agent_id: principal.agent_id.clone(),
        user_id: principal.user_id.clone(),
    };
    let json = serde_json::to_string(&payload).expect("principal payload should serialize");
    base64_url_encode(json.as_bytes())
}

/// Decode an opaque admin-route `principalId` back to a `Principal`.
pub fn decode_principal_id(encoded: &str) -> AppResult<Principal> {
    let bytes = base64_url_decode(encoded).map_err(|_| {
        AppError::invalid_request("invalid principalId encoding; expected base64url of canonical JSON")
    })?;
    let payload: PrincipalPayload = serde_json::from_slice(&bytes).map_err(|_| {
        AppError::invalid_request("invalid principalId payload; expected {\"agentId\":\"...\",\"userId\":\"...\"}")
    })?;
    if payload.user_id.trim().is_empty() || payload.agent_id.trim().is_empty() {
        return Err(AppError::invalid_request(
            "principalId must contain non-empty userId and agentId",
        ));
    }
    Ok(Principal {
        user_id: payload.user_id,
        agent_id: payload.agent_id,
    })
}

/// Encode session key + session id into an opaque admin-route `transcriptId`.
pub fn encode_transcript_id(session_key: &str, session_id: &str) -> String {
    let payload = TranscriptPayload {
        session_id: session_id.to_string(),
        session_key: session_key.to_string(),
    };
    let json = serde_json::to_string(&payload).expect("transcript payload should serialize");
    base64_url_encode(json.as_bytes())
}

/// Decode an opaque admin-route `transcriptId` back to (session_key, session_id).
pub fn decode_transcript_id(encoded: &str) -> AppResult<(String, String)> {
    let bytes = base64_url_decode(encoded).map_err(|_| {
        AppError::invalid_request("invalid transcriptId encoding; expected base64url of canonical JSON")
    })?;
    let payload: TranscriptPayload = serde_json::from_slice(&bytes).map_err(|_| {
        AppError::invalid_request("invalid transcriptId payload; expected {\"sessionId\":\"...\",\"sessionKey\":\"...\"}")
    })?;
    if payload.session_key.trim().is_empty() {
        return Err(AppError::invalid_request(
            "transcriptId must contain non-empty sessionKey",
        ));
    }
    Ok((payload.session_key, payload.session_id))
}

/// Base64url encode (no padding).
fn base64_url_encode(data: &[u8]) -> String {
    let standard = base64_encode(data);
    let mut result = String::with_capacity(standard.len());
    for ch in standard.chars() {
        match ch {
            '+' => result.push('-'),
            '/' => result.push('_'),
            '=' => {} // strip padding
            _ => result.push(ch),
        }
    }
    result
}

/// Base64url decode (tolerant of missing padding).
fn base64_url_decode(encoded: &str) -> Result<Vec<u8>, ()> {
    let mut standard = String::with_capacity(encoded.len() + 4);
    for ch in encoded.chars() {
        match ch {
            '-' => standard.push('+'),
            '_' => standard.push('/'),
            _ => standard.push(ch),
        }
    }
    // Add padding.
    let pad = (4 - standard.len() % 4) % 4;
    for _ in 0..pad {
        standard.push('=');
    }
    base64_decode(&standard)
}

/// Simple base64 encode (standard alphabet).
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as u32;
        let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
        result.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
        if i + 1 < data.len() {
            result.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if i + 2 < data.len() {
            result.push(ALPHABET[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        i += 3;
    }
    result
}

/// Simple base64 decode (standard alphabet with padding).
fn base64_decode(encoded: &str) -> Result<Vec<u8>, ()> {
    fn decode_char(c: u8) -> Result<u32, ()> {
        match c {
            b'A'..=b'Z' => Ok((c - b'A') as u32),
            b'a'..=b'z' => Ok((c - b'a' + 26) as u32),
            b'0'..=b'9' => Ok((c - b'0' + 52) as u32),
            b'+' => Ok(62),
            b'/' => Ok(63),
            b'=' => Ok(0),
            _ => Err(()),
        }
    }

    let bytes = encoded.as_bytes();
    if !bytes.len().is_multiple_of(4) {
        return Err(());
    }
    let mut result = Vec::with_capacity(bytes.len() * 3 / 4);
    let mut i = 0;
    while i < bytes.len() {
        let a = decode_char(bytes[i])?;
        let b = decode_char(bytes[i + 1])?;
        let c = decode_char(bytes[i + 2])?;
        let d = decode_char(bytes[i + 3])?;
        let triple = (a << 18) | (b << 12) | (c << 6) | d;
        result.push(((triple >> 16) & 0xFF) as u8);
        if bytes[i + 2] != b'=' {
            result.push(((triple >> 8) & 0xFF) as u8);
        }
        if bytes[i + 3] != b'=' {
            result.push((triple & 0xFF) as u8);
        }
        i += 4;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_principal_id() {
        let principal = Principal {
            user_id: "user:alice".to_string(),
            agent_id: "agent:claude-3".to_string(),
        };
        let encoded = encode_principal_id(&principal);
        let decoded = decode_principal_id(&encoded).expect("should decode");
        assert_eq!(decoded.user_id, "user:alice");
        assert_eq!(decoded.agent_id, "agent:claude-3");
    }

    #[test]
    fn round_trip_transcript_id() {
        let encoded = encode_transcript_id("key:session:1", "id:session:1");
        let (session_key, session_id) =
            decode_transcript_id(&encoded).expect("should decode");
        assert_eq!(session_key, "key:session:1");
        assert_eq!(session_id, "id:session:1");
    }

    #[test]
    fn principal_id_handles_special_chars() {
        let principal = Principal {
            user_id: "user+foo:bar/baz".to_string(),
            agent_id: "agent=qux&quux".to_string(),
        };
        let encoded = encode_principal_id(&principal);
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));
        assert!(!encoded.contains('='));
        let decoded = decode_principal_id(&encoded).expect("should decode");
        assert_eq!(decoded.user_id, principal.user_id);
        assert_eq!(decoded.agent_id, principal.agent_id);
    }
}
