use crate::handlers;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::upload_file::upload_files,
        crate::handlers::compress_file::compress_all_files,
        crate::handlers::check::check_status,
    ),
    components(
        schemas(
            crate::handlers::upload_file::UploadResponse,
            crate::handlers::compress_file::CompressionResponse,
            crate::handlers::check::StatusResponse,
            crate::handlers::check::ErrorResponse
        )
    ),
    tags(
        (name = "file-service", description = "File upload and compression service")
    )
)]
pub struct ApiDoc;
