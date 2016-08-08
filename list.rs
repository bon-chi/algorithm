struct Cell<'a> {
    data: Option<String>,
    pointer: Option<&'a Cell<'a>>,
}

impl<'a> Cell<'a> {
    fn set_data(&mut self, data: String) {
        self.data = Some(data);
    }

    fn set_pointer(&mut self, pointer: &'a Cell) {
        self.pointer = Some(pointer);
    }

    fn get_data(&self) -> Option<String> {
        self.data.clone()
    }

    fn get_pointer(&self) -> Option<&'a Cell> {
        self.pointer
    }
}

fn main() {
    let mut c4: Cell = Cell {
        data: None,
        pointer: None,
    };

    let mut c3: Cell = Cell {
        data: None,
        pointer: None,
    };

    let mut c2: Cell = Cell {
        data: None,
        pointer: None,
    };

    let mut c1: Cell = Cell {
        data: None,
        pointer: None,
    };

    let mut head: Cell = Cell {
        data: None,
        pointer: None,
    };

    c3.set_data("C".to_string());
    c2.set_data("B".to_string());
    c1.set_data("A".to_string());

    c2.set_pointer(&c3);
    c1.set_pointer(&c2);
    head.set_pointer(&c1);

    let mut pointer = &head;
    // let str = "B".to_string();
    // c3.set_data("D".to_string());
    // c2.set_pointer(&c4);
    // pointer.set_data(str);
    // loop {
    //     match pointer.get_data() {
    //         Some(str) => {
    //             pointer.set_data(str);
    //             break;
    //         }
    //         _ => {
    //             match pointer.get_pointer() {
    //                 Some(raw_pointer) => pointer = raw_pointer,
    //                 None => {
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }

    while let Some(raw_pointer) = pointer.get_pointer() {
        pointer = raw_pointer;
        match pointer.get_data() {
            Some(data) => println!("data:{}", data),
            None => {}
        }
    }
    println!("finish!");
}
