use crate::message::{Res, Req, MessageId, MessageType};
use crate::node::Node;

pub fn handle_broadcast(node: &mut Node, req: &Req, res: &mut Res) {
    assert_eq!(req.body.r#type, MessageType::Broadcast);
    res.body.r#type = MessageType::BroadcastOk;
    res.body.in_reply_to = req.body.msg_id;
    node.broadcaster.broadcast(req.body.message.unwrap());
    res.send().unwrap();
}

pub fn handle_read(node: &mut Node, req: &Req, res: &mut Res) {
    assert_eq!(req.body.r#type, MessageType::Read);
    res.body.r#type = MessageType::ReadOk;
    res.body.in_reply_to = req.body.msg_id;
    res.body.messages = Some(node.broadcaster.read());
    res.send().unwrap();
}

pub fn handle_topology(node: &mut Node, req: &Req, res: &mut Res) {
    assert_eq!(req.body.r#type, MessageType::Topology);
    res.body.in_reply_to = req.body.msg_id;
    res.body.r#type = MessageType::TopologyOk;
    res.send().unwrap();
}

pub struct Broadcaster {
    messages: Vec<MessageId>,
}

impl Broadcaster {
    pub fn new() -> Broadcaster {
        Broadcaster { 
            messages: Vec::new(),
        }
    }

    pub fn broadcast(&mut self, msg: MessageId) {
        self.messages.push(msg);
    }

    pub fn read(&mut self) -> Vec<MessageId> {
        self.messages.clone()
    }
}
