#[cfg(test)]
mod tests {
    use message_bus::{Message, Messagebus};

    #[test]
    fn it_works() {
        let mut mb = Messagebus::new(1);
        mb.publish(Message::new(1,2));
        mb.publish(Message::new(1,2));
        mb.publish(Message::new(1,2));
    }
}



mod message_bus {
extern crate crossbeam_channel;
use self::crossbeam_channel::unbounded;

use std::sync::Mutex;
use std::collections::hash_map::{HashMap, Entry};

    pub struct Message{
        to_id: String,
        from_id: String
    }

    impl Message {
        pub fn new(to: String, from: String) -> Self{
            Message{to_id: to, from_id: from}
        }
    }

    pub struct Messagebus{
        bus_id: String,
        global_recv: crossbeam_channel::Receiver<Message>,
        global_send: crossbeam_channel::Sender<Message>,
        subscribers:  Mutex<HashMap<String, (crossbeam_channel::Sender<Message>)>>
    }

    impl Messagebus{
        pub fn new(bus_id: String) -> Self{
            let (send, receive) = unbounded::<Message>();
            let mut bus = Messagebus{bus_id: bus_id, global_recv: receive, global_send: send, subscribers: Mutex::new(HashMap::new())};
            let (bus_self_tx, bus_self_rx) = bus.subscribe("bus".to_string()).unwrap();

            match bus.subscribers.get_mut(){
                Ok(exclusive_buffer) => {
                    match exclusive_buffer.entry("bus".to_string()) {
                        Entry::Vacant(eb) => {eb.insert(bus_self_tx);},
                        Entry::Occupied(mut e) => {println!("{:?}", e);}
                        }
                },
                Err(e) => {println!("{:?}", e);}
            }
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
                match self.subscribers.get_mut(){
                        Ok(exclusive_subscribers) => {
                                match exclusive_subscribers.get(&msg.to_id){
                                    Some(s) => {s.send(msg).unwrap()},
                                    None => {}
                                }
                        },
                        Err (e) => {}
                }
            }
    }

        pub fn publish(&mut self, m: Message) -> (){
            self.global_send.send(m).unwrap();
        }
    }
}
