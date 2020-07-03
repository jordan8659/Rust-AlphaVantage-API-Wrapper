use reqwest;
use serde_json;

pub mod wrapper {
    pub enum OutputSize {
        Full, 
        Compact,
    }

    impl OutputSize {
        fn value(&self) -> &str {
            match *self {
                OutputSize::Compact => "",
                OutputSize::Full => "",
            }
        }
    }

    pub enum Interval {
        OneMin,
        FiveMin,
        FiftheenMin,
        ThirtyMin,
        SixtyMin,        
    }

    impl Interval {
        fn value(&self) -> &str {
            match *self {
                Interval::OneMin => "1min",
                Interval::FiveMin => "5min",
                Interval::FiftheenMin => "15min",
                Interval::ThirtyMin => "30min",
                Interval::SixtyMin => "60min",
            }
        }
    }

    pub struct IntradayRequest<'a> {
        interval: Interval,
        output_size: OutputSize,
        api_key: &'a str,
    }

    impl<'b> IntradayRequest<'b> {
        pub fn new<'a>(interval: Interval, output_size: OutputSize, api_key: &'a str) -> IntradayRequest{
            IntradayRequest {
                interval,
                output_size,
                api_key,
            }
        }

        pub fn get_intraday_json(client: IntradayRequest, stock_symbol: &str) -> String {
            let request_url = format!("https://www/alphavantage.co/query?function=TIME_SERIES_INTRADAY
                &symbol={symbol}
                &interval={interval}
                &outputsize={output_size}
                &apikey={api_key}",
                symbol = stock_symbol,
                interval = client.interval.value(),
                output_size = client.output_size.value(),
                api_key = client.api_key,
            );
            
            //reqwest::blocking::get(&request_url).text() 

            reqwest::blocking::get(&request_url)
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


