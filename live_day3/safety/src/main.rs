use std::any::Any;
use std::fmt::Debug;

pub trait Animal: Debug {
    fn speak(&self);
}

trait Downcastable {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct Cat {}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

impl Downcastable for Cat {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct Dog {}
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}



fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
}

fn cat_factory() -> impl Animal {
    Cat{}
}

fn main() {
    let animals: Vec<Box<dyn Downcastable>> = vec![Box::new(Cat{}),];
    for animal  in animals.iter() {
        if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
            // I finally know that I crave lasagna
        }
    }
    
    
}