/*
    * METHOD
        - function that is associated with a particolar type or struct
        - takes parameters and returns a value, but defined as a memeber 
            of a struct or an enum
        - called using dot notation ( like accessing member of a struct )
        - implemented through an 'impl' block
        
        * ASSOCIATED FUNCTION
        - function is associated with a struct or an enum,
        but DOESN'T take an instance as its first parameter
        - called using the name of the type , not an instance of it
        - often used as constructors for a struc or an enum
        - implemented through an 'impl' block
*/

pub fn _methods_and_function_association(){

    println!("\n\n---- METHODS AND FUNCTION ASSOCIATION ---- \n\n");

    method_usage();
    associated_function_usage();
    consuming_heap_allocation();
    enum_methods();

    println!("\n\n---- //METHODS AND FUNCTION ASSOCIATION// ---- \n\n");
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

/*
    Syntax: 'impl' keyword + name of the struct or enum to associate
    &self -> reference to the instance of the associated struct or enum

    self and &self -> self will take the ownership of current struct instance, however,
        &self will only borrow a reference from the instance.
*/
impl Rectangle {
    //* METHODS */
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }
    fn calculate_volume(&self, profondita: u32) -> f32 {
        (self.calculate_area() * profondita) as f32
    }
    //* ASSOCIATED FUNCTION */
    fn new(base: u32, altezza: u32) -> Rectangle {
        println!("\n ...Creating new rectangle instance...");
        Rectangle{ width: base, height: altezza}
    }

    //* MOVING VALUE TO DESTROY INSTANCE (spiegato dopo) */
    fn _destroy(self) {
        Rectangle{width: self.width, height:self.height};
    }
}

fn method_usage(){

    let rect: Rectangle = Rectangle{height: 12, width: 24};
    
    //? access through dot notation
    //? &self in this case is the Rectangle instance 'rect' 
    let area: u32 = rect.calculate_area();

    let volume: f32 = rect.calculate_volume(5);

    println!("Rectangle area = {}", area);
    println!("Rectangle volume = {}", volume);

}

fn associated_function_usage(){

    /*
        To access an associated function we use the '::' operator
    */
    let rect2: Rectangle = Rectangle::new(5, 10);

    println!("Rectangle 2 instanciated via contructor:\n{:?}", rect2);

    let area: u32 = rect2.calculate_area();

    let volume: f32 = rect2.calculate_volume(2);

    println!("Rectangle 2 area = {}", area);
    println!("Rectangle 2 volume = {}", volume);
}

#[derive(Debug)]
struct HeapRectangle {
    width: Box<u32>,
    height: Box<u32>
}

impl HeapRectangle {

    fn constructor(width: u32, height: u32) -> HeapRectangle {
        HeapRectangle {
            width: Box::new(width),
            height: Box::new(height)
        }
    }

    //  This method "consumes" the resources of the caller object
    //* note that 'self' is not a reference
    /*
        self will take the ownership of current struct instance, however,
            &self will only borrow a reference from the instance.
    */
    fn destroy(self) { 
        // Destructure `self`
        let HeapRectangle{width, height} = self;

        println!("Destroying Heap Recangle (width: {}, height: {})", width, height);

        // `width` and `height` go out of scope and get freed.
    }
}

fn consuming_heap_allocation(){

    let heap_rect: HeapRectangle = HeapRectangle::constructor(12, 6);

    println!("\nHeap rectangle allocated: {:?}", heap_rect);
    
    heap_rect.destroy();
    
    // println!("{:?}", heap_rect); //? cant because consumed
    println!("Heap rectangle destroyed.");
}

/*
    * Implementing methods in enums
*/

#[derive(Debug)]
enum TrafficLightColor {
    _Red,
    Yellow,
    _Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            TrafficLightColor::_Red => "red light",
            TrafficLightColor::Yellow => "yellow light",
            TrafficLightColor::_Green => "green light",
        }
    }
}

fn enum_methods() {
    let c: TrafficLightColor = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow light");

    println!("\nenum TrafficLightColor => c = {:?}",c);
    println!("enum TrafficLightColor light color => c.color() = {}",c.color());
}