use crate::tuple::Tuple;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Tuple>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![vec![Tuple::color(0., 0., 0.); height]; width],
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_pixel_at(&self, x: usize, y: usize) -> Result<&Tuple, String> {
        let color = self
            .pixels
            .get(x)
            .ok_or(format!("x {} does not exist", x))?
            .get(y)
            .ok_or(format!("y {} does not exist", y))?;
        Ok(color)
    }

    pub fn write_pixel_at(&mut self, x: usize, y: usize, color: Tuple) -> Result<(), String> {
        if x >= self.width {
            return Err(format!("x {} out of range", x));
        }
        if y >= self.height {
            return Err(format!("y {} out of range", y));
        }
        self.pixels[x][y] = color;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.get_width(), 10);
        assert_eq!(c.get_height(), 20);

        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(
                    c.get_pixel_at(x, y)
                        .expect("Getting a pixel from canvas should work"),
                    &Tuple::color(0., 0., 0.)
                );
            }
        }
    }

    #[test]
    fn write_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Tuple::color(1., 0., 0.);
        c.write_pixel_at(2, 3, red)
            .expect("Writing to canvas should work");
        assert_eq!(c.get_pixel_at(2, 3).unwrap(), &Tuple::color(1.0, 0., 0.));
    }
}