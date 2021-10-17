#![allow(non_snake_case)]
//#![windows_subsystem = "windows"]
extern crate minifb;

use minifb::{Key, Window, WindowOptions, KeyRepeat};
use rand::Rng;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

struct Block {
    x:u32,
    y:u32,
}

struct Piece {
    x:i32,
    y:i32,
    color: u32,
    r: u32,
}

impl Piece {
    fn new(x:i32, y:i32, color:u32) -> Self {
        Piece{
            x: x,
            y: y,
            color: color,
            r: 0,
        }
    }
    fn getBlocks(&self) -> Vec<Block> {
        let s_x = self.x as f32;
        let s_y = self.y as f32;
        match self.color {
            1 => {
                return vec![Block{x: (s_x+rX(-1.0, 1.0, self.r)) as u32, y: (s_y+rY(-1.0, 1.0, self.r)) as u32}, Block{x: (s_x+rX(1.0, 0.0, self.r)) as u32, y: (s_y+rY(1.0, 0.0, self.r)) as u32}, Block{x: (s_x) as u32, y: (s_y) as u32}, Block{x: (s_x+rX(0.0, 1.0, self.r)) as u32, y: (s_y+rY(0.0, 1.0, self.r)) as u32}];
            },
            2 => {
                return vec![Block{x: (s_x+rX(1.0, 1.0, self.r)) as u32, y: (s_y+rY(1.0, 1.0, self.r)) as u32}, Block{x: (s_x+rX(-1.0, 0.0, self.r)) as u32, y: (s_y+rY(-1.0, 0.0, self.r)) as u32}, Block{x: (s_x) as u32, y: (s_y) as u32}, Block{x: (s_x+rX(1.0, 0.0, self.r)) as u32, y: (s_y+rY(1.0, 0.0, self.r)) as u32}];
            },
            3 => {
                return vec![Block{x: (s_x+rX(-0.5, 0.5, self.r)-0.5) as u32, y: (s_y+rY(-0.5, 0.5, self.r)+0.5) as u32}, Block{x: (s_x+rX(-0.5, -0.5, self.r)-0.5) as u32, y: (s_y+rY(-0.5, -0.5, self.r)+0.5) as u32}, Block{x: (s_x+rX(0.5, -0.5, self.r)-0.5) as u32, y: (s_y+rY(0.5, -0.5, self.r)+0.5) as u32}, Block{x: (s_x+rX(0.5, 0.5, self.r)-0.5) as u32, y: (s_y+rY(0.5, 0.5, self.r)+0.5) as u32}];
            },
            4 => {
                return vec![Block{x: (s_x+rX(1.0, 1.0, self.r)) as u32, y: (s_y+rY(1.0, 1.0, self.r)) as u32}, Block{x: (s_x+rX(-1.0, 0.0, self.r)) as u32, y: (s_y+rY(-1.0, 0.0, self.r)) as u32}, Block{x: (s_x) as u32, y: (s_y) as u32}, Block{x: (s_x+rX(0.0, 1.0, self.r)) as u32, y: (s_y+rY(0.0, 1.0, self.r)) as u32}];
            },
            5 => {
                return vec![Block{x: (s_x+rX(-1.5, 0.5, self.r)-0.5) as u32, y: (s_y+rY(-1.5, 0.5, self.r)+0.5) as u32}, Block{x: (s_x+rX(-0.5, 0.5, self.r)-0.5) as u32, y: (s_y+rY(-0.5, 0.5, self.r)+0.5) as u32}, Block{x: (s_x+rX(0.5, 0.5, self.r)-0.5) as u32, y: (s_y+rY(0.5, 0.5, self.r)+0.5) as u32}, Block{x: (s_x+rX(1.5, 0.5, self.r)-0.5) as u32, y: (s_y+rY(1.5, 0.5, self.r)+0.5) as u32}];
            },
            6 => {
                return vec![Block{x: (s_x+rX(-1.0, 1.0, self.r)) as u32, y: (s_y+rY(-1.0, 1.0, self.r)) as u32}, Block{x: (s_x+rX(-1.0, 0.0, self.r)) as u32, y: (s_y+rY(-1.0, 0.0, self.r)) as u32}, Block{x: (s_x) as u32, y: (s_y) as u32}, Block{x: (s_x+rX(1.0, 0.0, self.r)) as u32, y: (s_y+rY(1.0, 0.0, self.r)) as u32}];
            },
            7 => {
                return vec![Block{x: (s_x+rX(-1.0, 0.0, self.r)) as u32, y: (s_y+rY(-1.0, 0.0, self.r)) as u32}, Block{x: (s_x+rX(1.0, 0.0, self.r)) as u32, y: (s_y+rY(1.0, 0.0, self.r)) as u32}, Block{x: (s_x) as u32, y: (s_y) as u32}, Block{x: (s_x+rX(0.0, 1.0, self.r)) as u32, y: (s_y+rY(0.0, 1.0, self.r)) as u32}];
            },
            _ => {return vec![];}
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let colors = [0x000000, 0xEB4F65, 0xF38927, 0xF6D03C, 0x51B84D, 0x42AFE1, 0x1165B5, 0x9739A2];
    let mut board: Vec<u32> = vec![0; 10 * 40];
    let mut queue: Vec<u32> = vec![0; 5];
    let mut bag: Vec<u32> = vec![0; 7];
    for j in 0..7 {
        bag[j] = (j+1) as u32;
    }
    let mut piece: Piece = Piece::new(4, 19, get_piece(&mut bag));

    let mut x: u32 = 0;
    let mut y: u32 = 0;
    for i in buffer.iter_mut() {
        let mut col: u32 = 0;//((((x as f32) / ((WIDTH as f32)/256.0)) as u32) << 16)+(((y as f32) / ((HEIGHT as f32)/256.0)) as u32);

        if (y >= 40 && ((x >= 140 && x < 170) || (x >= 470 && x < 500))) || (x >= 140 && x < 500 && y >= 610) {
            col = 0xffffff;
        }
        *i = col;
        x+=1;
        if (x as usize) >= WIDTH {
            x = 0;
            y += 1;
        }
    }
    

    /*let mut iter: u32 = 1;
    for i in board.iter_mut() {
        *i = iter;
        iter += 1;
        if iter >= colors.len() as u32 {
            iter = 0;
        }
    }*/

    for i in queue.iter_mut() {
        *i = get_piece(&mut bag);
    }

    /*for i in queue.iter_mut() {
        *i = rand::thread_rng().gen_range(1..8);
    }*/

    /*let piece: Piece = Piece::new(0, 19, 1);
    for b in piece.getBlocks().iter() {
        println!("{}, {} vs {}, {}", b.x, b.y, 1, 19);
        if b.x == 1 && b.y == 19 {
            println!("colored!");
        }
    }*/

    let mut window = Window::new(
        "My First Rust Program",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    //window.limit_update_rate(Some(std::time::Duration::from_micros(6944)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        for b in piece.getBlocks().iter() {
            // println!("{}, {}", b.x, b.y);
            for x in ((b.x as i32)*30+170)..((b.x as i32)*30+200) {
                for y in ((19-(b.y as i32))*30+40)..((19-(b.y as i32))*30+70) {
                    buffer[(x+y*(WIDTH as i32)) as usize] = 0;
                }
            }
            //println!("colored! {} at {}, {}", col, x, y);
        }

        //controls
        if window.is_key_pressed(Key::Left, KeyRepeat::No) {
            if(piece.x>0) {
                piece.x-=1;
            }
        }

        if window.is_key_pressed(Key::Right, KeyRepeat::No) {
            if(piece.x<10) {
                piece.x+=1;
            }
        }

        //rendering
        for b in piece.getBlocks().iter() {
            // println!("{}, {}", b.x, b.y);
            for x in ((b.x as i32)*30+170)..((b.x as i32)*30+200) {
                for y in ((19-(b.y as i32))*30+40)..((19-(b.y as i32))*30+70) {
                    buffer[(x+y*(WIDTH as i32)) as usize] = colors[piece.color as usize]
                }
            }
            //println!("colored! {} at {}, {}", col, x, y);
        }
        /*x = 0;
        y = 0;
        for i in buffer.iter_mut() {
            let mut col: u32 = 0;//((((x as f32) / ((WIDTH as f32)/256.0)) as u32) << 16)+(((y as f32) / ((HEIGHT as f32)/256.0)) as u32);

            /*if (y >= 40 && ((x >= 140 && x < 170) || (x >= 470 && x < 500))) || (x >= 140 && x < 500 && y >= 610) {
                col = 0xffffff;
            }*/

            if x >= 170 && x < 470 && y < 610 {
                let b_x: i32 = ((x as i32)-170)/30;
                let b_y: i32 = (599-((y as i32)-10))/30;
                let index: usize = (b_x+b_y*10) as usize;
                col = colors[board[index] as usize];

                //let piece: Piece = Piece::new(p_x, 18-j*3, piece.color);
                for b in piece.getBlocks().iter() {
                    // println!("{}, {}", b.x, b.y);
                    if b.x == b_x as u32 && b.y == b_y as u32 {
                        col = colors[piece.color as usize];
                        //println!("colored! {} at {}, {}", col, x, y);
                        break;
                    }
                }
            }

            if x >= 500 && y >= 40 && y < 610 {
                let q_x: u32 = (((x as i32)-500)/30) as u32;
                let q_y: u32 = ((599-((y as i32)-40))/30) as u32;
                // println!("{}, {}", q_x, q_y);
                'outer: for j in 0..5 {
                    let mut p_x = 1;
                    if queue[j as usize] == 3 || queue[j as usize] == 5 {
                        p_x = 2;
                    }
                    let p: Piece = Piece::new(p_x, 18-j*3, queue[j as usize]);
                    for b in p.getBlocks().iter() {
                        // println!("{}, {}", b.x, b.y);
                        if b.x == q_x && b.y == q_y {
                            col = colors[queue[j as usize] as usize];
                            //println!("colored! {} at {}, {}", col, x, y);
                            break 'outer;
                        }
                    }
                }
            }
            
            if(col != 0) { *i = col; }// write something more funny here!

            //where on screen
            x+=1;
            if (x as usize) >= WIDTH {
                x = 0;
                y += 1;
            }
        }*/

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    
}

fn rX(x:f32, y:f32, r:u32) -> f32 {
    if r==0 {return x;}
    if r==1 {return y;}
    if r==2 {return -x;}
    return -y;
}

fn rY(x:f32, y:f32, r:u32) -> f32 {
    if r==0 {return y;}
    if r==1 {return -x;}
    if r==2 {return -y;}
    return x;
}

fn get_piece (p: &mut Vec<u32>) -> u32 {
    let num = rand::thread_rng().gen_range(0..p.len());
    let new_piece = p[num];
    (*p).remove(num);
    if p.len() == 0 {
        for j in 0..7 {
            p.push((j+1) as u32);
        }
    }
    return new_piece;
}

fn take<T>(vec: Vec<T>, index: usize) -> Option<T> {
    vec.into_iter().nth(index)
}

/*fn render_queue(x:u32, y:u32, q: &Vec<u32>) -> u32 {
    let q_x: u32 = (((x as i32)-500)/30) as u32;
    let q_y: u32 = ((599-((y as i32)-40))/30) as u32;
    // println!("{}, {}", q_x, q_y);
    for j in 0..5 {
        let piece: Piece = Piece::new(1, 19-j*3, q[j as usize]);
        for b in piece.getBlocks().iter() {
            // println!("{}, {}", b.x, b.y);
            if b.x == q_x && b.y == q_y {
                //println!("colored! {} at {}, {}", col, x, y);
                return q[j as usize];
            }
        }
    }
    return 0;
}*/

/*
let mut dir: i32 = 1;
    let mut z: i32 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        for i in buffer.iter_mut() {
            let mut col: u32 = ((((x as f32) / ((WIDTH as f32)/256.0)) as u32) << 16)+((z as u32) << 8)+(((y as f32) / ((HEIGHT as f32)/256.0)) as u32);
            *i = col; // write something more funny here!
            x+=1;
            if (x as usize) >= WIDTH {
                x = 0;
                y += 1;
            }
        }
        z += dir;
        if z == 255 {
            dir = -1;
        } else if z == 0 {
            dir = 1;
        }

*/