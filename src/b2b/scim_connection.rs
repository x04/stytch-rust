// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::scim::SCIMConnection;
use crate::b2b::scim::SCIMConnectionWithNextToken;
use crate::b2b::scim::SCIMConnectionWithToken;
use crate::b2b::scim::SCIMGroup;
use crate::b2b::scim::SCIMGroupImplicitRoleAssignments;
use serde::{Deserialize, Serialize};

/// CreateRequest: Request type for `Connection.create`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// display_name: A human-readable display name for the connection.
    pub display_name: std::option::Option<String>,
    pub identity_provider: std::option::Option<CreateRequestIdentityProvider>,
}
/// CreateResponse: Response type for `Connection.create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// connection: The `SCIM Connection` object affected by this API call. See the
    /// [SCIM Connection Object](https://stytch.com/docs/b2b/api/scim-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<SCIMConnectionWithToken>,
}
/// DeleteRequest: Request type for `Connection.delete`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// connection_id: The ID of the SCIM connection.
    pub connection_id: String,
}
/// DeleteResponse: Response type for `Connection.delete`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// connection_id: The `connection_id` that was deleted as part of the delete request.
    pub connection_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// GetGroupsRequest: Request type for `Connection.get_groups`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetGroupsRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// connection_id: The ID of the SCIM connection.
    pub connection_id: String,
    /// cursor: The `cursor` field allows you to paginate through your results. Each result array is limited to
    /// 1000 results. If your query returns more than 1000 results, you will need to paginate the responses
    /// using the `cursor`. If you receive a response that includes a non-null `next_cursor` in the
    /// `results_metadata` object, repeat the search call with the `next_cursor` value set to the `cursor` field
    /// to retrieve the next page of results. Continue to make search calls until the `next_cursor` in the
    /// response is null.
    pub cursor: std::option::Option<String>,
    /// limit: The number of search results to return per page. The default limit is 100. A maximum of 1000
    /// results can be returned by a single search request. If the total size of your result set is greater than
    /// one page size, you must paginate the response. See the `cursor` field.
    pub limit: std::option::Option<i32>,
}
/// GetGroupsResponse: Response type for `Connection.get_groups`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetGroupsResponse {
    /// scim_groups: A list of SCIM Connection Groups belonging to the connection.
    pub scim_groups: std::vec::Vec<SCIMGroup>,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// next_cursor: The `next_cursor` string is returned when your search result contains more than one page of
    /// results. This value is passed into your next search call in the `cursor` field.
    pub next_cursor: std::option::Option<String>,
}
/// GetRequest: Request type for `Connection.get`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
}
/// GetResponse: Response type for `Connection.get`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub connection: std::option::Option<SCIMConnection>,
}
/// RotateCancelRequest: Request type for `Connection.rotate_cancel`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RotateCancelRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// connection_id: The ID of the SCIM connection.
    pub connection_id: String,
}
/// RotateCancelResponse: Response type for `Connection.rotate_cancel`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateCancelResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// connection: The `SCIM Connection` object affected by this API call. See the
    /// [SCIM Connection Object](https://stytch.com/docs/b2b/api/scim-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<SCIMConnection>,
}
/// RotateCompleteRequest: Request type for `Connection.rotate_complete`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RotateCompleteRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// connection_id: The ID of the SCIM connection.
    pub connection_id: String,
}
/// RotateCompleteResponse: Response type for `Connection.rotate_complete`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateCompleteResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// connection: The `SCIM Connection` object affected by this API call. See the
    /// [SCIM Connection Object](https://stytch.com/docs/b2b/api/scim-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<SCIMConnection>,
}
/// RotateStartRequest: Request type for `Connection.rotate_start`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RotateStartRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// connection_id: The ID of the SCIM connection.
    pub connection_id: String,
}
/// RotateStartResponse: Response type for `Connection.rotate_start`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateStartResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// connection: The `SCIM Connection` object affected by this API call. See the
    /// [SCIM Connection Object](https://stytch.com/docs/b2b/api/scim-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<SCIMConnectionWithNextToken>,
}
/// UpdateRequest: Request type for `Connection.update`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// connection_id: The ID of the SCIM connection.
    pub connection_id: String,
    /// display_name: A human-readable display name for the connection.
    pub display_name: std::option::Option<String>,
    pub identity_provider: std::option::Option<UpdateRequestIdentityProvider>,
    /// scim_group_implicit_role_assignments: An array of SCIM group implicit role assignments. Each object in
    /// the array must contain a `group_id` and a `role_id`.
    pub scim_group_implicit_role_assignments:
        std::option::Option<std::vec::Vec<SCIMGroupImplicitRoleAssignments>>,
}
/// UpdateResponse: Response type for `Connection.update`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// connection: The `SAML Connection` object affected by this API call. See the
    /// [SAML Connection Object](https://stytch.com/docs/b2b/api/saml-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<SCIMConnection>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum CreateRequestIdentityProvider {
    #[serde(rename = "generic")]
    #[default]
    Generic,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "microsoftentra")]
    Microsoftentra,
    #[serde(rename = "cyberark")]
    Cyberark,
    #[serde(rename = "jumpcloud")]
    Jumpcloud,
    #[serde(rename = "onelogin")]
    Onelogin,
    #[serde(rename = "pingfederate")]
    Pingfederate,
    #[serde(rename = "rippling")]
    Rippling,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum UpdateRequestIdentityProvider {
    #[serde(rename = "generic")]
    #[default]
    Generic,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "microsoftentra")]
    Microsoftentra,
    #[serde(rename = "cyberark")]
    Cyberark,
    #[serde(rename = "jumpcloud")]
    Jumpcloud,
    #[serde(rename = "onelogin")]
    Onelogin,
    #[serde(rename = "pingfederate")]
    Pingfederate,
    #[serde(rename = "rippling")]
    Rippling,
}

pub struct Connection {
    http_client: crate::client::Client,
}

impl Connection {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn update(&self, body: UpdateRequest) -> crate::Result<UpdateResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path = format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
    pub async fn delete(&self, body: DeleteRequest) -> crate::Result<DeleteResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path = format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn rotate_start(
        &self,
        body: RotateStartRequest,
    ) -> crate::Result<RotateStartResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path =
            format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}/rotate/start");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn rotate_complete(
        &self,
        body: RotateCompleteRequest,
    ) -> crate::Result<RotateCompleteResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path =
            format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}/rotate/complete");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn rotate_cancel(
        &self,
        body: RotateCancelRequest,
    ) -> crate::Result<RotateCancelResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path =
            format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}/rotate/cancel");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn get_groups(&self, body: GetGroupsRequest) -> crate::Result<GetGroupsResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path = format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
    pub async fn create(&self, body: CreateRequest) -> crate::Result<CreateResponse> {
        let organization_id = &body.organization_id;
        let path = format!("/v1/b2b/scim/{organization_id}/connection");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn get(&self, body: GetRequest) -> crate::Result<GetResponse> {
        let organization_id = &body.organization_id;
        let path = format!("/v1/b2b/scim/{organization_id}/connection");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
}
