use std::collections::HashMap;

use gql_client::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub enum GraphQLResponse<T> {
    Data(T),
    Error(String),
}

impl<T> GraphQLResponse<T> {
    pub fn get_data(&self) -> Option<&T> {
        match self {
            GraphQLResponse::Data(data) => Some(data),
            _ => None,
        }
    }

    // TODO: Implement get_error
    pub fn get_error(&self) -> String {
        match self {
            GraphQLResponse::Error(error) => self.extract_error_message(error),
            _ => "".into(),
        }
    }

    fn extract_error_message(&self, error_string: &str) -> String {
        // Look for the position of "Message: "
        if let Some(start) = error_string.find("Message: ") {
            // Cut the string to remove everything before "Message: "
            let message_start = start + "Message: ".len();
            // Return the substring starting after "Message: "
            return error_string[message_start..].trim().to_string();
        }
        // If "Message: " is not found, return None
        "Undefined Error".to_string()
    }
}

pub async fn perform_query_without_vars<R: for<'de> Deserialize<'de>>(
    headers: Option<HashMap<String, String>>,
    endpoint: &str,
    query: &str,
) -> GraphQLResponse<R> {
    // let endpoint = "http://localhost:3001";

    // create query
    //     let query = r#"
    //        query Query {
    //            getUsers {
    //                id
    //                email
    //                fullName
    //                age
    //            }
    //        }
    //    "#;

    let client = match headers {
        Some(headers) => Client::new_with_headers(endpoint, headers),
        None => Client::new(endpoint)
    };

    let response = client.query::<R>(query).await;

    match response {
        Ok(data) => GraphQLResponse::Data(data.unwrap()),
        Err(err) => GraphQLResponse::Error(format!("{:?}", err)),
    }
}


pub async fn perform_mutation_or_query_with_vars<R: for<'de> Deserialize<'de> + Serialize, T: for<'de> Deserialize<'de> + Serialize>(
    headers: Option<HashMap<String, String>>,
    endpoint: &str,
    query: &str,
    vars: T,
) -> GraphQLResponse<R> {
    // let endpoint = "http://localhost:3001";

    // create query
    // let query = r#"
    //     mutation Mutation($user: UserInput!) {
    //         signUp(user: $user) {
    //             id
    //             email
    //             fullName
    //             age
    //         }
    //     }
    // "#;

    let client = match headers {
        Some(headers) => Client::new_with_headers(endpoint, headers),
        None => Client::new(endpoint)
    };

    let response = client.query_with_vars::<R, T>(query, vars).await;

    match response {
        Ok(data) => GraphQLResponse::Data(data.unwrap()),
        Err(err) => GraphQLResponse::Error(format!("{:?}", err)),
    }
}
