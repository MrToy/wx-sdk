//! # wx_func API Documentation
//! ## `wx_func` is a [WeChat SDK](https://mp.weixin.qq.com/) written in [Rust](https://www.rust-lang.org/).
//! ## Features
//! Fealtures can be checked at [README page](https://github.com/ilovelll/wx_func/blob/main/README.md)

//! ## QuickStart

//! First, please refer to this [page](https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Access_Overview.html) to provide these values: `token`, `EncodingAESKey`,`EncodingMode`.
//! ```rust
//! use wx_func::wechat::{ServerConfig, EncodingMode};
//!
//! let config = ServerConfig::new(token, Some("aes_key"), EncodingMode::Plain);
//! let sdk = WxSdk::new_with_default_token_client("app_id", "app_secret", config);
//! ```
//! Then, you can use the sdk functions, like get current menu info:
//! ```rust
//! use wx_func::office_account::menu;
//!
//! let menu = menu::get_current_selfmenu_info(&sdk);
//! ```

//! ## Contributing

//! Issue reports and Pull Requests are always welcome!

//! ## License

//! wx_func is available under the [_MIT License_](https://github.com/ilovelll/wx_func/blob/main/LICENSE)


pub mod access_token;

pub use access_token::AccessToken;

pub mod error;
pub mod office_account;
pub use error::SdkResult;
pub mod wechat;
pub use access_token::TokenClient;
