use crate::ids::{BlockId, DatabaseId};
use crate::models::error::ErrorResponse;
use crate::models::search::{DatabaseQuery, SearchRequest};
use crate::models::{Database, ListResponse, Object, Page};
use ids::{AsIdentifier, PageId};
use models::block::Block;
use models::comment::Comment;
use models::search::NotionSearch;
use models::users::User;
use models::PageCreateRequest;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client, ClientBuilder, RequestBuilder};
use tracing::Instrument;

pub mod ids;
pub mod models;

pub use chrono;

const NOTION_API_VERSION: &str = "2022-02-22";

/// An wrapper Error type for all errors produced by the [`NotionApi`](NotionApi) client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid Notion API Token: {}", source)]
    InvalidApiToken { source: header::InvalidHeaderValue },

    #[error("Unable to build reqwest HTTP client: {}", source)]
    ErrorBuildingClient { source: reqwest::Error },

    #[error("Error sending HTTP request: {}", source)]
    RequestFailed {
        #[from]
        source: reqwest::Error,
    },

    #[error("Error reading response: {}", source)]
    ResponseIoError { source: reqwest::Error },

    #[error("Error parsing json response: {}", source)]
    JsonParseError { source: serde_json::Error },

    #[error("Unexpected API Response")]
    UnexpectedResponse { response: Object },

    #[error("API Error {}({}): {}", .error.code, .error.status, .error.message)]
    ApiError { error: ErrorResponse },
}

/// An API client for Notion.
/// Create a client by using [new(api_token: String)](Self::new()).
#[derive(Clone)]
pub struct NotionApi {
    client: Client,
}

impl NotionApi {
    /// Creates an instance of NotionApi.
    /// May fail if the provided api_token is an improper value.
    pub fn new(api_token: String) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Notion-Version",
            HeaderValue::from_static(NOTION_API_VERSION),
        );

        let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", api_token))
            .map_err(|source| Error::InvalidApiToken { source })?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|source| Error::ErrorBuildingClient { source })?;

        Ok(Self { client })
    }

    async fn make_json_request(
        &self,
        request: RequestBuilder,
    ) -> Result<Object, Error> {
        let request = request.build()?;
        let url = request.url();
        tracing::trace!(
            method = request.method().as_str(),
            url = url.as_str(),
            "Sending request"
        );
        let json = self
            .client
            .execute(request)
            .instrument(tracing::trace_span!("Sending request"))
            .await
            .map_err(|source| Error::RequestFailed { source })?
            .text()
            .instrument(tracing::trace_span!("Reading response"))
            .await
            .map_err(|source| Error::ResponseIoError { source })?;

        tracing::debug!("JSON Response: {}", json);
        #[cfg(test)]
        {
            dbg!(serde_json::from_str::<serde_json::Value>(&json)
                .map_err(|source| Error::JsonParseError { source })?);
        }
        let result =
            serde_json::from_str(&json).map_err(|source| Error::JsonParseError { source })?;

        match result {
            Object::Error { error } => Err(Error::ApiError { error }),
            response => Ok(response),
        }
    }

    async fn make_json_request_user(
        &self,
        request: RequestBuilder,
    ) -> Result<User, Error> {
        let request = request.build()?;
        let url = request.url();
        tracing::trace!(
            method = request.method().as_str(),
            url = url.as_str(),
            "Sending request"
        );
        let json = self
            .client
            .execute(request)
            .instrument(tracing::trace_span!("Sending request"))
            .await
            .map_err(|source| Error::RequestFailed { source })?
            .text()
            .instrument(tracing::trace_span!("Reading response"))
            .await
            .map_err(|source| Error::ResponseIoError { source })?;

        tracing::debug!("JSON Response: {}", json);
        #[cfg(test)]
        {
            dbg!(serde_json::from_str::<serde_json::Value>(&json)
                .map_err(|source| Error::JsonParseError { source })?);
        }
        let result = serde_json::from_str::<User>(&json)
            .map_err(|source| Error::JsonParseError { source })?;

        Ok(result)
    }

    async fn make_json_request_comments(
        &self,
        request: RequestBuilder,
    ) -> Result<ListResponse<Comment>, Error> {
        let request = request.build()?;
        let url = request.url();
        tracing::trace!(
            method = request.method().as_str(),
            url = url.as_str(),
            "Sending request"
        );
        let json = self
            .client
            .execute(request)
            .instrument(tracing::trace_span!("Sending request"))
            .await
            .map_err(|source| Error::RequestFailed { source })?
            .text()
            .instrument(tracing::trace_span!("Reading response"))
            .await
            .map_err(|source| Error::ResponseIoError { source })?;

        tracing::debug!("JSON Response: {}", json);
        #[cfg(test)]
        {
            dbg!(serde_json::from_str::<serde_json::Value>(&json)
                .map_err(|source| Error::JsonParseError { source })?);
        }
        let result = serde_json::from_str::<ListResponse<Comment>>(&json)
            .map_err(|source| Error::JsonParseError { source })?;

        Ok(result)
    }

    /// List all the databases shared with the supplied integration token.
    /// Because of the deprecation of the original endpoint this just calls
    /// [search()](Self::search()) with a filter on databases
    #[deprecated(
        note = "This method is deprecated. Please use `search()` with a filter on databases instead."
    )]
    pub async fn list_databases(&self) -> Result<ListResponse<Database>, Error> {
        self.search(NotionSearch::filter_by_databases())
            .await
            .map(|response| response.only_databases())
    }

    /// Search all pages in notion.
    /// `query` can either be a [SearchRequest] or a slightly more convenient
    /// [NotionSearch](models::search::NotionSearch) query.
    pub async fn search<T: Into<SearchRequest>>(
        &self,
        query: T,
    ) -> Result<ListResponse<Object>, Error> {
        let result = self
            .make_json_request(
                self.client
                    .post("https://api.notion.com/v1/search")
                    .json(&query.into()),
            )
            .await?;

        match result {
            Object::List { list } => Ok(list),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    /// Get a database by [DatabaseId].
    pub async fn get_database<T: AsIdentifier<DatabaseId>>(
        &self,
        database_id: T,
    ) -> Result<Database, Error> {
        let result = self
            .make_json_request(self.client.get(format!(
                "https://api.notion.com/v1/databases/{}",
                database_id.as_id()
            )))
            .await?;

        match result {
            Object::Database { database } => Ok(database),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    /// Get a page by [PageId].
    pub async fn get_page<T: AsIdentifier<PageId>>(
        &self,
        page_id: T,
    ) -> Result<Page, Error> {
        let result = self
            .make_json_request(self.client.get(format!(
                "https://api.notion.com/v1/pages/{}",
                page_id.as_id()
            )))
            .await?;

        match result {
            Object::Page { page } => Ok(page),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    /// Creates a new page and return the created page
    pub async fn create_page<T: Into<PageCreateRequest>>(
        &self,
        page: T,
    ) -> Result<Page, Error> {
        let result = self
            .make_json_request(
                self.client
                    .post("https://api.notion.com/v1/pages")
                    .json(&page.into()),
            )
            .await?;

        match result {
            Object::Page { page } => Ok(page),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    /// Query a database and return the matching pages.
    pub async fn query_database<D, T>(
        &self,
        database: D,
        query: T,
    ) -> Result<ListResponse<Page>, Error>
    where
        T: Into<DatabaseQuery>,
        D: AsIdentifier<DatabaseId>,
    {
        let result = self
            .make_json_request(
                self.client
                    .post(&format!(
                        "https://api.notion.com/v1/databases/{database_id}/query",
                        database_id = database.as_id()
                    ))
                    .json(&query.into()),
            )
            .await?;
        match result {
            Object::List { list } => Ok(list.expect_pages()?),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    pub async fn get_block_children<T: AsIdentifier<BlockId>>(
        &self,
        block_id: T,
    ) -> Result<ListResponse<Block>, Error> {
        let result = self
            .make_json_request(self.client.get(&format!(
                "https://api.notion.com/v1/blocks/{block_id}/children",
                block_id = block_id.as_id()
            )))
            .await?;

        match result {
            Object::List { list } => Ok(list.expect_blocks()?),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    pub async fn get_comments<T: AsIdentifier<BlockId>>(
        &self,
        block_id: T,
    ) -> Result<ListResponse<Comment>, Error> {
        let result = self
            .make_json_request_comments(self.client.get(&format!(
                "https://api.notion.com/v1/comments?block_id={block_id}",
                block_id = block_id.as_id()
            )))
            .await?;

        Ok(result)
    }

    pub async fn get_user(
        &self,
        user_id: &str,
    ) -> Result<User, Error> {
        tracing::info!("Notion Client Get User");

        let result = self
            .make_json_request_user(
                self.client
                    .get(&format!("https://api.notion.com/v1/users/{user_id}")),
            )
            .await;

        tracing::info!("Notion Client User: {:#?}", result);

        Ok(result?)
    }
}
