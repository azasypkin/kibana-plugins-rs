mod elasticsearch_client;
mod scoped_elasticsearch_client;

pub use self::{
    elasticsearch_client::{AuthenticationInfo, ElasticsearchClient, ElasticsearchClientSecurity},
    scoped_elasticsearch_client::ScopedElasticsearchClient,
};
