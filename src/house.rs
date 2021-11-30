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

    pub fn add_room(&self, _room: Room) -> Result<(), Error> {
        todo!()
    }

    pub fn get_room(&self, _name: String) -> Option<Room> {
        todo!()
    }

    pub fn remove_room(&self, _name: String) -> Result<(), Error> {
        todo!()
    }

    pub fn list_rooms(&self) -> Vec<String> {
        todo!()
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

    fn test_empty_house() -> House {
        House::new(String::from("MyHouse"))
    }

    fn test_house() -> House {
        let house = test_empty_house();
        let _res = house.add_room(test_room());
        house
    }

    fn test_room() -> Room {
        Room::new(String::from("MyRoom"))
    }

    #[test]
    fn create_house() {
        let house = test_empty_house();
        assert_eq!(house.name, test_empty_house().name);
        assert_eq!(house.list_rooms(), Vec::<String>::new());
    }

    #[test]
    fn add_room() {
        let house = test_empty_house();
        assert_eq!(house.add_room(test_room()), Ok(()));
        assert_eq!(house.add_room(test_room()), Err(Error::AlreadyExist));
        assert_eq!(house.list_rooms(), vec![test_room().name]);

        let house2 = House::new(String::from("MyHouse2"));
        assert_eq!(house2.add_room(test_room()), Err(Error::NotUnique));
    }

    #[test]
    fn remove_room() {
        let house = test_house();
        assert_eq!(house.list_rooms(), vec![test_room().name]);
        assert_eq!(house.remove_room(String::from(test_room().name)), Ok(()));
        assert_eq!(house.remove_room(test_room().name), Err(Error::NotExist));
        assert_eq!(house.list_rooms(), Vec::<String>::new());
    }

    #[test]
    fn get_room() {
        let house = test_house();
        assert_eq!(house.get_room(test_room().name).is_some(), true);
        assert_eq!(house.get_room(String::from("MyRoom2")).is_none(), true);
    }
}
