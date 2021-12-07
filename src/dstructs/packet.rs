pub struct Packet {
    data: String,
}

impl Packet {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    pub fn from(data: String) -> Self {
        Self { data }
    }

    pub fn get_len(&self) -> u8 {
        self.data.len() as u8
    }

    pub fn append(&mut self, data: String) -> u8 {
        let len = data.len();
        self.data.push_str(&data);
        len as u8
    }

    pub fn push(&mut self, data: u8) {
        //println!("{:08b}", data);
        self.data.push_str(&format!("{:08b}", data));
    }

    pub fn get_bin_slice(&self, start_index: usize, end_index: usize) -> String {
        self.data[start_index..end_index].to_string()
    }

    pub fn get_slice(&self, start_index: usize, end_index: usize) -> Vec<u8> {
        if (end_index - start_index) % 8 != 0 {
            panic!("Requested slice not possible. Index is not in 8 bit boundary.");
        }
        if self.data.len() % 8 != 0 {
            panic!("Data not in 8-bit boundary {:?}", self.data);
        }
        let mut data: Vec<u8> = Vec::with_capacity(self.data.len());
        for i in (start_index..end_index).step_by(8) {
            data.push(u8::from_str_radix(&self.data[i..i + 8], 2).unwrap());
        }
        data
    }
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        if self.data.len() % 8 != 0 {
            panic!(
                "Data not in 8-bit boundary {:?}, Length: {}",
                self.data,
                self.data.len()
            );
        }
        let mut data: Vec<u8> = Vec::with_capacity(self.data.len());
        for i in (0..self.data.len()).step_by(8) {
            data.push(u8::from_str_radix(&self.data[i..i + 8], 2).unwrap());
        }
        data
    }
}

impl From<Vec<u8>> for Packet {
    fn from(array: Vec<u8>) -> Self {
        let mut p = Self::new();
        for elmt in array {
            p.push(elmt);
        }
        p
    }
}

impl From<&[u8]> for Packet {
    fn from(array: &[u8]) -> Self {
        let mut p = Self::new();
        for elmt in array {
            p.push(*elmt);
        }
        p
    }
}
