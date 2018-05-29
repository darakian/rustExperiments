#[cfg(test)]
mod tests {
    use message_bus::{Message, Messagebus};

    #[test]
    fn it_works() {
        let mut mb = Messagebus::new("bus");
        mb.publish(Message::new("1",2));
        mb.publish(Message::new("2",2));
        mb.publish(Message::new("3",2));
        assert_eq!(mb.check_channel_len(), 3);
        mb.publish(Message::new("bus",2));
        mb.do_messaging();
    }
}



mod message_bus {
extern crate crossbeam_channel;
use self::crossbeam_channel::unbounded;

use std::sync::Mutex;
use std::collections::hash_map::{HashMap, Entry};

    #[derive(Debug, Clone)]
    pub struct Message{
        publish_tag: String,
        publisher: u64
    }

    impl Message {
        pub fn new(to: &str, from: u64) -> Self{
            Message{publish_tag: to.to_string(), publisher: from}
        }
    }

    pub struct Messagebus{
        bus_id: String,
        global_recv: crossbeam_channel::Receiver<Message>,
        global_send: crossbeam_channel::Sender<Message>,
        subscribers:  Mutex<HashMap<u64, (crossbeam_channel::Sender<Message>)>>,
        feeds: Mutex<HashMap<String, Vec<crossbeam_channel::Sender<Message>>>>
    }

    impl Messagebus{
        pub fn new(bus_id: &str) -> Self{
            let (send, receive) = unbounded::<Message>();
            let mut bus = Messagebus{bus_id: bus_id.to_string(), global_recv: receive, global_send: send, subscribers: Mutex::new(HashMap::new()), feeds: Mutex::new(HashMap::new())};
            let (bus_self_tx, bus_self_rx) = bus.join(0).unwrap();
            bus
        }

        pub fn join(&mut self, component_id: u64) -> Result<(crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>), &str>{
            let (send, receive) = unbounded::<Message>();
            match self.subscribers.get_mut(){
                Ok(exclusive_subscribers) => {
                    match exclusive_subscribers.entry(component_id) {
                        Entry::Vacant(eb) => {eb.insert(send.clone());},
                        Entry::Occupied(mut e) => {return Err("Sub_ID in use");}
                        }
                },
                Err(e) => {println!("{:?}", e);return Err("Poison error")}
            }
            Ok((self.global_send.clone(), receive))
        }

        pub fn subscribe(&mut self, component_id: u64, sub_tag: String) -> Result<(), &str>{
            match self.subscribers.get_mut(){
                Ok(exclusive_subscribers) => {
                    match self.feeds.get_mut(){
                        Ok(exclusive_feeds) => {
                            match exclusive_feeds.get_mut(&sub_tag) {
                                Some(vec) => {
                                    if vec.contains(exclusive_subscribers.get(&component_id).unwrap()) {return Ok(())}
                                    else {vec.push(exclusive_subscribers.get(&component_id).unwrap().clone())}
                                },
                                None => {}
                                }
                        },
                        Err(e) => {}
                    }
                },
                Err(e) => {println!("{:?}", e);return Err("Poison error")}
            }
            Ok(())
        }

        pub fn do_messaging(&mut self) {
            loop {
                let msg = self.global_recv.recv().unwrap();
                println!("{:?}", msg);
                if msg.publish_tag == self.bus_id{
                    //handle message for self
                    return
                }
                match self.feeds.get_mut(){
                        Ok(exclusive_feeds) => {
                                match exclusive_feeds.get(&msg.publish_tag){
                                    Some(feed_subscribers) => {feed_subscribers.iter().for_each(|x| x.send(msg.clone()).unwrap())},
                                    None => {drop(msg)}
                                }
                        },
                        Err (e) => {}
                }
            }
        }


        //Testing methods
        pub fn publish(&mut self, m: Message) -> (){
            self.global_send.send(m).unwrap();
        }

        pub fn check_channel_len(&self) -> usize{
            self.global_recv.len()
        }
    }
}
