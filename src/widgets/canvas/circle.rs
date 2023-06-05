use crate::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

/// Shape to draw a circle with a given center and radius and with the given color
#[derive(Debug, Clone)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
}

impl Shape for Circle {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        for angle in 0..360 {
            let radians = f64::from(angle).to_radians();
            let circle_x = self.radius.mul_add(radians.cos(), self.x);
            let circle_y = self.radius.mul_add(radians.sin(), self.y);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        buffer::Buffer,
        layout::Rect,
        style::Color,
        symbols::Marker,
        widgets::{
            canvas::{Canvas, Circle},
            Widget,
        },
    };

    #[test]
    fn test_it_draws_a_circle() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 5.0,
                    y: 2.0,
                    radius: 5.0,
                    color: Color::Reset,
                });
            })
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0]);
        canvas.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "     ⢀⣠⢤⣀ ",
            "    ⢰⠋  ⠈⣇",
            "    ⠘⣆⡀ ⣠⠇",
            "      ⠉⠉⠁ ",
            "          ",
        ]);
        assert_eq!(buffer, expected);
    }
}
