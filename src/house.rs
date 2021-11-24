use crate::room::Room;

pub struct House {
    _name: String,
    _rooms: Vec<Room>,
}

impl House {
    pub fn new(_name: String) -> Self {
        House {
            _name,
            _rooms: Vec::new(),
        }
    }

    fn _add_room(&self, _room: Room) -> Result<(), String> {
        todo!()
    }

    fn _remove_room(&self, _name: String) -> Result<(), String> {
        todo!()
    }

    fn _list_rooms(&self) -> Vec<String> {
        todo!()
    }

    fn _report(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_house() {
        let mut _house = crate::House::new(String::from("MyHouse"));
    }
}
