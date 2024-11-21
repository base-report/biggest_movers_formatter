use regex::Regex;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug)]
struct StockTicker {
    symbol: String,
}

struct TickerVariant {
    count: usize,
    prefix: char,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tickers = extract_tickers(&input)?;
    let formatted = format_multiple_variants(&tickers);

    println!("{}", formatted);

    Ok(())
}

fn extract_tickers(input: &str) -> Result<Vec<StockTicker>, Box<dyn Error>> {
    let ticker_pattern = Regex::new(r"\$([A-Z]+)")?;

    let tickers: Vec<StockTicker> = ticker_pattern
        .captures_iter(input)
        .map(|cap| StockTicker {
            symbol: cap[1].to_string(),
        })
        .collect();

    Ok(tickers)
}

fn format_header() -> String {
    let mut header = String::from("📈 Big Movers of the day (ranked by today's price change)\n\n");
    header.push_str("Start tracking these today using our free stock screener: https://base.report/screener?filter_key=IJQVaoxW\n\n");
    header
}

fn format_tickers(tickers: &[StockTicker], count: usize, prefix: char) -> String {
    tickers
        .iter()
        .take(count)
        .map(|ticker| format!("{}{}", prefix, ticker.symbol))
        .collect::<Vec<String>>()
        .join(" ")
}

fn format_multiple_variants(tickers: &[StockTicker]) -> String {
    let variants = vec![
        TickerVariant {
            count: 30,
            prefix: '$',
        },
        TickerVariant {
            count: 4,
            prefix: '$',
        },
        TickerVariant {
            count: 10,
            prefix: '#',
        },
    ];

    variants
        .iter()
        .map(|variant| {
            format!(
                "{}{}",
                format_header(),
                format_tickers(tickers, variant.count, variant.prefix)
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n\n\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tickers() -> Vec<StockTicker> {
        vec![
            StockTicker {
                symbol: String::from("AAPL"),
            },
            StockTicker {
                symbol: String::from("MSFT"),
            },
            StockTicker {
                symbol: String::from("GOOGL"),
            },
            StockTicker {
                symbol: String::from("AMZN"),
            },
            StockTicker {
                symbol: String::from("META"),
            },
        ]
    }

    #[test]
    fn test_extract_tickers() {
        let input = "$AAPL $MSFT $GOOGL";
        let tickers = extract_tickers(input).unwrap();
        assert_eq!(tickers[0].symbol, "AAPL");
        assert_eq!(tickers[1].symbol, "MSFT");
        assert_eq!(tickers[2].symbol, "GOOGL");
    }

    #[test]
    fn test_format_tickers() {
        let tickers = create_test_tickers();
        let formatted = format_tickers(&tickers, 3, '$');
        assert_eq!(formatted, "$AAPL $MSFT $GOOGL");
    }

    #[test]
    fn test_format_multiple_variants() {
        let tickers = create_test_tickers();
        let output = format_multiple_variants(&tickers);
        assert!(output.contains("$AAPL $MSFT $GOOGL $AMZN"));
        assert!(output.contains("#AAPL #MSFT #GOOGL"));
    }
}
