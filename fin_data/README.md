# Financial Data Fetcher in Rust

## Overview
This project aims to build a Rust application that periodically fetches and records the pricing data of Bitcoin, Ethereum, and the S&P 500 index. The application will:
* Define three structs: `Bitcoin`, `Ethereum`, and `SP500`.
* Create a `Pricing` trait and implement it for each struct.
* Utilize the `ureq` library for HTTP requests and the `serde` library for JSON parsing.
* Continuously fetch data every 10 seconds.
* Save the fetched data into three separate files.
* Provide a screenshot of the running program.
* Share a GitHub repository link containing the source code.
