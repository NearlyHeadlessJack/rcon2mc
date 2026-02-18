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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreatePacketError {
    #[error("Input payload should not contain 0x00")]
    InputPayloadContainsZero,
    #[error("Input payload oversize")]
    InputPayloadOversize,
    #[error("Missing field {0} when creating packet")]
    MissingField(&'static str),
}

#[derive(Error, Debug)]
pub enum BPacketConverterError {
    #[error("Invalid packet: {0}")]
    InvalidPacket(String),
    #[error("Segmenting Error: {0}")]
    SegmentingError(String),
}

#[derive(Error, Debug)]
pub enum RconConnectionError {
    #[error("TCPConnection Timeout")]
    TCPConnectionTimeoutError,
    #[error("TCPConnection Error")]
    TCPConnectionError(#[source] std::io::Error),
    #[error("Missing field {0} when creating TCP Stream")]
    MissingField(&'static str),
    #[error("Stream reading error")]
    StreamReadingError(#[source] std::io::Error),
    #[error("Stream writing error")]
    StreamWritingError(#[source] std::io::Error),
    #[error("Stream closing error")]
    StreamClosingError(#[source] std::io::Error),
}

#[derive(Error, Debug)]
pub enum RconError {
    #[error("Missing field {0} when creating an Rcon connection")]
    MissingField(&'static str),
    #[error("Authentication Failed: Incorrect Password")]
    IncorrectPasswordError,
    #[error("Rcon connection error")]
    RconConnectionError(#[from] RconConnectionError),
    #[error("Failed to create packet")]
    PacketCreationError(#[from] CreatePacketError),
    #[error("Failed to parse received packet")]
    PacketConversionError(#[from] BPacketConverterError),
    #[error("Feedback information is None")]
    FeedbackIsNone,
    #[error("Mismatched Response Packet ID")]
    MismatchedResponsePacketID,
    #[error("Port number is out of range")]
    PortOutOfRangeError,
    #[error("Invalid Command")]
    InvalidCommandError,
    #[error("UnknownParserError for {0}")]
    UnknownParserError(String),
}
