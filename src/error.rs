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
    #[error("Input payload should not end with 0x00")]
    InputPayloadEndWithZero,
    #[error("Input payload oversize")]
    InputPayloadOversize,
    #[error("Missing filed {0} when creating packet")]
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
    #[error("TCPConnection Error: {0}")]
    TCPConnectionError(String),
    #[error("Missing filed {0} when create TCP Stream")]
    MissingField(&'static str),
    #[error("Stream reading error: {0}")]
    StreamReadingError(String),
    #[error("Stream writing error: {0}")]
    StreamWritingError(String),
    #[error("Stream closing error: {0}")]
    StreamClosingError(String),
}

#[derive(Error, Debug)]
pub enum RconError {
    #[error("Missing filed {0} when create a Rcon connection")]
    MissingField(&'static str),
    #[error("Authentication Failed")]
    AuthenticationFailed,
    #[error("Error in Authentication:{0}")]
    AuthenticationError(String),
    #[error("Rcon connection error: {0}")]
    RconConnectionError(String),
    #[error("Rcon send packet error: {0}")]
    RconSendPacketError(String),
    #[error("Rcon receive packet error: {0}")]
    RconReceivePacketError(String),
    #[error("Feedback information is None")]
    FeedbackIsNone,
    #[error("Rcon connection shutdown error: {0}")]
    RconShutdownError(String),
}
