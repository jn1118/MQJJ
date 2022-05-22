use util::{
    result::{JasmineError, JasmineResult},
    transaction::JasmineMessage,
};
///A trait representing a JasmineSubscriber interface.
pub trait JasmineSubscriber {
    ///A function takes in a topic and subscribe the topic.
    fn subscribe(topic: String) -> JasmineResult<()>;
    ///A function takes in a topic and unsubscribe the topic.
    fn unsubscribe(topic: String) -> JasmineResult<()>;
}