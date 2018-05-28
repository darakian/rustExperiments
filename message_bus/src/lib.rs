#[cfg(test)]
mod tests {
    use message_bus::{Message, Messagebus};

    #[test]
    fn it_works() {
        let mut mb = Messagebus::new(1);
        mb.publish(Message::new(1,2));
        mb.publish(Message::new(1,2));
        mb.publish(Message::new(1,2));
        assert_ne!(mb.process(), mb.process());
        mb.process();
    }

    #[test]
    fn print() {
        println!("Yes?");
    }
}



mod message_bus {
extern crate crossbeam_channel;
use self::crossbeam_channel::unbounded;

use std::sync::Mutex;
use std::collections::hash_map::{HashMap, Entry};

    pub struct Message{
        to_id: u32,
        from_id: u32
    }

    impl Message {
        pub fn new(to: u32, from: u32) -> Self{
            Message{to_id: to, from_id: from}
        }
    }

    pub struct Messagebus{
        bus_id: u64,
        m_buffer: Mutex<Vec<Message>>,
        subscribers:  Mutex<HashMap<u32, (crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>)>>
    }

    impl Messagebus{
        pub fn new(bus_id: u64) -> Self{
            Messagebus{bus_id: bus_id, m_buffer: Mutex::new(Vec::new()), subscribers: Mutex::new(HashMap::new())}
        }

        pub fn subscribe(&mut self, sub_tag: u32) -> Result<(crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>), &str>{
            let (send, receive) = unbounded::<Message>();
            match self.subscribers.get_mut(){
                Ok(exclusive_buffer) => {
                    match exclusive_buffer.entry(sub_tag) {
                        Entry::Vacant(eb) => {eb.insert((send.clone(), receive.clone()));},
                        Entry::Occupied(mut e) => {return Err("Sub_ID in use");}
                        }
                },
                Err(e) => {println!("{:?}", e);return Err("Poison error")}
            }
            Ok((send, receive))
        }

        pub fn process(&mut self) -> usize {
            let mut r_value = 0;
            match self.m_buffer.get_mut(){
                Ok(mut exclusive_messages) => {
                    r_value = exclusive_messages.len();
                    match self.subscribers.get_mut(){
                            Ok(exclusive_subscribers) => {
                                for entry in exclusive_messages.drain(..){
                                    //let msg = exclusive_messages.remove(entry.0);
                                    match exclusive_subscribers.get(&entry.to_id){
                                        Some(s) => {s.0.send(entry).unwrap()},
                                        None => {}
                                    }
                                }
                            },
                            Err (e) => {}
                    }
                },
                Err(e) => {}
            }
            r_value
        }

        pub fn publish(&mut self, m: Message) -> (){
            match self.m_buffer.get_mut(){
                Ok(exclusive_buffer) => exclusive_buffer.push(m),
                Err(e) => {}
            }
        }
    }
}
