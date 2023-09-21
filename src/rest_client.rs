
pub trait RequestType {
    fn get_url(&self) -> String;
    fn get_query(&self) -> Vec<(&String, &String)>;
}

pub struct RestClient {
    api_url: String,
    auth_key: String,
    client: reqwest::Client,
}

impl RestClient {
    pub fn new(api_url: impl Into<String>, auth_key: impl Into<String>, timeout: Option<core::time::Duration>) -> Result<Self, String> {

        let mut client = reqwest::ClientBuilder::new();

        if let Some(timeout) = timeout {
            client = client.timeout(timeout);
        }

        Ok(RestClient {
            api_url: api_url.into(),
            auth_key: auth_key.into(),
            client: client.build().unwrap(),
        })
    }

    pub(crate) async fn send_request<ResponseType>(&self, request: &impl RequestType) -> Result<ResponseType, reqwest::Error>
        where
            ResponseType: serde::de::DeserializeOwned,
    {
        let uri = request.get_url();
        let query_params = request.get_query();
        let res = self
            .client
            .get(format!("{}{}", self.api_url, uri))
            .bearer_auth(&self.auth_key)
            .query(&query_params)
            .send()
            .await;

        match res {
            Ok(res) => {
                if res.status() == 200 {
                    // let text = &res.text().await.unwrap();
                    // Ok(text.to_string())
                    res.json::<ResponseType>().await
                } else {
                    Err(res.error_for_status().err().unwrap())
                }
            }
            Err(e) => Err(e),
        }
    }
}