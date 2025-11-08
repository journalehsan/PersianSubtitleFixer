use std::fs;
use std::path::{Path, PathBuf};

use encoding_rs::WINDOWS_1256;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEncoding {
    Auto,
    Utf8,
    Windows1256,
}

impl InputEncoding {
    pub const ALL: [InputEncoding; 3] = [
        InputEncoding::Auto,
        InputEncoding::Utf8,
        InputEncoding::Windows1256,
    ];

    pub fn label(self) -> &'static str {
        match self {
            InputEncoding::Auto => "Auto-detect (UTF-8 → Windows-1256)",
            InputEncoding::Utf8 => "UTF-8",
            InputEncoding::Windows1256 => "Windows-1256 (Arabic)",
        }
    }
}

#[derive(Debug)]
pub struct DecodedFile {
    pub text: String,
    pub used_encoding: InputEncoding,
}

#[derive(Error, Debug)]
pub enum SubtitleError {
    #[error("Failed to read {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to write {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Text is not valid UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub fn read_file(path: &Path, encoding: InputEncoding) -> Result<DecodedFile, SubtitleError> {
    let bytes = fs::read(path).map_err(|source| SubtitleError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    decode_bytes(&bytes, encoding)
}

pub fn write_utf8(path: &Path, contents: &str) -> Result<(), SubtitleError> {
    fs::write(path, contents).map_err(|source| SubtitleError::Write {
        path: path.to_path_buf(),
        source,
    })
}

fn decode_bytes(bytes: &[u8], encoding: InputEncoding) -> Result<DecodedFile, SubtitleError> {
    match encoding {
        InputEncoding::Utf8 => {
            let text = String::from_utf8(bytes.to_vec())?;
            Ok(DecodedFile {
                text,
                used_encoding: InputEncoding::Utf8,
            })
        }
        InputEncoding::Windows1256 => Ok(DecodedFile {
            text: decode_windows_1256(bytes),
            used_encoding: InputEncoding::Windows1256,
        }),
        InputEncoding::Auto => try_auto(bytes),
    }
}

fn try_auto(bytes: &[u8]) -> Result<DecodedFile, SubtitleError> {
    if let Ok(text) = String::from_utf8(bytes.to_vec()) {
        Ok(DecodedFile {
            text,
            used_encoding: InputEncoding::Utf8,
        })
    } else {
        Ok(DecodedFile {
            text: decode_windows_1256(bytes),
            used_encoding: InputEncoding::Windows1256,
        })
    }
}

fn decode_windows_1256(bytes: &[u8]) -> String {
    let (decoded, _, _) = WINDOWS_1256.decode(bytes);
    decoded.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_detects_utf8() {
        let data = "سلام".as_bytes();
        let decoded = decode_bytes(data, InputEncoding::Auto).unwrap();
        assert_eq!(decoded.used_encoding, InputEncoding::Utf8);
        assert_eq!(decoded.text, "سلام");
    }

    #[test]
    fn auto_falls_back_to_windows1256() {
        // 0xe3 is 'س' in Windows-1256
        let data = vec![0xe3, 0xe1, 0xe7, 0xe3];
        let decoded = decode_bytes(&data, InputEncoding::Auto).unwrap();
        assert_eq!(decoded.used_encoding, InputEncoding::Windows1256);
        assert_eq!(decoded.text, "سلام");
    }
}
