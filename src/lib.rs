use anyhow::{Context, Result};

pub fn find_matches(content: impl std::io::BufRead, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        let line: String = line.with_context(|| "could not read line")?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| "could not write to writer")?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_match() {
        let data = b"Line 1\nLine 2\nLine 3";
        let cursor = std::io::Cursor::new(data);
        let reader = std::io::BufReader::new(cursor);
        
        let mut result = Vec::new();
        let _ = find_matches(reader, "2", &mut result);
        assert_eq!(result, b"Line 2\n");
    }

    #[test]
    fn find_no_matches() {
        let data = b"Line 1\nLine 2\nLine 3";
        let cursor = std::io::Cursor::new(data);
        let reader = std::io::BufReader::new(cursor);
        
        let mut result = Vec::new();
        let _ = find_matches(reader, "4", &mut result);
        assert_eq!(result, b"");
    }

    #[test]
    fn fail_read_line() {
        struct ErrorReader;

        impl std::io::Read for ErrorReader {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Simulated read error"))
            }
        }

        let reader = std::io::BufReader::new(ErrorReader);
        
        let result = find_matches(reader, "4", Vec::new());
        assert!(result.is_err())
    }
}
