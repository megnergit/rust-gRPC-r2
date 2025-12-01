//pub mod hello {
//   tonic::include_proto!("../proto-defs/src/generated/hello");
//}
// pub mod hello {
// //    include!("../../proto-defs/src/generated/hello.rs");
//       include!(concat!(env!("OUT_DIR"), "/hello.rs"));
// }

// pub mod hello {
//     tonic::include_proto!("hello");
// }



use tonic::{transport::Server, Request, Response, Status};
use proto_defs::hello::greeter_server::{Greeter, GreeterServer};
use proto_defs::hello::{HelloReply, HelloRequest};



#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self, 
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        let reply = HelloReply {
            message: format!("Hello {}!", name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("gRPC Server listening on {}", addr);

    Server::builder()
    .add_service(GreeterServer::new(greeter))
    .serve(addr)
    .await?;

    Ok(())
}
