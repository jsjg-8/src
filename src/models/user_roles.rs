/*
 * Kill Bill
 *
 * Kill Bill is an open-source billing and payments platform
 *
 * The version of the OpenAPI document: 0.24.10
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRoles {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
}

impl UserRoles {
    pub fn new(username: String, password: String, roles: Vec<String>) -> UserRoles {
        UserRoles {
            username,
            password,
            roles,
        }
    }
}

