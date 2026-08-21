use std::{fs::File, io::Write};

mod util;

fn main() {
    let mut args_iter = std::env::args();

    // Program data
    let prog = {
        let prog_path = args_iter.next().unwrap();
        std::fs::read(prog_path).unwrap()
    };

    let writer = {
        let out_path = args_iter.next().unwrap();
        File::create(out_path).unwrap()
    };

    #[cfg(feature = "feat-reqwest")]
    {
        use reqwest::{Client, get};
        analyse!(&prog, &writer, "reqwest", get::<String>);
        analyse!(&prog, &writer, "reqwest", get::<&'static String>);
        analyse!(&prog, &writer, "reqwest", get::<&'static str>);
        analyse!(&prog, &writer, "reqwest", get::<url::Url>);
        analyse!(&prog, &writer, "reqwest", Client::request::<String>);
        analyse!(&prog, &writer, "reqwest", Client::request::<&'static String>);
        analyse!(&prog, &writer, "reqwest", Client::request::<&'static str>);
        analyse!(&prog, &writer, "reqwest", Client::request::<url::Url>);
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
            analyse!(&prog, &writer, "hyper", <hyper::Uri as std::str::FromStr>::from_str);
        }

        // Corresponds to `Full::new` in `Full::new(Bytes::from("Hello, World!")))`
        // Refer to https://hyper.rs/guides/1/server/hello-world/#creating-a-service for usage.
        // 2026-08-15
        {
            analyse!(&prog, &writer, "hyper", hyper::Response::<Full<Bytes>>::new);
            analyse!(&prog, &writer, "hyper", <Full<Bytes> as From<&'static str>>::from);
            analyse!(&prog, &writer, "hyper", <Full<Bytes> as From<&'static str>>::from);
            analyse!(&prog, &writer, "hyper", <Full<Bytes> as From<&[u8]>>::from);
            analyse!(&prog, &writer, "hyper", <Full<Bytes> as From<&str>>::from);
            analyse!(&prog, &writer, "hyper", <Full<Bytes> as From<String>>::from);
            analyse!(&prog, &writer, "hyper", <Full<Bytes> as From<Vec<u8>>>::from);
        }

        // Corresponds to `boxed()` in `Empty::<Bytes>::new().map_err(|never| match never {}).boxed()`
        // *The same function can be resolved from `req.into_body().boxed()`*

        // Expect self argument to point to a http_body::Body, which holds some type of data buffer.
        // Refer to https://hyper.rs/guides/1/server/echo/#routing for usage.
        // 2026-08-15
        {
            analyse!(&prog, &writer, "hyper", Empty::<Bytes>::boxed);
        }

        // Chokepoint methods which *might* carry HTTP request bodies.
        // Refer to Hyper's source in files
        //    - "hyper-1.11.0/src/proto/h1/dispatch.rs".
        //    - "hyper-1.11.0/src/body/incoming.rs".
        {
            analyse!(&prog, &writer, "hyper", Frame::<<Full<Bytes> as Body>::Data>::into_data); // poll_write
            analyse!(&prog, &writer, "hyper", Frame::<Bytes>::into_data); // poll_read
            analyse!(&prog, &writer, "hyper", futures_channel::mpsc::Sender::<Result<Bytes, Error>>::try_send); // send_data
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
            analyse!(&prog, &writer, "http", Builder::uri::<String>);
            analyse!(&prog, &writer, "http", Builder::uri::<&str>);
            analyse!(&prog, &writer, "http", Builder::body::<String>);
            analyse!(&prog, &writer, "http", Builder::body::<&str>);
            analyse!(&prog, &writer, "http", Builder::header::<HeaderName, HeaderValue>);
        }
    }
}
