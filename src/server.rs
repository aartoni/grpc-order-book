use std::pin::Pin;

use futures_util::Stream;
use proto::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use tonic::{transport::Server, Status};

pub mod proto {
    tonic::include_proto!("orderbook");
}

#[derive(Default)]
pub struct OrderBookServer {}

#[tonic::async_trait]
impl OrderbookAggregator for OrderBookServer {
    type BookSummaryStream = Pin<Box<dyn Stream<Item = Result<proto::Summary, Status>> + Send + 'static>>;

    async fn book_summary(&self,request:tonic::Request<proto::Empty>,)->Result<tonic::Response<Self::BookSummaryStream>,tonic::Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = OrderBookServer::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(OrderbookAggregatorServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
