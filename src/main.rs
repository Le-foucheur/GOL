use core::fmt;
use std::{io::{stdout, Write}, ops::{Index, IndexMut}, thread::sleep, time::Duration};
use libc::{exit};
use terminal_size::{terminal_size, Width, Height};
use ctrlc;

const TEMPS: Duration = Duration::from_millis(0);

#[derive(Clone, Debug)]
struct Matrice<T> {
    largeur: usize,
    hauteur: usize,
    tableau: Vec<T>
}  

impl<T: Clone> Matrice<T> {
    fn new (largeur: usize, hauteur: usize, val_init: T) -> Self {
        Self {
            largeur: largeur,
            hauteur: 2 * hauteur,
            tableau: vec![val_init; largeur * 2 * hauteur],
        }
    }
}

impl<T> Index<(usize, usize)> for Matrice<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x,y) = index;
        &self.tableau[self.largeur * y + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrice<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x,y) = index;
        &mut self.tableau[self.largeur * y + x]
    }
}

impl fmt::Display for Matrice<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\x1b[1;1H")?;
        for y in (0..self.hauteur).step_by(2) {
            for x in 0..self.largeur {
                match (self[(x,y)], self[(x,y + 1)]) {
                (true, true) => write!(f, "█")?,
                (true, false) => write!(f, "▀")?,
                (false, true) => write!(f, "▄")?,
                (false, false) => write!(f, " ")?,
                }
            }
        }
        Ok(())
    }
}

fn coovois (i: isize, x: isize, y:isize) -> (isize, isize) {
    (x + (i%3) - 1, y + ((i/3)%3) - 1)
}

fn tempsp1 (mat:&mut Matrice<bool>) -> () {
    let tmp = mat.clone();
    for x in 0..tmp.largeur {
        for y in 0..tmp.hauteur {
             let mut vivant: i8 = 0;
            for i in 0..9 {
                if i != 4 {
                    let (x,y) = coovois(i, x as isize, y as isize);
                    if x < 0 || x as usize >= tmp.largeur || y < 0 || y as usize >= tmp.hauteur {
                        vivant +=1
                    }
                    else if tmp[(x as usize, y as usize)] {
                        vivant += 1
                    }
                }
            }
            if tmp[(x,y)] && vivant > 3 || vivant < 2{
                mat[(x,y)] = false;
            } else if !tmp[(x,y)] && vivant == 3 {
                mat[(x,y)] = true;
            }
        }
    }
}

fn main () {
    print!("\x1b[?47h\x1b[?1049h\x1b[?25l");
    ctrlc::set_handler(move || {print!("\x1b[?1049l\x1b[?25h\x1b[?47l");let _ = stdout().flush(); unsafe { exit(0) }; }).expect("T’Con");
    let (Width(w),Height(h)) = terminal_size().unwrap();
    let w:usize = w.into();
    let h:usize = h.into();

    let mut a = Matrice::new(w, h, false);

    a[(w/2, h)] = true;
    a[(w/2 - 1, h)] = true;
    a[(w/2 + 1, h)] = true;
    
    a[(1,0)] = true;
    a[(2,1)] = true;
    a[(0,2)] = true;
    a[(1,2)] = true;
    a[(2,2)] = true;
    
    a[(1 + 96,0)] = true;
    a[(2 + 96,1)] = true;
    a[(0 + 96,2)] = true;
    a[(1 + 96,2)] = true;
    a[(2 + 96,2)] = true;


    loop {
        let now = std::time::Instant::now();
        print!("{a}");
        tempsp1(&mut a);
        let dif = now.elapsed();
        if dif <= TEMPS {
            sleep(TEMPS - dif);
        }
    }


}
