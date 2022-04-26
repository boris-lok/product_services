mod product;

mod pb {
    include!("../gen/grpc.product.rs");
}

#[tokio::main]
async fn main() {}
