use reqwest;
use serde_json;

const BASE_URL:&str = "https://www.alphavantage.co/query?";

mod alpha_vantage_wrapper {

    enum OutputSize {
        Full, 
        Compact,
    }

    struct AVClient<'a> {
        interval: u8,
        output_size: OutputSize,
        api_key: &'a str,
    }

    impl<'b> AVClient<'b> {
        pub fn new<'a>(interval: u8, output_size: OutputSize, api_key: &'a str) -> AVClient{
            AVClient {
                interval,
                output_size,
                api_key,
            }
        }
    }

    mod stock_time_series {
        fn weekly_prices(fname: &str, stock_symbol: &str) -> String {
            // dummy return
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


