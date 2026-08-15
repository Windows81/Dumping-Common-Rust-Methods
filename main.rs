mod util;

fn main() {
    let program_data = {
        let path = std::env::args().next().unwrap();
        std::fs::read(path).unwrap()
    };

    #[cfg(feature = "feat-reqwest")]
    {
        use reqwest::{Client, get};
        analyse!(&program_data, "reqwest", get::<String>);
        analyse!(&program_data, "reqwest", get::<&'static String>);
        analyse!(&program_data, "reqwest", get::<&'static str>);
        analyse!(&program_data, "reqwest", get::<url::Url>);
        analyse!(&program_data, "reqwest", Client::request::<String>);
        analyse!(&program_data, "reqwest", Client::request::<&'static String>);
        analyse!(&program_data, "reqwest", Client::request::<&'static str>);
        analyse!(&program_data, "reqwest", Client::request::<url::Url>);
    }

    #[cfg(feature = "feat-hyper")]
    {
        use http_body_util::{BodyExt, Empty, Full};
        use hyper::{
            Error,
            body::{Body, Bytes, Frame},
        };
        /*
        The functions I included are pertinent to both clients and servers which use Hyper.

        However, many of these functions are inlined, meaning that reverse-engineers
        can't really use them as chokepoints for analysing data which calls those functions.

        And it's difficult to find non-inlined functions which shall carry pertinent HTTP-request information.
        */

        // Corresponds to `parse::<hyper::Uri>()` in "http://httpbin.org/ip".parse::<hyper::Uri>()?`
        // Refer to https://hyper.rs/guides/1/client/basic/#setup for usage.
        // 2026-08-15
        {
            analyse!(&program_data, "hyper", <hyper::Uri as std::str::FromStr>::from_str);
        }

        // Corresponds to `Full::new` in `Full::new(Bytes::from("Hello, World!")))`
        // Refer to https://hyper.rs/guides/1/server/hello-world/#creating-a-service for usage.
        // 2026-08-15
        {
            analyse!(&program_data, "hyper", hyper::Response::<Full<Bytes>>::new);
            analyse!(&program_data, "hyper", <Full<Bytes> as From<&'static str>>::from);
            analyse!(&program_data, "hyper", <Full<Bytes> as From<&'static str>>::from);
            analyse!(&program_data, "hyper", <Full<Bytes> as From<&[u8]>>::from);
            analyse!(&program_data, "hyper", <Full<Bytes> as From<&str>>::from);
            analyse!(&program_data, "hyper", <Full<Bytes> as From<String>>::from);
            analyse!(&program_data, "hyper", <Full<Bytes> as From<Vec<u8>>>::from);
        }

        // Corresponds to `boxed()` in `Empty::<Bytes>::new().map_err(|never| match never {}).boxed()`
        // *The same function can be resolved from `req.into_body().boxed()`*

        // Expect self argument to point to a http_body::Body, which holds some type of data buffer.
        // Refer to https://hyper.rs/guides/1/server/echo/#routing for usage.
        // 2026-08-15
        {
            analyse!(&program_data, "hyper", Empty::<Bytes>::boxed);
        }

        // Chokepoint methods which *might* carry HTTP request bodies.
        // Refer to Hyper's source in files
        //    - "hyper-1.11.0/src/proto/h1/dispatch.rs".
        //    - "hyper-1.11.0/src/body/incoming.rs".
        {
            analyse!(&program_data, "hyper", Frame::<<Full<Bytes> as Body>::Data>::into_data); // poll_write
            analyse!(&program_data, "hyper", Frame::<Bytes>::into_data); // poll_read
            analyse!(&program_data, "hyper", futures_channel::mpsc::Sender::<Result<Bytes, Error>>::try_send); // send_data
        }
    }

    #[cfg(feature = "feat-http")]
    {
        use http::{HeaderValue, header::HeaderName, request::Builder};

        /*
        let req = Request::builder()
          .uri("https://roblox.com")
          .header("Accept", "text/html")
          .header("X-Custom-Foo", "bar")
          .body(())
          .unwrap();
        */
        {
            analyse!(&program_data, "http", Builder::uri::<String>);
            analyse!(&program_data, "http", Builder::uri::<&str>);
            analyse!(&program_data, "http", Builder::body::<String>);
            analyse!(&program_data, "http", Builder::body::<&str>);
            analyse!(&program_data, "http", Builder::header::<HeaderName, HeaderValue>);
        }
    }
}
