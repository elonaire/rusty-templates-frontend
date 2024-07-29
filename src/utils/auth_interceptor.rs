use reqwest::{header::ACCESS_CONTROL_ALLOW_CREDENTIALS, Client};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct QueryFormatter {
    pub query: String,
    pub variables: Option<String>,
}

pub async fn retrieve_new_token(current_token: &String) -> Result<String, reqwest::Error> {
    let endpoint = option_env!("ACL_SERVICE_URL").expect("ACL_SERVICE_URL env var not set");
    let client = Client::builder().build().unwrap();

    // TODO: Implement the logic to check if the user is authenticated
    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", current_token).as_str())
        .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
        .json(&QueryFormatter {
            query: (r#"
                query Query {
                    checkAuth {
                        isAuth
                        sub
                    }
                }
            "#
            ).to_string(),
            variables: None,
        })
        // .body(r#"{"query":"query Query { checkAuth { isAuth sub } }"}, "operationName": "Query" "#)
        .send()
        .await?;

    log::info!("Response: {:?}", response);

    // TODO: Implement the logic to check if the user is authenticated
    // Extract the new user token from the response headers
    let new_token = response
        .headers()
        .get("New-Access-Token")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string())
        .unwrap_or(current_token.clone());

    Ok(new_token)
}
