
use log::info;
use reqwest::{Client, Url};
use crate::auth::service_account::ServiceAccountCredentials;

use super::ServiceBase;

// curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:streamGenerateContent?alt=sse&key=${GOOGLE_API_KEY}" 

static GEMINI_SERVICE_SCOPE: &str = "https://generativelanguage.googleapis.com/v1beta";
// static GET_ROUTE_URL: &str = "https://routes.googleapis.com/directions/v2:computeRoutes";

#[derive(Debug, Clone)]
pub struct GeminiService {
    base: ServiceBase
}

impl GeminiService {
    /// Create `GeminiService` Authenticate by using API keys.
    ///
    /// * `api_key` -  API key to use to authenticate to Google Cloud APIs and services that support API keys.
    pub fn new_with_api_key(api_key: String) -> Self {
        return Self { base: ServiceBase::new_with_api_key(api_key) }
    }

    /// Create `GeminiService` Authenticate by Service Credentials.
    ///
    /// * `service_account_credentials` -  `ServiceAccountCredentials` to use to authenticate to Google Cloud APIs.
    pub fn new_with_credentials(service_account_credentials: ServiceAccountCredentials) -> Self {
        return Self { base: ServiceBase::new_with_credentials(service_account_credentials, vec![GEMINI_SERVICE_SCOPE]) }
    }

    /// Sends message to API.
    /// See https://cloud.google.com/translate/docs/basic/discovering-supported-languages
    ///
    // pub async fn message(&mut self, message: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    pub async fn message(&mut self, message: &str, model: &str) -> Result<(), Box<dyn std::error::Error>> {

        let base_url = Url::parse(&format!("{}", GEMINI_SERVICE_SCOPE ))?;
        let headers = self.base.create_headers().await?;
        let request_query = format!("/{}/{}", model, message );

        let builder = Client::new().get(base_url)
                .query(&request_query)
                .headers(headers);

        let body = self.base.make_request(builder).await?;


        // Deserialisieren Sie die Antwort von JSON zu einem String
        serde_json::from_str::<String>(&body)?;
        Ok(())

    }
    

    /// Returns token_count
    pub async fn tokens(&mut self, model: &str) -> Result<(), Box<dyn std::error::Error>> {

        // curl https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:countTokens?key=$GOOGLE_API_KEY \
        // -H 'Content-Type: application/json' \
        // -X POST \
        // -d '{
        // "contents": [{
        //     "parts":[{
        //     "text": "Write a story about a magic backpack."}]}]}' > response.json
        info!("model@tokens: {}", serde_json::to_string(&model)?);

        let base_url = Url::parse(&format!("{}", GEMINI_SERVICE_SCOPE ))?;
        let request_query = format!("/{}:countTokens", model );
        info!("request_query: {}", request_query);
        let headers = self.base.create_headers().await?;


        let builder = Client::new().get(base_url)
                .query(&request_query)
                .headers(headers);

        // println!("base_url: {}", &base_url);

        let body = self.base.make_request(builder).await?;


        // Deserialisieren Sie die Antwort von JSON zu einem String
        serde_json::from_str::<String>(&body)?;
        Ok(())
    }

}
