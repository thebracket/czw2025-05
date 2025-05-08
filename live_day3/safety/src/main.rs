fn print_it<T: ToString+ std::fmt::Debug +Clone>(t: T) {
    println!("{}", t.to_string());
}

fn print_it2<T>(t: T)
where T: ToString 
{
    println!("{}", t.to_string());
}

struct Degrees(f32);
struct Radians(f32);

impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Degrees(rad.0 * 180.0 / std::f32::consts::PI)
    }
}

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Radians(deg.0 * std::f32::consts::PI / 180.0)
    }
}

fn sin(angle: impl Into<Radians>) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}

struct PersonId(usize);
struct BookId(usize);

use std::collections::HashMap;

#[derive(Debug)]
struct HashMapBucket<KEY, VALUE> {
    map: HashMap<KEY, Vec<VALUE>>,
}

impl<KEY, VALUE> HashMapBucket<KEY, VALUE> 
where KEY: Eq + std::hash::Hash
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: KEY, value: VALUE) {
        let entry = self.map.entry(key).or_insert_with(Vec::new);
        entry.push(value);
    }
}

fn main() {
    // let mut my_map = HashMapBucket::new();
    // my_map.insert(0, "hello");
    // my_map.insert(1, "world");
    // my_map.insert(2, "hello");
    // println!("{:?}", my_map);
    // 
    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // numbers.iter().for_each(|n| println!("{n}"));
    // for n in &numbers {}

    //let numbers: Vec<u32> = Counter::new(10).collect();
    //println!("{numbers:?}");
    
    
}

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Counter {
    fn len(&self) -> usize {
        self.max as usize
    }
}