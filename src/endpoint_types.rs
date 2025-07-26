#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateSpeechResponse {
    ApplicationOctetStream(Vec<u8>),
    TextEventStream(crate::types::CreateSpeechResponseStreamEvent),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateTranscriptionResponse {
    ApplicationJson(crate::types::CreateTranscriptionResponseJson),
    ApplicationJsonVerbose(crate::types::CreateTranscriptionResponseVerboseJson),
    TextEventStream(crate::types::CreateTranscriptionResponseStreamEvent),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranslationResponse {
    CreateTranslationResponseJson(CreateTranslationResponseJson),
    CreateTranslationResponseVerboseJson(CreateTranslationResponseVerboseJson),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateBatchRequestBodyEndpoint {
    #[serde(rename = "/v1/responses")]
    V1Responses,
    #[serde(rename = "/v1/chat/completions")]
    V1ChatCompletions,
    #[serde(rename = "/v1/embeddings")]
    V1Embeddings,
    #[serde(rename = "/v1/completions")]
    V1Completions,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateBatchRequestBodyCompletionWindow {
    #[serde(rename = "24h")]
    Size24h,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBatchRequestBody {
    /** The ID of an uploaded file that contains requests for the new batch.

    See [upload file](/docs/api-reference/files/create) for how to upload a file.

    Your input file must be formatted as a [JSONL file](/docs/api-reference/batch/request-input), and must be uploaded with the purpose `batch`. The file can contain up to 50,000 requests, and can be up to 200 MB in size. */
    pub input_file_id: String,
    /** The endpoint to be used for all requests in the batch. Currently `/v1/responses`, `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are supported. Note that `/v1/embeddings` batches are also restricted to a maximum of 50,000 embedding inputs across all requests in the batch. */
    pub endpoint: CreateBatchRequestBodyEndpoint,
    /** The time frame within which the batch should be processed. Currently only `24h` is supported. */
    pub completion_window: CreateBatchRequestBodyCompletionWindow,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateChatCompletionResponse {
    ApplicationJson(crate::types::CreateChatCompletionResponse),
    TextEventStream(crate::types::CreateChatCompletionStreamResponse),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateChatCompletionRequestBody {
    pub metadata: Metadata,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateEvalRequestBody {
    /** Rename the evaluation. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteEvalResponse {
    pub object: String,
    pub deleted: bool,
    pub eval_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteEvalRunResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPaginatedFineTuningJobsQuery(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminApiKeysCreateRequestBody {
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminApiKeysDeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAuditLogsQuery {
    /** Return only events whose `effective_at` (Unix seconds) is greater than this value. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<u64>,
    /** Return only events whose `effective_at` (Unix seconds) is greater than or equal to this value. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<u64>,
    /** Return only events whose `effective_at` (Unix seconds) is less than this value. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<u64>,
    /** Return only events whose `effective_at` (Unix seconds) is less than or equal to this value. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateResponseResponse {
    ApplicationJson(crate::types::Response),
    TextEventStream(crate::types::ResponseStreamEvent),
}
