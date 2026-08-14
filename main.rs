mod util;

fn main() {
    let program_data = {
        let path = std::env::args().next().unwrap();
        std::fs::read(path).unwrap()
    };

    #[cfg(feature = "dep_reqwest")]
    {
        analyse!(&program_data, reqwest::get::<String>);
        analyse!(&program_data, reqwest::get::<&'static String>);
        analyse!(&program_data, reqwest::get::<&'static str>);
        analyse!(&program_data, reqwest::get::<url::Url>);

        analyse!(&program_data, reqwest::Client::request::<String>);
        analyse!(&program_data, reqwest::Client::request::<&'static String>);
        analyse!(&program_data, reqwest::Client::request::<&'static str>);
        analyse!(&program_data, reqwest::Client::request::<url::Url>);
    }
}
