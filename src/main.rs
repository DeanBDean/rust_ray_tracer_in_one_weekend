use std::usize;

fn main() {
  let number_of_x_pixels = 200;
  let number_of_y_pixels = 100;
  println!("P3\n{} {}\n255", number_of_x_pixels, number_of_y_pixels);
  (0 .. number_of_y_pixels).rev().for_each(|current_y_pixel| {
    (0 .. number_of_x_pixels).for_each(|current_x_pixel| {
      let red_value = current_x_pixel as f32 / number_of_x_pixels as f32;
      let green_value = current_y_pixel as f32 / number_of_y_pixels as f32;
      let blue_value = 0.2;
      let red_value = (255.99 * red_value) as usize;
      let green_value = (255.99 * green_value) as usize;
      let blue_value = (255.99 * blue_value) as usize;
      println!("{} {} {}", red_value, green_value, blue_value);
    })
  })
}
