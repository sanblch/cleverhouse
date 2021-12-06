use crate::room::Room;
use std::collections::HashMap;

pub struct House {
    pub name: String,
    rooms: HashMap<String, Room>,
}

impl House {
    pub fn new(name: String) -> Self {
        House {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: &str, room: Room) -> Result<(), Error> {
        match self.rooms.insert(name.to_owned(), room) {
            Some(room) => {
                self.rooms.insert(name.to_owned(), room);
                Err(Error::AlreadyExist)
            }
            None => Ok(())
        }
    }

    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(&name.to_owned())
    }

    pub fn remove_room(&mut self, name: &str) -> Result<(), Error> {
        match self.rooms.remove(&name.to_string()) {
            Some(_room) => Ok(()),
            None => Err(Error::NotExist),
        }
    }

    pub fn list_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(|c| c.0.as_str()).collect()
    }

    pub fn report(&self) {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    AlreadyExist,
    NotExist,
    NotUnique,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::AlreadyExist => write!(f, "House already contains a room with the name"),
            Error::NotExist => write!(f, "Room doesn't exist"),
            Error::NotUnique => write!(f, "Room name not unique"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    const H1: &str = "MyHouse1";
    const H2: &str = "MyHouse2";
    const R1: &str = "MyRoom1";
    const R2: &str = "MyRoom2";

    fn test_empty_house() -> House {
        House::new(H1.to_string())
    }

    fn test_house() -> House {
        let mut house = test_empty_house();
        let _res = house.add_room(R1, Room::new());
        house
    }

    #[test]
    fn create_house() {
        let house = test_empty_house();
        assert_eq!(house.name, test_empty_house().name);
        assert_eq!(house.list_rooms(), Vec::<String>::new());
    }

    #[test]
    fn add_room() {
        let mut house = test_empty_house();
        assert_eq!(house.add_room(R1, Room::new()), Ok(()));
        assert_eq!(house.add_room(R1, Room::new()), Err(Error::AlreadyExist));
        assert_eq!(house.list_rooms(), vec![R1]);

        let mut house2 = House::new(H2.to_string());
        assert_eq!(house2.add_room(R1, Room::new()), Err(Error::NotUnique));
    }

    #[test]
    fn remove_room() {
        let mut house = test_house();
        assert_eq!(house.list_rooms(), vec![R1]);
        assert_eq!(house.remove_room(R1), Ok(()));
        assert_eq!(house.remove_room(R1), Err(Error::NotExist));
        assert_eq!(house.list_rooms(), Vec::<String>::new());
    }

    #[test]
    fn get_room() {
        let house = test_house();
        assert_eq!(house.get_room(R1).is_some(), true);
        assert_eq!(house.get_room(R2).is_none(), true);
    }
}
