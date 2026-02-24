/*
 * // Copyright (c) 2026 Jack Wang
 * //
 * // Permission is hereby granted, free of charge, to any person obtaining a copy
 * // of this software and associated documentation files (the "Software"), to deal
 * // in the Software without restriction, including without limitation the rights
 * // to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * // copies of the Software, and to permit persons to whom the Software is
 * // furnished to do so, subject to the following conditions:
 * //
 * // The above copyright notice and this permission notice shall be included in all
 * // copies or substantial portions of the Software.
 * //
 * // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * // OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * // SOFTWARE.
 * //
 * // Author: Jack Wang <wang@rjack.cn>
 * // GitHub: https://github.com/nearlyheadlessjack/rcon2mc
 */
use crate::error::RconError;

pub fn check_invalid_command(raw_feedback: &str) -> Result<(), RconError> {
    if raw_feedback.contains("Unknown or incomplete command.") {
        Err(RconError::InvalidCommandError)?
    }
    Ok(())
}

pub trait StringProcessor {
    fn trim_whitespace(&mut self) -> Result<&mut Self, RconError>;
    fn trim_linebreak(&mut self) -> Result<&mut Self, RconError>;
    fn locate_to_useful_content(&mut self, prefix: &str) -> Result<&mut Self, RconError>;
    fn locate_to_useful_content_before(&mut self, prefix: &str) -> Result<&mut Self, RconError>;

    fn segment(&mut self, delimiter: &str) -> Result<Vec<String>, RconError>;
}

impl StringProcessor for String {
    fn trim_whitespace(&mut self) -> Result<&mut Self, RconError> {
        let cleaned = self.replace(' ', "");
        *self = cleaned;
        Ok(self)
    }
    fn trim_linebreak(&mut self) -> Result<&mut Self, RconError> {
        // Delete all linebreak \n, \r, \r\n
        let cleaned = self.replace("\r\n", "").replace('\n', "").replace('\r', "");
        *self = cleaned;
        Ok(self)
    }
    fn locate_to_useful_content(&mut self, prefix: &str) -> Result<&mut Self, RconError> {
        if let Some(pos) = self.find(prefix) {
            let remaining = self[pos + prefix.len()..].to_string();
            *self = remaining;
            Ok(self)
        } else {
            Err(RconError::UnknownParserError(
                "Cannot locate useful content".to_string(),
            ))
        }
    }

    fn locate_to_useful_content_before(&mut self, prefix: &str) -> Result<&mut Self, RconError> {
        if let Some(pos) = self.find(prefix) {
            let remaining = self[..pos].to_string();
            *self = remaining;
            Ok(self)
        } else {
            Err(RconError::UnknownParserError(
                "Cannot locate useful content".to_string(),
            ))
        }
    }
    fn segment(&mut self, delimiter: &str) -> Result<Vec<String>, RconError> {
        let segments: Vec<String> = self
            .split(delimiter)
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            Err(RconError::UnknownParserError(format!(
                "unable to segment the String with {}",
                delimiter
            )))
        } else {
            Ok(segments)
        }
    }
}

mod test {
    #[allow(unused_imports)]
    use crate::parser::utils::StringProcessor;
    #[test]
    fn test_string_processor_linebreak() {
        let mut s = "line1\nline2\nline3\rline4\r\nline5".to_string();
        let result = s.trim_linebreak().unwrap();
        assert_eq!(result.as_str(), "line1line2line3line4line5");
    }
    #[test]
    fn test_string_processor_whitespace() {
        let mut s = "line1 line2      line3  ".to_string();
        let result = s.trim_whitespace().unwrap();
        assert_eq!(result.as_str(), "line1line2line3");
    }
    #[test]
    fn test_string_processor_whitespace_and_linebreak() {
        let mut s = "line1 line2      line3  \nline4 \rline5\r\nline6 \n".to_string();
        let result = s.trim_whitespace().unwrap().trim_linebreak().unwrap();
        assert_eq!(result.as_str(), "line1line2line3line4line5line6");
    }
}
