#[derive(Debug)]
struct Message {
    to: u64,
    content: String
}

fn fetch_cube_station_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

struct GroundStation;

impl GroundStation {
    fn connect(&self, cube_id: u64) -> CubeStation {
        CubeStation{
            id: cube_id,
        }
    }
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

#[derive(Debug)]
struct CubeStation {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox{
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }
    fn deliver(&mut self, cube: &CubeStation) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == cube.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

impl CubeStation {
    fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn main() {
    let mut mail = Mailbox{messages: vec![]};
    let base = GroundStation{};
    let cubes = fetch_cube_station_ids();
    for cube in cubes {
        let msg = Message{to: cube, content:String::from("hello")};
        base.send(&mut mail, msg);
    }
    let cubes = fetch_cube_station_ids();
    for cube in cubes {
        let mut cube_item = base.connect(cube);
        let msg = cube_item.recv(&mut mail).unwrap();
        println!("cube station{:?} get msg: {:?}", cube, msg);
    }
}