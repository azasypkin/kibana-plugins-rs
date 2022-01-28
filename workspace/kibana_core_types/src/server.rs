mod core;
mod elasticsearch;
mod http;
mod logging;
pub mod packages;
mod plugins;

pub use self::{
    core::CoreSetup,
    elasticsearch::{
        AuthenticationInfo, ElasticsearchClient, ElasticsearchClientSecurity,
        ScopedElasticsearchClient,
    },
    http::{
        BasePath, HttpSetup, Request, RequestContext, RequestHandler, Response, ResponseFactory,
        ResponseOptions, RouteConfig, RouteSchema, Router,
    },
    logging::{Logger, LoggerFactory},
    plugins::PluginInitializerContext,
};
