use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use macroquad::color::{BLACK, Color};
use macroquad::texture::Image;

type Job = Box<dyn FnOnce() -> (u32, u32, Color) + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<Job>>,
    res_receiver: Option<mpsc::Receiver<(u32, u32, Color)>>,
}

impl ThreadPool {
    pub fn new(num_threads: u32) -> Self {
        let mut threads = Vec::with_capacity(num_threads as usize);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let (res_sender, res_receiver) = mpsc::channel();

        for _ in 0..num_threads {
            let rec = Arc::clone(&receiver);
            let res_sender = res_sender.clone();
            threads.push(thread::spawn(move || {
                loop {
                    let job = match rec.lock().unwrap().recv() {
                        Ok(job) => job,
                        Err(_) => break,
                    };
                    res_sender.send(job()).unwrap();
                }
            }));
        }
        Self {
            threads,
            sender: Some(sender),
            res_receiver: Some(res_receiver),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> (u32, u32, Color) + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = self.sender.as_ref() {
            sender.send(job).unwrap();
        }
    }

    pub fn gather(&mut self, width: u16, height: u16) -> Image {
        let mut image = Image::gen_image_color(width, height, BLACK);
        let total_pixels = (width as u32) * (height as u32);

        if let Some(receiver) = self.res_receiver.take() {
            for idx in 0..total_pixels {
                let (x, y, color) = receiver.recv().unwrap();
                image.set_pixel(x as u32, y as u32, color);
                if idx % 10_000 == 0 {
                    println!("Solved {idx} / {total_pixels}");
                }
            }
        }

        image
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for thread in &mut self.threads.drain(..) {
            thread.join().unwrap();
        }
    }
}
