use tokio::net::TcpStream;
use std::io::ErrorKind;
use std::error::Error;
use std::time::SystemTime;
use httparse::Status;
use httpdate::fmt_http_date;
use tokio_native_tls::TlsStream;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use crate::icy;

pub enum Stream {
    Plain( TcpStream ),
    Tls( Box< TlsStream< TcpStream > > )
}

impl Stream {
    pub async fn read( &mut self, buf: &mut [ u8 ] ) -> std::io::Result< usize > {
        match self {
            Stream::Plain( stream ) => stream.read( buf ).await,
            Stream::Tls( stream ) => stream.read( buf ).await
        }
    }

    pub async fn write_all( &mut self, buf: &[ u8 ] ) -> std::io::Result< () > {
        match self {
            Stream::Plain( stream ) => stream.write_all( buf ).await,
            Stream::Tls( stream ) => stream.write_all( buf ).await
        }
    }
}

pub async fn send_bad_request( stream: &mut TcpStream, id: &str, message: Option< ( &str, &str ) > ) -> Result< (), Box< dyn Error > > {
    stream.write_all( b"HTTP/1.0 400 Bad Request\r\n" ).await?;
    stream.write_all( ( format!( "Server: {}\r\n", id ) ).as_bytes() ).await?;
    stream.write_all( b"Connection: Close\r\n" ).await?;
    if let Some( ( content_type, text ) ) = message {
        stream.write_all( ( format!( "Content-Type: {}\r\n", content_type ) ).as_bytes() ).await?;
        stream.write_all( ( format!( "Content-Length: {}\r\n", text.len() ) ).as_bytes() ).await?;
    }
    server_info(stream).await?;
    if let Some( ( _, text ) ) = message {
        stream.write_all( text.as_bytes() ).await?;
    }

    Ok( () )
}


pub async fn send_forbidden( stream: &mut TcpStream, id: &str, message: Option< ( &str, &str ) > ) -> Result< (), Box< dyn Error > > {
    stream.write_all( b"HTTP/1.0 403 Forbidden\r\n" ).await?;
    stream.write_all( ( format!( "Server: {}\r\n", id ) ).as_bytes() ).await?;
    stream.write_all( b"Connection: Close\r\n" ).await?;
    if let Some( ( content_type, text ) ) = message {
        stream.write_all( ( format!( "Content-Type: {}\r\n", content_type ) ).as_bytes() ).await?;
        stream.write_all( ( format!( "Content-Length: {}\r\n", text.len() ) ).as_bytes() ).await?;
    }
    server_info(stream).await?;
    if let Some( ( _, text ) ) = message {
        stream.write_all( text.as_bytes() ).await?;
    }

    Ok( () )
}

pub async fn send_unauthorized( stream: &mut TcpStream, id: &str, message: Option< ( &str, &str ) > ) -> Result< (), Box< dyn Error > > {
    stream.write_all( b"HTTP/1.0 401 Authorization Required\r\n" ).await?;
    stream.write_all( ( format!( "Server: {}\r\n", id ) ).as_bytes() ).await?;
    stream.write_all( b"Connection: Close\r\n" ).await?;
    if let Some( ( content_type, text ) ) = message {
        stream.write_all( ( format!( "Content-Type: {}\r\n", content_type ) ).as_bytes() ).await?;
        stream.write_all( ( format!( "Content-Length: {}\r\n", text.len() ) ).as_bytes() ).await?;
    }
    stream.write_all( b"WWW-Authenticate: Basic realm=\"Icy Server\"\r\n" ).await?;
    server_info(stream).await?;
    if let Some( ( _, text ) ) = message {
        stream.write_all( text.as_bytes() ).await?;
    }

    Ok( () )
}

pub async fn server_info( stream: &mut TcpStream )  -> Result< (), Box< dyn Error > > {
    stream.write_all( ( format!( "Date: {}\r\n", fmt_http_date( SystemTime::now() ) ) ).as_bytes() ).await?;
    stream.write_all( b"Cache-Control: no-cache, no-store\r\n" ).await?;
    stream.write_all( b"Expires: Mon, 26 Jul 1997 05:00:00 GMT\r\n" ).await?;
    stream.write_all( b"Pragma: no-cache\r\n" ).await?;
    stream.write_all( b"Access-Control-Allow-Origin: *\r\n\r\n" ).await?;

    Ok( () )
}

pub async fn send_listener_ok( stream: &mut TcpStream, id: &str, properties: &icy::IcyProperties, meta_enabled: bool, metaint: usize ) -> Result< (), Box< dyn Error > > {
    stream.write_all( b"HTTP/1.0 200 OK\r\n" ).await?;
    stream.write_all( ( format!( "Server: {}\r\n", id ) ).as_bytes() ).await?;
    stream.write_all( b"Connection: Close\r\n" ).await?;
    stream.write_all( ( format!( "Date: {}\r\n", fmt_http_date( SystemTime::now() ) ) ).as_bytes() ).await?;
    stream.write_all( ( format!( "Content-Type: {}\r\n", properties.content_type ) ).as_bytes() ).await?;
    stream.write_all( b"Cache-Control: no-cache, no-store\r\n" ).await?;
    stream.write_all( b"Expires: Mon, 26 Jul 1997 05:00:00 GMT\r\n" ).await?;
    stream.write_all( b"Pragma: no-cache\r\n" ).await?;
    stream.write_all( b"Access-Control-Allow-Origin: *\r\n" ).await?;

    // If metaint is enabled
    if meta_enabled {
        stream.write_all( ( format!( "icy-metaint:{}\r\n", metaint ) ).as_bytes() ).await?;
    }

    // Properties or default
    if let Some( br ) = properties.bitrate.as_ref() {
        stream.write_all( ( format!( "icy-br:{}\r\n", br ) ).as_bytes() ).await?;
    }
    stream.write_all( ( format!( "icy-description:{}\r\n", properties.description.as_ref().unwrap_or( &"Unknown".to_string() ) ) ).as_bytes() ).await?;
    stream.write_all( ( format!( "icy-genre:{}\r\n", properties.genre.as_ref().unwrap_or( &"Undefined".to_string() ) ) ).as_bytes() ).await?;
    stream.write_all( ( format!( "icy-name:{}\r\n", properties.name.as_ref().unwrap_or( &"Unnamed Station".to_string() ) ) ).as_bytes() ).await?;
    stream.write_all( ( format!( "icy-pub:{}\r\n", properties.public as usize ) ).as_bytes() ).await?;
    stream.write_all( ( format!( "icy-url:{}\r\n\r\n", properties.url.as_ref().unwrap_or( &"Unknown".to_string() ) ) ).as_bytes() ).await?;

    Ok( () )
}

pub async fn send_not_found( stream: &mut TcpStream, id: &str, message: Option< ( &str, &str ) > ) -> Result< (), Box< dyn Error > > {
    stream.write_all( b"HTTP/1.0 404 File Not Found\r\n" ).await?;
    stream.write_all( ( format!( "Server: {}\r\n", id ) ).as_bytes() ).await?;
    stream.write_all( b"Connection: Close\r\n" ).await?;
    if let Some( ( content_type, text ) ) = message {
        stream.write_all( ( format!( "Content-Type: {}\r\n", content_type ) ).as_bytes() ).await?;
        stream.write_all( ( format!( "Content-Length: {}\r\n", text.len() ) ).as_bytes() ).await?;
    }
    stream.write_all( ( format!( "Date: {}\r\n", fmt_http_date( SystemTime::now() ) ) ).as_bytes() ).await?;
    stream.write_all( b"Cache-Control: no-cache, no-store\r\n" ).await?;
    stream.write_all( b"Expires: Mon, 26 Jul 1997 05:00:00 GMT\r\n" ).await?;
    stream.write_all( b"Pragma: no-cache\r\n" ).await?;
    stream.write_all( b"Access-Control-Allow-Origin: *\r\n\r\n" ).await?;
    if let Some( ( _, text ) ) = message {
        stream.write_all( text.as_bytes() ).await?;
    }

    Ok( () )
}

pub async fn send_ok( stream: &mut TcpStream, id: &str, message: Option< ( &str, &str ) > ) -> Result< (), Box< dyn Error > > {
    stream.write_all( b"HTTP/1.0 200 OK\r\n" ).await?;
    stream.write_all( ( format!( "Server: {}\r\n", id ) ).as_bytes() ).await?;
    stream.write_all( b"Connection: Close\r\n" ).await?;
    if let Some( ( content_type, text ) ) = message {
        stream.write_all( ( format!( "Content-Type: {}\r\n", content_type ) ).as_bytes() ).await?;
        stream.write_all( ( format!( "Content-Length: {}\r\n", text.len() ) ).as_bytes() ).await?;
    }
    server_info(stream).await?;
    if let Some( ( _, text ) ) = message {
        stream.write_all( text.as_bytes() ).await?;
    }

    Ok( () )
}

pub async fn send_continue( stream: &mut TcpStream, id: &str ) -> Result< (), Box< dyn Error > > {
    stream.write_all( b"HTTP/1.0 200 OK\r\n" ).await?;
    stream.write_all( ( format!( "Server: {}\r\n", id ) ).as_bytes() ).await?;
    stream.write_all( b"Connection: Close\r\n" ).await?;
    server_info(stream).await?;
    Ok( () )
}

pub async fn send_internal_error( stream: &mut TcpStream, id: &str, message: Option< ( &str, &str ) > ) -> Result< (), Box< dyn Error > > {
    stream.write_all( b"HTTP/1.0 500 Internal Server Error\r\n" ).await?;
    stream.write_all( ( format!( "Server: {}\r\n", id ) ).as_bytes() ).await?;
    stream.write_all( b"Connection: Close\r\n" ).await?;
    if let Some( ( content_type, text ) ) = message {
        stream.write_all( ( format!( "Content-Type: {}\r\n", content_type ) ).as_bytes() ).await?;
        stream.write_all( ( format!( "Content-Length: {}\r\n", text.len() ) ).as_bytes() ).await?;
    }
    server_info(stream).await?;
    if let Some( ( _, text ) ) = message {
        stream.write_all( text.as_bytes() ).await?;
    }

    Ok( () )
}

pub async fn read_http_response( stream: &mut Stream, buffer: &mut Vec< u8 >, max_len: usize ) -> Result< usize, Box< dyn Error > > {
    let mut buf = [ 0; 1024 ];
    loop {
        let mut headers = [ httparse::EMPTY_HEADER; 32 ];
        let mut res = httparse::Response::new( &mut headers );
        let read = stream.read( &mut buf ).await?;
        buffer.extend_from_slice( &buf[ .. read ] );
        match res.parse( &buffer ) {
            Ok( Status::Complete( offset ) ) => return Ok( offset ),
            Ok( Status::Partial ) if buffer.len() > max_len => return Err( Box::new( std::io::Error::new( ErrorKind::Other, "Request exceeded the maximum allowed length" ) ) ),
            Ok( Status::Partial ) => (),
            Err( e ) => return Err( Box::new( std::io::Error::new( ErrorKind::InvalidData, format!( "Received an invalid request: {}", e ) ) ) )
        }
    }
}
