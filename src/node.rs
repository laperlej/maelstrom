use crate::message::{Req, Res, MessageType, NodeId};
use crate::generate::Generator;
use crate::broadcast::Broadcaster;
use std::collections::HashMap;

pub struct Node {
    pub id: NodeId,
    pub peers: Vec<NodeId>,
    pub handlers: HashMap<MessageType, fn(&mut Node, &Req, &mut Res)>,
    pub generator: Generator,
    pub broadcaster: Broadcaster,
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: String::new(),
            peers: Vec::new(),
            handlers: HashMap::new(),
            generator: Generator::new(),
            broadcaster: Broadcaster::new(),
        }
    }

    pub fn handle_fn(&mut self, msg_type: MessageType, handler: fn(&mut Node, &Req, &mut Res)) {
        self.handlers.insert(msg_type, handler);
    }
    
    pub fn handle_msg(&mut self, req: &Req) {
        match self.handlers.get(&req.body.r#type) {
            Some(handler) => {
                let mut res = Res::new(self.id.clone(), req.src.clone());
                handler(self, req, &mut res)
            }
            None => panic!("No handler for message type {:?}", req.body.r#type),
        }
    }
}


