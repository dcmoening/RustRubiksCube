use std::io::{self, Write};

pub mod CubeArray {
    //A Rubiks cube is made up of 6 sides. Each side contains 6 small squares 
    //Actions are done on the following faces: Front, Back, Left, Right, Top, Lower
    //Each are relative to the Front face.
    //Rotate cube to change to a different front face.

    /*
    Front:  w,w,w,w,w,w,w,w,w
    Back:   y,y,y,y,y,y,y,y,y
    Top:    b,b,b,b,b,b,b,b,b
    Lower:  g,g,g,g,g,g,g,g,g
    Right:  r,r,r,r,r,r,r,r,r
    Left:   o,o,o,o,o,o,o,o,o
    */
    
    #[derive(Copy, Clone, Debug)]
    pub enum RubikColor {
        White,
        Yellow,
        Blue,
        Green,
        Red,
        Orange
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Cube {
        front: [RubikColor; 9],
        back: [RubikColor; 9],
        top: [RubikColor; 9],
        lower: [RubikColor; 9],
        right: [RubikColor; 9],
        left: [RubikColor; 9],
    }

    impl Cube {
        pub fn printcube(&self) ->() {
            for i in 0..self.front.len() {print!("{:?} ", self.front[i])};
            print!("\r\n");
            for i in 0..self.left.len() {print!("{:?} ", self.left[i])};
            print!("\r\n");
            for i in 0..self.right.len() {print!("{:?} ", self.right[i])};
            print!("\r\n");
            for i in 0..self.top.len() {print!("{:?} ", self.top[i])};
            print!("\r\n");
            for i in 0..self.lower.len() {print!("{:?} ", self.lower[i])};
            print!("\r\n");
            for i in 0..self.back.len() {print!("{:?} ", self.back[i])};
            print!("\r\n");
        }
    }

    impl Default for Cube {
        fn default() -> Cube {
            Cube {
                front: [RubikColor::White; 9],
                back: [RubikColor::Yellow; 9],
                top: [RubikColor::Blue; 9],
                lower: [RubikColor::Green; 9],
                right: [RubikColor::Red; 9],
                left: [RubikColor::Orange; 9]
            }
        }
    }
}