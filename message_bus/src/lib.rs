#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



mod message_bus {
extern crate crossbeam_channel;
use self::crossbeam_channel::unbounded;

use std::sync::Mutex;

    struct Message{
        to_id: u32,
        from_id: u32
    }

    struct Messagebus{
        bus_id: u64,
        m_buffer: Mutex<Vec<Message>>,
        subscribers: Vec<(u32, crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>)>
    }

    impl Messagebus{
        fn new(bus_id: u64) -> Self{
            Messagebus{bus_id: bus_id, m_buffer: Mutex::new(Vec::new()), subscribers: Vec::new()}
        }

        pub fn subscribe(&mut self, sub_tag: u32) -> (crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>){
            let (send, receive) = unbounded::<Message>();
            self.subscribers.push((sub_tag, send.clone(), receive.clone()));
            (send, receive)
        }

        fn process(&mut self) {
            match self.m_buffer.get_mut(){
                Ok(exclusive_buffer) => {
                    for message in exclusive_buffer.iter(){
                    }
                    exclusive_buffer.truncate(0)
                },
                Err(e) => {}
            }

        }

        pub fn publish(&mut self, m: Message) -> (){
            match self.m_buffer.get_mut(){
                Ok(exclusive_buffer) => {
                    exclusive_buffer.push(m)
                },
                Err(e) => {}
            }
        }
    }
}
