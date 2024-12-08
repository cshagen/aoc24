use std::fs::read_to_string;

pub fn get_lines(filename: &str) -> Vec<String> {
    read_to_string(&filename)
        .unwrap()
        .lines()
        .map(String::from)
        //.filter(|l| l.len() > 0)
        .collect()
}

pub struct Queue<T> {
	queue: Vec<T>

}
impl<T> Queue<T> {
	pub fn new() -> Self {
		Queue { queue: Vec::new()}
	}
	pub fn enqueue(&mut self, item:T) {
		self.queue.push(item)
	}
	pub fn dequeue(&mut self) -> T{
		self.queue.remove(0)
	}
}