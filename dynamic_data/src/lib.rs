#[derive(Debug, Clone, Copy)]
pub enum Message {
    Previous(isize),
    Next(isize),
}
pub trait DataLoader<T> {
    fn get_next_n(&self, n: isize, from: isize) -> Vec<T>;
    fn get_previous_n(&self, n: isize, from: isize) -> Vec<T>;
}

pub struct DynamicData<T, D>
where
    D: DataLoader<T>,
{
    current_start: isize,
    capacity: usize,
    data_source: D,
    data: Vec<T>,
}

impl<T, D: DataLoader<T>> DynamicData<T, D> {
    pub fn new(data_source: D) -> Self {
        DynamicData {
            current_start: 0,
            capacity: 40,
            data_source,
            data: Vec::new(),
        }
    }
    pub fn current_start(mut self, s: isize) -> Self {
        self.current_start = s;
        self
    }

    pub fn capacity(mut self, k: usize) -> Self {
        self.capacity = k;
        self
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn fetch_next(&mut self, n: isize) {
        let from = self.current_start + (self.data.len() as isize);
        let data_to_add = self.data_source.get_next_n(n, from);
        let over_capacity = (data_to_add.len() + self.data.len()) as isize - self.capacity as isize;

        if over_capacity > 0 {
            self.data = self.data.split_off(over_capacity as usize);
            self.current_start += over_capacity as isize;
        }

        self.data.extend(data_to_add);
    }

    pub fn fetch_previous(&mut self, n: isize) {
        let from = self.current_start;
        let mut data_to_add = self.data_source.get_previous_n(n, from);
        self.current_start -= data_to_add.len() as isize;
        self.data.reverse();
        data_to_add.reverse();
        self.data.extend(data_to_add);
        self.data.reverse();
        self.data.truncate(self.capacity);
    }
}
