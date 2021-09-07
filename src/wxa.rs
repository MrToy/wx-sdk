use crate::{access_token::AccessTokenProvider, error::CommonResponse};
use crate::{wechat::WxApiRequestBuilder, SdkResult, WxSdk};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

pub mod data_analysis;

#[derive(Debug, Serialize)]
pub struct QuerySession {
    /// 小程序 appId
    pub appid: String,
    /// 小程序 appSecret
    pub secret: String,
    /// 登录时获取的 code
    pub js_code: String,
    // 授权类型，此处只需填写 authorization_code
    // pub grant_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    /// 用户唯一标识
    pub openid: String,
    /// 会话密钥
    #[serde(skip_serializing)]
    pub session_key: String,
    /// 用户在开放平台的唯一标识符，若当前小程序已绑定到微信开放平台帐号下会返回，详见 UnionID 机制说明。
    pub unionid: String,
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckEncryptedResult {
    /// 错误码
    pub errcode: i32,
    /// 错误提示信息
    pub errmsg: String,
    /// 是否是合法的数据
    pub vaild: bool,
    /// 加密数据生成的时间戳
    pub create_time: i32,
}

#[derive(Debug, Serialize)]
pub struct QueryPaidUnionId {
    /// 支付用户唯一标识
    pub openid: String,
    /// 微信支付订单号
    #[serde(default)]
    pub transaction_id: Option<String>,
    /// 微信支付分配的商户号，和商户订单号配合使用
    #[serde(default)]
    pub mch_id: Option<String>,
    /// 微信支付商户订单号，和商户号配合使用
    #[serde(default)]
    pub out_trade_no: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnionIdResult {
    /// 用户唯一标识，调用成功后返回
    pub unionid: String,
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
}

async fn get_send<'a, A: WxApiRequestBuilder, R: DeserializeOwned, P: Serialize>(
    api_builder: &'a A,
    url: &'static str,
    param: &'a P,
) -> SdkResult<R> {
    let builder = api_builder.wx_get(url).await?.query(param);
    let res = builder.send().await?.json::<R>().await?;
    Ok(res)
}

async fn post_send<'a, A: WxApiRequestBuilder, R: DeserializeOwned, D: Serialize>(
    api_builder: &'a A,
    url: &'static str,
    post_data: &'a D,
) -> SdkResult<R> {
    let builder = api_builder.wx_post(url).await?.json(post_data);
    let res = builder.send().await?.json::<CommonResponse<R>>().await?;
    res.into()
}

/// 小程序接口SDK，由于 Rust Doc 中还无法搜索中文，请直接搜索相关请求 url 中的关键信息。
pub struct WxaSdk<T: AccessTokenProvider> {
    pub(crate) sdk: crate::WxSdk<T>,
}

impl<T: AccessTokenProvider> WxaSdk<T> {
    pub async fn code_to_session(&self, js_code: String) -> SdkResult<LoginResult> {
        let url = "https://api.weixin.qq.com/sns/jscode2session?grant_type=authorization_code";
        let query = QuerySession {
            js_code: js_code,
            appid: self.sdk.app_id.clone(),
            secret: self.sdk.app_secret.clone(),
        };
        get_send(&self.sdk, url, &query).await
    }

    pub async fn check_encrypted_data(
        &self,
        encrypted_msg_hash: &str,
    ) -> SdkResult<CheckEncryptedResult> {
        let url = "https://api.weixin.qq.com/wxa/business/checkencryptedmsg";
        let post_data = &json!({ "encrypted_msg_hash": encrypted_msg_hash });
        post_send(&self.sdk, url, post_data).await
    }

    pub async fn get_paid_unionid(&self, param: &QueryPaidUnionId) -> SdkResult<UnionIdResult> {
        let url = "https://api.weixin.qq.com/wxa/getpaidunionid";
        post_send(&self.sdk, url, &param).await
    }

    /// Data analysis 数据分析模块
    pub fn data_analysis(&self) -> data_analysis::DataAnalysisModule<WxSdk<T>> {
        data_analysis::DataAnalysisModule(&self.sdk)
    }
}

// #[test]
// fn test_query_data_option() {
//     #[derive(Serialize, Deserialize, Debug)]
//     pub struct Data {
//         pub aid: String,
//         pub key: Option<String>,
//     }
//     let data = &Data { aid: "aaaa".into(), key: None };
//     let builder = reqwest::Client::new().get("https://a.b.com/").query(&data);
//     println!("{:?}", &builder); // query: Some("aid=aaaa")
//
//     let data = &Data { aid: "aaaa".into(), key: Some("".into()) };
//     let builder = reqwest::Client::new().get("https://a.b.com/").query(&data);
//     println!("{:?}", &builder); // query: Some("aid=aaaa&key=")
// }

// #[test]
// fn test_query_data_array() { // 不支持数组
//     #[derive(Serialize, Deserialize, Debug)]
//     pub struct Data {
//         pub keys: Vec<i32>,
//     }
//     let data = &Data { keys: vec![1, 2, 3, 4] };
//     let builder = reqwest::Client::new().get("https://b.com/").query(&data);
//     println!("{:?}", &builder); //
// }