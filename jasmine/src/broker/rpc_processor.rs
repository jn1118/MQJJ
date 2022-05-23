use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::async_trait;
use tonic::codegen::http::Request;
use tonic::transport::Channel;
use tonic::{Response, Status};
use util::result::{JasmineError, JasmineResult};
use util::rpc::broker::jasmine_broker_server::{JasmineBroker, JasmineBrokerServer};
use util::rpc::broker::{
    ConnectRequest, Empty, PublishRequest, PublishResponse, SubscribeRequest, SubscribeResponse,
};

use util::rpc::client::jasmine_client_client::JasmineClientClient;

struct Broker {
    pub subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    pub client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
    pub message_queue: Arc<Mutex<Vec<(String, String)>>>,
}

#[tonic::async_trait]
impl JasmineBroker for Broker {
    async fn hook(
        &self,
        request: tonic::Request<ConnectRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_client_map = self.client_map.lock().await;
        let address = request.into_inner().address;
        match JasmineClientClient::connect(format!("http://{}", &address)).await {
            Ok(client) => {
                (*temp_client_map).insert(address, client);
                return Ok(Response::new(Empty {}));
            }
            Err(error) => {
                return Err(Status::unknown("error"));
            }
        }
    }

    async fn unhook(
        &self,
        request: tonic::Request<ConnectRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_client_map = self.client_map.lock().await;
        let address = request.into_inner().address;

        (*temp_client_map).remove(&address);

        return Ok(Response::new(Empty {}));
    }

    async fn publish(
        &self,
        request: tonic::Request<PublishRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_message_queue = self.message_queue.lock().await;
        let temp_request = request.into_inner().clone();
        let topic = temp_request.topic;
        let message = temp_request.message;
        (*temp_message_queue).push((topic, message));
        drop(temp_message_queue);
        return Ok(Response::new(Empty {}));
    }

    async fn subscribe(
        &self,
        request: tonic::Request<SubscribeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_subscriber_map = self.subscriber_map.lock().await;
        let temp_request = request.into_inner().clone();
        let address = temp_request.address;
        let topic = temp_request.topic;

        match (*temp_subscriber_map).get_mut(&topic) {
            Some(set) => {
                set.insert(address);
                return Ok(Response::new(Empty {}));
            }
            None => {
                let mut set = HashSet::new();
                set.insert(address);
                (*temp_subscriber_map).insert(topic, set);
                return Ok(Response::new(Empty {}));
            }
        }
    }

    async fn unsubscribe(
        &self,
        request: tonic::Request<SubscribeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_subscriber_map = self.subscriber_map.lock().await;
        let temp_request = request.into_inner().clone();
        let address = temp_request.address;
        let topic = temp_request.topic;

        match (*temp_subscriber_map).get_mut(&topic) {
            Some(set) => {
                set.remove(&address);
            }
            None => {}
        }

        return Ok(Response::new(Empty {}));
    }

    async fn ping(&self, request: tonic::Request<Empty>) -> Result<Response<Empty>, Status> {
        return Ok(Response::new(Empty {}));
    }
}