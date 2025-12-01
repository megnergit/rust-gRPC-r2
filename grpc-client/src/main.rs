// pub mod hello {
//   tonic::include_proto!("../proto-defs/src/generated/hello");
//}

//    let out_dir = std::env::var("OUT_DIR")?;
    
// pub mod hello {
//     include!(concat!(env!("OUT_DIR"), "/hello.rs"));
// }


use tonic::Request;
use proto_defs::hello::greeter_client::GreeterClient;
use proto_defs::hello::HelloRequest;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let response = client
    .say_hello(Request::new(HelloRequest {
        name: "Mia".into(),
    }))
    .await?;

    println!("Response = {:?}", response);

    Ok(())
}