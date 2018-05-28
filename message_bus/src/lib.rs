#[cfg(test)]
mod tests {
    use message_bus::{Message, Messagebus};

    #[test]
    fn it_works() {
        let mut mb = Messagebus::new("bus");
        mb.publish(Message::new("1","2"));
        mb.publish(Message::new("2","2"));
        mb.publish(Message::new("3","2"));
        assert_eq!(mb.check_channel_len(), 3);
        mb.publish(Message::new("bus","2"));
        
        mb.do_messaging();
    }
}



mod message_bus {
extern crate crossbeam_channel;
use self::crossbeam_channel::unbounded;

use std::sync::Mutex;
use std::collections::hash_map::{HashMap, Entry};

    #[derive(Debug)]
    pub struct Message{
        to_id: String,
        from_id: String
    }

    impl Message {
        pub fn new(to: &str, from: &str) -> Self{
            Message{to_id: to.to_string(), from_id: from.to_string()}
        }
    }

    pub struct Messagebus{
        bus_id: String,
        global_recv: crossbeam_channel::Receiver<Message>,
        global_send: crossbeam_channel::Sender<Message>,
        subscribers:  Mutex<HashMap<String, (crossbeam_channel::Sender<Message>)>>
    }

    impl Messagebus{
        pub fn new(bus_id: &str) -> Self{
            let (send, receive) = unbounded::<Message>();
            let working_bus_id = bus_id.clone();
            let mut bus = Messagebus{bus_id: bus_id.to_string(), global_recv: receive, global_send: send, subscribers: Mutex::new(HashMap::new())};
            let (bus_self_tx, bus_self_rx) = bus.subscribe(working_bus_id.to_string()).unwrap();
            bus
        }

        pub fn subscribe(&mut self, sub_tag: String) -> Result<(crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>), &str>{
            let (send, receive) = unbounded::<Message>();
            match self.subscribers.get_mut(){
                Ok(exclusive_buffer) => {
                    match exclusive_buffer.entry(sub_tag) {
                        Entry::Vacant(eb) => {eb.insert(send.clone());},
                        Entry::Occupied(mut e) => {return Err("Sub_ID in use");}
                        }
                },
                Err(e) => {println!("{:?}", e);return Err("Poison error")}
            }
            Ok((self.global_send.clone(), receive))
        }

        pub fn do_messaging(&mut self) {
            loop {
                let msg = self.global_recv.recv().unwrap();
                println!("{:?}", msg);
                if msg.to_id == self.bus_id{
                    //handle message for self
                    return
                }
                match self.subscribers.get_mut(){
                        Ok(exclusive_subscribers) => {
                                match exclusive_subscribers.get(&msg.to_id){
                                    Some(s) => {s.send(msg).unwrap()},
                                    None => {drop(msg)}
                                }
                        },
                        Err (e) => {}
                }
            }
    }

        pub fn publish(&mut self, m: Message) -> (){
            self.global_send.send(m).unwrap();
        }

        pub fn check_channel_len(&self) -> usize{
            self.global_recv.len()
        }
    }
}
