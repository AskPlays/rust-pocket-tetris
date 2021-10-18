#![allow(non_snake_case)]
//#![windows_subsystem = "windows"]
extern crate minifb;

use minifb::{Key, Window, WindowOptions, KeyRepeat};
use rand::Rng;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

const OTHER_SPINS:[[(i32,i32); 4]; 8] = [
    [(-1, 0),(-1, 1),(0,-2),(-1,-2)],
    [( 1, 0),( 1,-1),(0, 2),( 1, 2)],
    [( 1, 0),( 1,-1),(0, 2),( 1, 2)],
    [(-1, 0),(-1, 1),(0,-2),(-1,-2)],
    [( 1, 0),( 1, 1),(0,-2),( 1,-2)],
    [(-1, 0),(-1,-1),(0, 2),(-1, 2)],
    [(-1, 0),(-1,-1),(0, 2),(-1, 2)],
    [( 1, 0),( 1, 1),(0,-2),( 1,-2)],
];

const I_SPINS:[[(i32,i32); 4]; 8]  = [
    [(-2, 0),( 1, 0),(-2,-1),( 1, 2)],
    [( 2, 0),(-1, 0),( 2, 1),(-1,-2)],
    [(-1, 0),( 2, 0),(-1, 2),( 2,-1)],
    [( 1, 0),(-2, 0),( 1,-2),(-2, 1)],
    [( 2, 0),(-1, 0),( 2, 1),(-1,-2)],
    [(-2, 0),( 1, 0),(-2,-1),( 1, 2)],
    [( 1, 0),(-2, 0),( 1,-2),(-2, 1)],
    [(-1, 0),( 2, 0),(-1, 2),( 2,-1)],
];

const SPINS180:[(i32,i32); 4] = [
    ( 0, 1),
    ( 1, 0),
    ( 0,-1),
    (-1, 0)
];

struct Block {
    x:i32,
    y:i32,
}

struct Score {
    score: i32,
    name: String,
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
                return vec![Block{x: (s_x+rX(-1.0, 1.0, self.r)) as i32, y: (s_y+rY(-1.0, 1.0, self.r)) as i32}, Block{x: (s_x+rX(1.0, 0.0, self.r)) as i32, y: (s_y+rY(1.0, 0.0, self.r)) as i32}, Block{x: (s_x) as i32, y: (s_y) as i32}, Block{x: (s_x+rX(0.0, 1.0, self.r)) as i32, y: (s_y+rY(0.0, 1.0, self.r)) as i32}];
            },
            2 => {
                return vec![Block{x: (s_x+rX(1.0, 1.0, self.r)) as i32, y: (s_y+rY(1.0, 1.0, self.r)) as i32}, Block{x: (s_x+rX(-1.0, 0.0, self.r)) as i32, y: (s_y+rY(-1.0, 0.0, self.r)) as i32}, Block{x: (s_x) as i32, y: (s_y) as i32}, Block{x: (s_x+rX(1.0, 0.0, self.r)) as i32, y: (s_y+rY(1.0, 0.0, self.r)) as i32}];
            },
            3 => {
                return vec![Block{x: (s_x+rX(-0.5, 0.5, self.r)-0.5) as i32, y: (s_y+rY(-0.5, 0.5, self.r)+0.5) as i32}, Block{x: (s_x+rX(-0.5, -0.5, self.r)-0.5) as i32, y: (s_y+rY(-0.5, -0.5, self.r)+0.5) as i32}, Block{x: (s_x+rX(0.5, -0.5, self.r)-0.5) as i32, y: (s_y+rY(0.5, -0.5, self.r)+0.5) as i32}, Block{x: (s_x+rX(0.5, 0.5, self.r)-0.5) as i32, y: (s_y+rY(0.5, 0.5, self.r)+0.5) as i32}];
            },
            4 => {
                return vec![Block{x: (s_x+rX(1.0, 1.0, self.r)) as i32, y: (s_y+rY(1.0, 1.0, self.r)) as i32}, Block{x: (s_x+rX(-1.0, 0.0, self.r)) as i32, y: (s_y+rY(-1.0, 0.0, self.r)) as i32}, Block{x: (s_x) as i32, y: (s_y) as i32}, Block{x: (s_x+rX(0.0, 1.0, self.r)) as i32, y: (s_y+rY(0.0, 1.0, self.r)) as i32}];
            },
            5 => {
                return vec![Block{x: (s_x+rX(-1.5, 0.5, self.r)-0.5) as i32, y: (s_y+rY(-1.5, 0.5, self.r)+0.5) as i32}, Block{x: (s_x+rX(-0.5, 0.5, self.r)-0.5) as i32, y: (s_y+rY(-0.5, 0.5, self.r)+0.5) as i32}, Block{x: (s_x+rX(0.5, 0.5, self.r)-0.5) as i32, y: (s_y+rY(0.5, 0.5, self.r)+0.5) as i32}, Block{x: (s_x+rX(1.5, 0.5, self.r)-0.5) as i32, y: (s_y+rY(1.5, 0.5, self.r)+0.5) as i32}];
            },
            6 => {
                return vec![Block{x: (s_x+rX(-1.0, 1.0, self.r)) as i32, y: (s_y+rY(-1.0, 1.0, self.r)) as i32}, Block{x: (s_x+rX(-1.0, 0.0, self.r)) as i32, y: (s_y+rY(-1.0, 0.0, self.r)) as i32}, Block{x: (s_x) as i32, y: (s_y) as i32}, Block{x: (s_x+rX(1.0, 0.0, self.r)) as i32, y: (s_y+rY(1.0, 0.0, self.r)) as i32}];
            },
            7 => {
                return vec![Block{x: (s_x+rX(-1.0, 0.0, self.r)) as i32, y: (s_y+rY(-1.0, 0.0, self.r)) as i32}, Block{x: (s_x+rX(1.0, 0.0, self.r)) as i32, y: (s_y+rY(1.0, 0.0, self.r)) as i32}, Block{x: (s_x) as i32, y: (s_y) as i32}, Block{x: (s_x+rX(0.0, 1.0, self.r)) as i32, y: (s_y+rY(0.0, 1.0, self.r)) as i32}];
            },
            _ => {return vec![];}
        }
    }
    fn collide(&self, c_x: i32, c_y: i32, brd: &Vec<u32>) -> bool {
        for b in self.getBlocks().iter() {
            let b_x: i32 = b.x;
            let b_y: i32 = b.y;
            if b_x+c_x < 0 || b_x+c_x > 9 || b_y+c_y < 0 || b_y+c_y > 39 || brd[(b_x+c_x+(b_y+c_y)*10) as usize] != 0 {
                return true;
            }
        }
        return false;
    }

    fn collideT(&mut self, c: (i32, i32), brd: &Vec<u32>) -> bool {
        for b in self.getBlocks().iter() {
            let b_x: i32 = b.x;
            let b_y: i32 = b.y;
            if b_x+c.0 < 0 || b_x+c.0 > 9 || b_y+c.1 < 0 || b_y+c.1 > 39 || brd[(b_x+c.0+(b_y+c.1)*10) as usize] != 0 {
                return true;
            }
        }
        self.x += c.0;
        self.y += c.1;
        return false;
    }

    fn collideSRS(&mut self, prev_r:u32, brd: &Vec<u32>) -> bool {
        let mut collided:bool = true;
        if collided { collided = self.collide(0, 0, &brd); }
        if collided {
            let aR = prev_r as i32;
            let bR = self.r as i32;
            if (bR-aR).abs() == 2i32 {
                collided = self.collideT(SPINS180[aR as usize], &brd);
            } else {
                for i in 0..4 {
                    if self.color == 5 {
                        if aR==0 && bR==1 { collided = self.collideT(I_SPINS[0][i], &brd); }
                        if aR==1 && bR==0 { collided = self.collideT(I_SPINS[1][i], &brd); }
                        if aR==1 && bR==2 { collided = self.collideT(I_SPINS[2][i], &brd); }
                        if aR==2 && bR==1 { collided = self.collideT(I_SPINS[3][i], &brd); }
                        if aR==2 && bR==3 { collided = self.collideT(I_SPINS[4][i], &brd); }
                        if aR==3 && bR==2 { collided = self.collideT(I_SPINS[5][i], &brd); }
                        if aR==3 && bR==0 { collided = self.collideT(I_SPINS[6][i], &brd); }
                        if aR==0 && bR==3 { collided = self.collideT(I_SPINS[7][i], &brd); }
                    } else {
                        if aR==0 && bR==1 { collided = self.collideT(OTHER_SPINS[0][i], &brd); }
                        if aR==1 && bR==0 { collided = self.collideT(OTHER_SPINS[1][i], &brd); }
                        if aR==1 && bR==2 { collided = self.collideT(OTHER_SPINS[2][i], &brd); }
                        if aR==2 && bR==1 { collided = self.collideT(OTHER_SPINS[3][i], &brd); }
                        if aR==2 && bR==3 { collided = self.collideT(OTHER_SPINS[4][i], &brd); }
                        if aR==3 && bR==2 { collided = self.collideT(OTHER_SPINS[5][i], &brd); }
                        if aR==3 && bR==0 { collided = self.collideT(OTHER_SPINS[6][i], &brd); }
                        if aR==0 && bR==3 { collided = self.collideT(OTHER_SPINS[7][i], &brd); }
                    }
                    if !collided { break; }
                }
            }
        }
        return collided;
    }

    fn set_block(&self, brd: &mut Vec<u32>) {
        for b in self.getBlocks().iter() {
            // let b_x: u32 = b.x as u32;
            // let b_y: u32 = b.y as u32;
            (*brd)[(b.x+b.y*10) as usize] = self.color;
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    const COLORS: [u32; 8] = [0x000000, 0xEB4F65, 0xF38927, 0xF6D03C, 0x51B84D, 0x42AFE1, 0x1165B5, 0x9739A2];
    const HOLD_COLORS: [u32; 8] = [0x000000, 0x762833, 0x7a4514, 0x7b681e, 0x295c27, 0x215871, 0x09335b, 0x4c1d51];
    let mut board: Vec<u32> = vec![0; 10 * 40];
    let mut queue: Vec<u32> = vec![0; 5];
    let mut bag: Vec<u32> = vec![0; 7];
    for j in 0..7 {
        bag[j] = (j+1) as u32;
    }
    let mut piece: Piece = Piece::new(4, 19, get_piece(&mut bag));
    if piece.color == 3 || piece.color == 5 {
        piece.x = 5;
    }
    let mut hold: u32 = 0;
    let mut held = false;
    for i in queue.iter_mut() {
        *i = get_piece(&mut bag);
    }
    let mut gameover = false;
    let mut now = std::time::Instant::now();
    let mut keys = [0; 3];
    let mut level = 1;
    let mut lines_cleared = 0;
    let mut soft_drop = 0;
    let mut soft_drop_rate = 20f64;
    let mut lock_delay = 0;
    let mut manipulations = 0;
    let das = 100;
    let mut arr = 10;

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
    // Limit to max ~144 fps update rate
    //window.limit_update_rate(Some(std::time::Duration::from_micros(6944)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let delta = now.elapsed().as_millis();
        now = std::time::Instant::now();

        //controls
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            board = vec![0; 10 * 40];
            queue = vec![0; 5];
            bag = vec![0; 7];
            for j in 0..7 {
                bag[j] = (j+1) as u32;
            }
            piece = Piece::new(4, 19, get_piece(&mut bag));
            if piece.color == 3 || piece.color == 5 {
                piece.x = 5;
            }
            hold = 0;
            for i in queue.iter_mut() {
                *i = get_piece(&mut bag);
            }
            gameover = false;
            soft_drop = 0;
            lock_delay = 0;
            manipulations = 0;
            held = false;
            lines_cleared = 0;
        }

        if window.is_key_pressed(Key::Q, KeyRepeat::No) {
            soft_drop_rate = 0f64;
            arr = 0;
        }

        if gameover == false {
            let gravity = (0.8 - (level as f64 - 1f64) * 0.007).powf((level - 1) as f64);
            if keys[0] > 0 {
                while keys[0] > (gravity*1000f64/soft_drop_rate) as u128 && !piece.collide(0, -1, &board) {
                    if piece.y>-1 && !piece.collide(0, -1, &board) && manipulations < 15 {
                        piece.y-=1;
                        soft_drop = 0;
                        lock_delay = 0;
                    }
                    keys[0] -= (gravity*1000f64/soft_drop_rate) as u128;
                }
                keys[0] += delta;
            }
            if keys[1] > 0 {
                while keys[1] > das && !piece.collide(-1, 0, &board) {
                    if !piece.collide(-1, 0, &board) && manipulations < 15 {
                        piece.x-=1;
                        lock_delay = 0;
                    }
                    keys[1] -= arr;
                }
                keys[1] += delta;
            }
            if keys[2] > 0 {
                while keys[2] > das && !piece.collide(1, 0, &board) {
                    if !piece.collide(1, 0, &board) && manipulations < 15 {
                        piece.x+=1;
                        lock_delay = 0;
                    }
                    keys[2] -= arr;
                }
                keys[2] += delta;
            }

            if window.is_key_pressed(Key::Down, KeyRepeat::No) && manipulations < 15 {
                if piece.y>-1 && !piece.collide(0, -1, &board) {
                    piece.y-=1;
                    soft_drop = 0;
                    if lock_delay != 0 { manipulations+=1; }
                    lock_delay = 0;
                }
                while soft_drop_rate == 0f64 && !piece.collide(0, -1, &board) {
                    piece.y-=1;
                    soft_drop = 0;
                    lock_delay = 0;
                }
                keys[0] = 1;
            }
            if window.is_key_pressed(Key::Left, KeyRepeat::No) && manipulations < 15 {
                if piece.x>-1 && !piece.collide(-1, 0, &board) {
                    piece.x-=1;
                    if lock_delay != 0 { manipulations+=1; }
                    lock_delay = 0;
                }
                keys[1] = 1;
            }
    
            if window.is_key_pressed(Key::Right, KeyRepeat::No) && manipulations < 15 {
                if piece.x<11 && !piece.collide(1, 0, &board) {
                    piece.x+=1;
                    if lock_delay != 0 { manipulations+=1; }
                    lock_delay = 0;
                }
                keys[2] = 1;
            }
    
            if (window.is_key_pressed(Key::Up, KeyRepeat::No) || window.is_key_pressed(Key::D, KeyRepeat::No) || window.is_key_pressed(Key::C, KeyRepeat::No)) && manipulations < 15  {
                let prev_r = piece.r.clone();
                piece.r = (piece.r+1)%4;
                if piece.collideSRS(prev_r, &board) {piece.r = (piece.r+3)%4}
                else { if lock_delay != 0 { manipulations+=1; }
                lock_delay = 0; }
            }
    
            if (window.is_key_pressed(Key::A, KeyRepeat::No) || window.is_key_pressed(Key::Z, KeyRepeat::No)) && manipulations < 15 {
                let prev_r = piece.r.clone();
                piece.r = (piece.r+3)%4;
                if piece.collideSRS(prev_r, &board) {piece.r = (piece.r+1)%4}
                else { if lock_delay != 0 { manipulations+=1; }
                lock_delay = 0; }
            }

            if (window.is_key_pressed(Key::W, KeyRepeat::No) || window.is_key_pressed(Key::X, KeyRepeat::No)) && manipulations < 15 {
                let prev_r = piece.r.clone();
                piece.r = (piece.r+2)%4;
                if piece.collideSRS(prev_r, &board) {piece.r = (piece.r+2)%4}
                else { if lock_delay != 0 { manipulations+=1; }
                lock_delay = 0; }
            }

            if window.is_key_pressed(Key::LeftShift, KeyRepeat::No) && !held {
                if hold == 0 {
                    let piece_color:u32 = piece.color.clone();
                    hold = piece_color;
                    // piece = Piece::new(4, 19, get_piece(&mut bag));
                    piece = Piece::new(4, 19, queue[0]);
                    if piece.color == 3 || piece.color == 5 {
                        piece.x = 5;
                    }
                    queue.remove(0);
                    queue.insert(4, get_piece(&mut bag));
                } else {
                    let hold_color:u32 = hold.clone();
                    hold = piece.color.clone();
                    // piece = Piece::new(4, 19, get_piece(&mut bag));
                    piece = Piece::new(4, 19, hold_color);
                    if piece.color == 3 || piece.color == 5 {
                        piece.x = 5;
                    }
                }
                if piece.collide(0,0, &board) {
                    gameover = true;
                }
                held = true;
            }
    
            if window.is_key_pressed(Key::Space, KeyRepeat::No) {
                while piece.y>-1 && !piece.collide(0, -1, &board) {
                    piece.y-=1;
                }
                /*piece.set_block(&mut board);
                piece = Piece::new(4, 19, queue[0]);
                queue.remove(0);
                queue.insert(4, get_piece(&mut bag));*/
                place_piece(&mut board, &mut piece, &mut queue, &mut bag);
                if piece.collide(0,0, &board) {
                    gameover = true;
                }
                let s:Score = placed_piece(&mut board);
                if s.score > 0 { println!("Score: {}, move: {}", s.score, s.name); }
                lines_cleared += s.score;
                if lines_cleared >= 10 {
                    lines_cleared -= 10;
                    level += 1;
                    println!("Level Up! Level: {}", level);
                }
                soft_drop = 0;
                lock_delay = 0;
                manipulations = 0;
                held = false;
            }

            if !piece.collide(0, -1, &board) { 
                while soft_drop > (gravity*1000f64) as u128 && !piece.collide(0, -1, &board) {
                    piece.y-=1;
                    soft_drop -= (gravity*1000f64) as u128;
                }
            } else {
                if lock_delay > 500 {
                    place_piece(&mut board, &mut piece, &mut queue, &mut bag);
                    if piece.collide(0,0, &board) {
                        gameover = true;
                    }
                    let s:Score = placed_piece(&mut board);
                    if s.score > 0 { println!("Score: {}, move: {}", s.score, s.name); }
                    lines_cleared += s.score;
                    if lines_cleared >= 10 {
                        lines_cleared -= 10;
                        level += 1;
                        println!("Level Up! Level: {}", level);
                    }
                    soft_drop = 0;
                    lock_delay = 0;
                    manipulations = 0;
                    held = false;
                }
            }

            soft_drop += delta;
            if piece.collide(0, -1, &board) {
                lock_delay += delta;
                soft_drop = 0;
            }
        }

        if window.is_key_released(Key::Down) {
            keys[0] = 0;
        }
        if window.is_key_released(Key::Left) {
            keys[1] = 0;
        }
        if window.is_key_released(Key::Right) {
            keys[2] = 0;
        }

        //rendering
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        /*x = 0;
        y = 0;*/
        for i in buffer.iter_mut() {
            let mut col: u32 = 0;//((((x as f32) / ((WIDTH as f32)/256.0)) as u32) << 16)+(((y as f32) / ((HEIGHT as f32)/256.0)) as u32);

            if (y >= 10 && ((x >= 140 && x < 170) || (x >= 470 && x < 500))) || (x >= 140 && x < 500 && y >= 610) {
                col = 0xffffff;
            }

            if x >= 170 && x < 470 && y < 610 {
                if (x+11)%30 < 2 || (y+21)%30 < 2 {
                    col = 0xbfbfbf;
                }
                let b_x: i32 = ((x as i32)-170)/30;
                let b_y: i32 = (599-((y as i32)-10))/30;
                let index: usize = (b_x+b_y*10) as usize;
                if board[index] != 0 { col = COLORS[board[index] as usize]; }
            }

            if x >= 20 && x < 140 && y >= 40 && y < 160 {
                let b_x: i32 = ((x as i32)-20)/30;
                let b_y: i32 = (599-((y as i32)-40))/30;

                let mut p_x = 1;
                if hold == 3 || hold == 5 {
                    p_x = 2;
                }
                let hold_piece: Piece = Piece::new(p_x, 18, hold);
                for b in hold_piece.getBlocks().iter() {
                    // println!("{}, {}", b.x, b.y);
                    if b.x == b_x && b.y == b_y {
                        col = COLORS[hold as usize];
                        //println!("colored! {} at {}, {}", col, x, y);
                        break;
                    }
                }
            }

            if x >= 500 && y >= 40 && y < 610 {
                let q_x: i32 = ((x as i32)-500)/30;
                let q_y: i32 = (599-((y as i32)-40))/30;
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
                            col = COLORS[queue[j as usize] as usize];
                            //println!("colored! {} at {}, {}", col, x, y);
                            break 'outer;
                        }
                    }
                }
            }
            
            *i = col;// write something more funny here!

            //where on screen
            x+=1;
            if (x as usize) >= WIDTH {
                x = 0;
                y += 1;
            }
        }

        //hold rendering
        let mut p: Piece = Piece::new(piece.x, piece.y, piece.color);
        p.r = piece.r;
        while p.y>-1 && !p.collide(0, -1, &board) {
            p.y-=1;
        }
        for b in p.getBlocks().iter() {
            if b.x >= 0 && b.x < 10 && b.y >= 0 && b.y < 40 {
                for x in ((b.x as i32)*30+170)..((b.x as i32)*30+200) {
                    for y in ((19-(b.y as i32))*30+10)..((19-(b.y as i32))*30+40) {
                        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 { buffer[(x+y*(WIDTH as i32)) as usize] = HOLD_COLORS[piece.color as usize]; }
                    }
                }
            }
        }

        //piece rendering
        for b in piece.getBlocks().iter() {
            if b.x >= 0 && b.x < 10 && b.y >= 0 && b.y < 40 {
                for x in ((b.x as i32)*30+170)..((b.x as i32)*30+200) {
                    for y in ((19-(b.y as i32))*30+10)..((19-(b.y as i32))*30+40) {
                        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 { buffer[(x+y*(WIDTH as i32)) as usize] = COLORS[piece.color as usize] }
                    }
                }
            }
        } 

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

fn place_piece(brd: &mut Vec<u32>, piece: &mut Piece, queue: &mut Vec<u32>, bag: &mut Vec<u32>) {
    piece.set_block(brd);
    let mut p_x = 4;
    if queue[0] == 3 || queue[0] == 5 {
        p_x = 5;
    }
    *piece = Piece::new(p_x, 19, queue[0]);
    queue.remove(0);
    queue.insert(4, get_piece(bag));
}

fn placed_piece(brd: &mut Vec<u32>) -> Score {
    let mut line_clears = 0;
    let mut y: i32 = 0;
    while y < 20 {
        let mut pieces = 0;
        for x in 0..10 {
            if brd[(x+y*(10 as i32)) as usize] != 0 {
                pieces += 1;
            }
        }
        if pieces == 10 {
            for k in y..20 {
                for x in 0..10 {
                    if k != 19 {
                        brd[(x+k*(10 as i32)) as usize] = brd[(x+(k+1)*(10 as i32)) as usize];
                    } else {
                        brd[(x+k*(10 as i32)) as usize] = 0;
                    }
                }
            }
            line_clears += 1;
            y -= 1;
        }
        y += 1;
    }
    let mut name = String::new();
    if line_clears == 1 {name += "single"}
    if line_clears == 2 {name += "double"}
    if line_clears == 3 {name += "triple"}
    if line_clears == 4 {name += "quad"}
    return Score{
        score: line_clears,
        name: name
    }
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

*//*use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

struct MyGame {
    // Your game state and assets go here...
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| MyGame { /* ... */ })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!
    }
}*/