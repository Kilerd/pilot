use serde::{Deserialize, Serialize, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ApiResult<T> {
    pub ok: bool,
    pub error_code: Option<i32>,
    pub description: Option<String>,
    pub result: Option<T>,
}

impl<T> ApiResult<T> {

    pub fn into_result(self) -> BotResult<T> {
        if self.ok {
            Ok(self.result.unwrap())
        }else {
            Err(ApiError {
                error_code: self.error_code.unwrap(),
                description: self.description.unwrap()
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiError {
    pub error_code: i32,
    pub description: String,
}


pub type BotResult<T> = Result<T, ApiError>;



#[cfg(test)]
mod test {
    use crate::error::{BotResult, ApiError, ApiResult};

    #[test]
    fn serde_api_result_is_ok() {
        let response = r#"{
          "ok": true,
          "result": true,
          "description": "Webhook was deleted"
        }"#;
        let result= serde_json::from_str::<ApiResult<bool>>(response);
        let bot_result = result.unwrap().into_result();
        debug_assert_eq!(bot_result, Ok(true));
    }

    #[test]
    fn serde_api_result_is_ok_2() {
        let response = r#"{
          "ok": false,
          "error_code": 401,
          "description": "Unauthorized"
        }"#;

        let result= serde_json::from_str::<ApiResult<bool>>(response);
        let bot_result = result.unwrap().into_result();
        debug_assert_eq!(bot_result, Err(ApiError {
            error_code: 401,
            description: "Unauthorized".into()
        }));
    }
}