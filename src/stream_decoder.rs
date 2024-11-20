use std::io::ErrorKind;
use std::error::Error;

#[derive(PartialEq)]
pub enum TransferEncoding {
    Identity,
    Chunked,
    Length(usize),
}

pub struct StreamDecoder {
    encoding: TransferEncoding,
    remainder: usize,
    chunk: Vec<u8>,
}

impl StreamDecoder {
    pub fn new(encoding: TransferEncoding) -> StreamDecoder {
        let remainder = match &encoding {
            TransferEncoding::Length(v) => *v,
            _ => 1,
        };
        StreamDecoder {
            encoding,
            remainder,
            chunk: Vec::new(),
        }
    }

    pub fn decode(
        &mut self,
        out: &mut Vec<u8>,
        buf: &[u8],
        length: usize,
    ) -> Result<usize, Box<dyn Error + Send>> {
        if length == 0 || self.is_finished() {
            Ok(0)
        } else {
            match &self.encoding {
                TransferEncoding::Identity => {
                    out.extend_from_slice(&buf[..length]);
                    Ok(length)
                }
                TransferEncoding::Chunked => {
                    let mut read = 0;
                    let mut index = 0;
                    while index < length && self.remainder != 0 {
                        match self.remainder {
                            1 => {
                                // Get the chunk size
                                self.chunk.push(buf[index]);
                                index += 1;
                                if self.chunk.windows(2).nth_back(0) == Some(b"\r\n") {
                                    // Ignore chunk extensions
                                    if let Some(cutoff) =
                                        self.chunk.iter().position(|&x| x == b';' || x == b'\r')
                                    {
                                        self.remainder = match std::str::from_utf8( &self.chunk[ .. cutoff ] ) {
                                            Ok( res ) =>  match usize::from_str_radix( res, 16 ) {
                                                Ok( hex ) => hex,
                                                Err( e ) => return Err( Box::new( std::io::Error::new( ErrorKind::InvalidData, format!( "Invalid value provided for chunk size: {}", e ) ) ) )
                                            }
                                            Err( e ) => return Err( Box::new( std::io::Error::new( ErrorKind::InvalidData, format!( "Could not parse chunk size: {}", e ) ) ) )
                                        };
                                        // Check if it's the last chunk
                                        // Ignore trailers
                                        if self.remainder != 0 {
                                            // +2 for remainder
                                            // +2 for extra CRLF
                                            self.remainder += 4;
                                            self.chunk.clear();
                                        }
                                    } else {
                                        return Err(Box::new(std::io::Error::new(
                                            ErrorKind::InvalidData,
                                            "Missing CRLF",
                                        )));
                                    }
                                }
                            }
                            2 => {
                                // No more chunk data should be read
                                if self.chunk.windows(2).nth_back(0) == Some(b"\r\n") {
                                    // Append current data
                                    read += self.chunk.len() - 2;
                                    out.extend_from_slice(&self.chunk[..self.chunk.len() - 2]);
                                    // Prepare for reading the next chunk size
                                    self.remainder = 1;
                                    self.chunk.clear();
                                } else {
                                    return Err(Box::new(std::io::Error::new(
                                        ErrorKind::InvalidData,
                                        "Missing CRLF from chunk",
                                    )));
                                }
                            }
                            v => {
                                // Get the chunk data
                                let max_read = std::cmp::min(length - index, v - 2);
                                self.chunk.extend_from_slice(&buf[index..index + max_read]);
                                index += max_read;
                                self.remainder -= max_read;
                            }
                        }
                    }

                    Ok(read)
                }
                TransferEncoding::Length(_) => {
                    let allowed = std::cmp::min(length, self.remainder);
                    if allowed != 0 {
                        out.extend_from_slice(&buf[..allowed]);
                        self.remainder -= allowed;
                    }
                    Ok(allowed)
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.encoding != TransferEncoding::Identity && self.remainder == 0
    }
}
