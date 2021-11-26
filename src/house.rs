use crate::room::Room;

pub struct House {
    pub name: String,
    _rooms: Vec<Room>,
}

impl House {
    pub fn new(name: String) -> Self {
        House {
            name,
            _rooms: Vec::new(),
        }
    }

    pub fn add_room(&self, _room: Room) -> Result<(), String> {
        todo!()
    }

    pub fn get_room(&self, _name: String) -> Result<Room, String> {
        todo!()
    }

    pub fn remove_room(&self, _name: String) -> Result<(), String> {
        todo!()
    }

    pub fn list_rooms(&self) -> Vec<String> {
        todo!()
    }

    pub fn report(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_house() -> House {
        House::new(String::from("MyHouse"))
    }

    fn test_room() -> Room {
        Room::new(String::from("MyRoom"))
    }

    #[test]
    fn create_house() {
        let house = test_house();
        assert_eq!(house.name, "MyHouse");
        assert_eq!(house.list_rooms(), Vec::<String>::new());
    }

    #[test]
    fn add_remove_room() {
        let house = test_house();
        let room = test_room();
        let _res = house.add_room(room);
        assert_eq!(house.list_rooms(), vec![test_room().name]);
        let _res2 = house.remove_room(String::from(test_room().name));
        assert_eq!(house.list_rooms(), Vec::<String>::new());
    }

    #[test]
    fn get_room() {
        let house = test_house();
        let room = test_room();
        let _res = house.add_room(room);
        let room2 = match house.get_room(test_room().name) {
            Ok(u) => u,
            Err(_e) => unreachable!(),
        };
        assert_eq!(room2.name, test_room().name);
    }
}
