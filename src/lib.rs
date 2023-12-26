use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender, SetAddRequest};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct HelloActor {}

/// Implementation of the HttpServer capability contract
#[async_trait]
impl HttpServer for HelloActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let name = match req.query_string.split("=").collect::<Vec<&str>>()[..] {
            ["name", name] => {
                KeyValueSender::new()
                    .set_add(
                        ctx,
                        &SetAddRequest {
                            set_name: "names".to_string(),
                            value: name.to_string(),
                        },
                    )
                    .await?;
                name.to_string()
            }
            ["names"] => KeyValueSender::new()
                .set_query(ctx, "names")
                .await?
                .join(", "),
            _ => "World".to_string(),
        };

        Ok(HttpResponse::ok(format!("Hello, {}!", name,)))
    }
}

