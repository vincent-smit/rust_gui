fn main() {
    println!("Hello, world!");

    let screen =  Screen {
        components: vec![Box::new(SelectBox {
            width: 500,
            height: 500,
            options: vec![String::from("A"),
            String::from("B"),
            String::from("C")
            ],
        }),
        Box::new(Button {
            width: 10,
            height: 10,
            label: String::from("Press me."),

        })
        ]
    };

    screen.run();

}


pub trait Draw {
    fn draw(&self);
}

// Using traits objects
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// using trait bounds
pub struct ScreenBound<T: Draw> {
    pub components: Vec<T>,

}

impl<T> ScreenBound<T>
    where T: Draw {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
}

// Button struct and trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("The width is {}, the height is {}, the label is {}", self.width, self.height, self.label)
    }
}

// SelectBox struct and trait

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("The options are {:?}", self.options)
    }
}