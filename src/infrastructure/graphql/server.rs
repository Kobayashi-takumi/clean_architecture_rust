use super::resolver::{mutation::Mutation, query::Query};
use crate::domain::service::ServiceFactory;
use crate::infrastructure::database::{pool, transaction::PgTransaction};
use crate::infrastructure::{
    database::{query::DatabaseQueryFactory, repository::DatabaseRepositoryFactory},
    memory::{query::MemoryQueryFactory, repository::MemoryRepositoryFactory},
};
use crate::interface_adapter::adapter::PersistenceAdapterFactory;
use crate::shared::{
    config::Config,
    error::{Error, Result},
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    body::Body,
    extract::Extension,
    http::{header::HeaderMap, Request},
    middleware::{from_fn, Next},
    response::{Html, IntoResponse, Response},
    routing::get,
    serve, Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;

type ISchema = Schema<Query, Mutation, EmptySubscription>;

async fn health_check() -> &'static str {
    "Healthy!"
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

pub async fn start(config: &Config) -> Result<()> {
    let pool = pool(config).await?;
    log::info!("{pool:?}");
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health-check", get(health_check))
        .layer(from_fn(di_middleware))
        .layer(Extension(schema))
        .layer(Extension(config.clone()))
        .layer(Extension(pool.clone()));
    let port = "8000";
    let listener = TcpListener::bind(format!("0.0.0.0:{port}").as_str())
        .await
        .map_err(|e| {
            log::error!("{}", e);
            Error::Unknown
        })?;
    log::info!("Server running on port {port}");
    serve(listener, app.into_make_service())
        .await
        .map_err(|e| {
            log::error!("{}", e);
            Error::Unknown
        })?;
    Ok(())
}

async fn graphql_handler(
    schema: Extension<ISchema>,
    Extension(service_factory): Extension<ServiceFactory>, // ExtensionからConfigを取得
    _headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(service_factory))
        .await
        .into()
}

async fn di_middleware(
    Extension(_config): Extension<Config>,
    Extension(pool): Extension<PgPool>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let tx = match PgTransaction::begin(&pool).await {
        Ok(val) => val,
        Err(err) => return err.into_response(),
    };
    // let repository_factory = MemoryRepositoryFactory::new();
    let repository_factory = DatabaseRepositoryFactory::new(tx.get());
    // let query_factory = MemoryQueryFactory::new();
    let query_factory = DatabaseQueryFactory::new(&pool);
    let adapter_factory = PersistenceAdapterFactory::new(
        repository_factory.task_repository(),
        query_factory.task_query(),
    );
    let usecase_factory = ServiceFactory::new(
        adapter_factory.task_persistence_adapter(),
        adapter_factory.task_persistence_adapter(),
        adapter_factory.task_persistence_adapter(),
        adapter_factory.task_persistence_adapter(),
    );
    req.extensions_mut().insert(usecase_factory.clone());
    let response = next.run(req).await;
    drop(usecase_factory);
    drop(adapter_factory);
    drop(query_factory);
    drop(repository_factory);
    if response.status().is_success() {
        if let Err(e) = tx.commit().await {
            return e.into_response();
        }
    } else if let Err(e) = tx.rollback().await {
        return e.into_response();
    }
    response
}
