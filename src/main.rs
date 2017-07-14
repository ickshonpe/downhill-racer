extern crate piston_window;
extern crate rand;
extern crate find_folder;
extern crate opengl_graphics;

mod utility;
mod gl_version;


use piston_window::*;
//use opengl_graphics::*;

fn main() {
   let texture_filenames = vec![
      (Tile::Empty, "empty.png"),
      (Tile::Player, "man.png"),
      (Tile::Tree, "tree.png"),
      (Tile::Path, "path.png"),
      (Tile::Boulder, "boulder.png"),
      (Tile::Flag, "flag.png"),
      (Tile::Finish, "finish.png")];
   let tile_size = (16, 16);
   let mut course = generate_course(30, 500, 0.1, 0.01);
   let mut player_crashed = false;
   let mut course_finished = false;
   let mut player_x = (course.x_len / 2) + 1;
   let mut player_y = 0;
   let mut player_y_exact = player_y as f64;
   let mut player_speed = 0.1;
   let mut player_score = 0;
   let mut acceleration = 0.001;
   course.set(player_x, player_y, Tile::Player);
   let mut player_direction = Direction::Straight;

   let window_size = [tile_size.0 * course.x_len() as u32, 900];
   let mut window: PistonWindow =
      WindowSettings::new("Downhill Racer", window_size)
         .fullscreen(false)
         .exit_on_esc(true)
         .vsync(false)
         .build()
         .unwrap();
   let mut textures = std::collections::HashMap::new();
   for (tile, file) in texture_filenames {
      let texture_file_path = utility::find_asset(file);
      let new_texture =
         Texture::from_path(
            &mut window.factory,
            texture_file_path,
            Flip::None,
            &TextureSettings::new()).unwrap();
      textures.insert(tile, new_texture);
   }
   let factory = window.factory.clone();
   let font_path = utility::find_asset("font.ttf");
   let mut glyphs = Glyphs::new(font_path, factory).unwrap();
   while let Some(e) = window.next() {
      if let Input::Update(args) = e {
         if !course_finished && !player_crashed {
            player_y_exact += if player_speed < 1.0 { player_speed  } else { 1.0 };
            player_speed += acceleration;
            if player_y as f64 + 1.0 < player_y_exact {
               course.set(player_x, player_y, Tile::Path);
               player_x = match player_direction {
                  Direction::Straight => player_x,
                  Direction::Left => player_x - 1,
                  Direction::Right => player_x + 1
               };
               player_y += 1;
               let target_tile = course.get(player_x, player_y);
               match target_tile {
                  Tile::Empty | Tile::Path => {
                     course.set(player_x, player_y, Tile::Player);
                     player_score += 1;
                  }
                  Tile::Finish => {
                     course.set(player_x, player_y, Tile::Player);
                     course_finished = true;
                     player_score += course.y_len();
                  }
                  Tile::Flag => {
                     course.set(player_x, player_y, Tile::Player);
                     player_score += 5;
                  }
                  Tile::Tree | Tile:: Boulder => {
                     course.set(player_x, player_y, Tile::Player);
                     player_crashed = true;
                  }
                  _ => {
                     panic!("Player hit player?!");
                  }
               }
            }
         }
      }
      if let Input::Render(args) = e {
         window.draw_2d(&e, |context, g2d| {
            let clear_color = [0.1, 0.4, 0.1, 1.0];
            clear(clear_color, g2d);
            for x in 0..course.x_len {
               for y in 0..course.y_len {
                  let tile = course.get(x, y);
                  let texture = textures.get(&tile).unwrap();
                  let size = texture.get_size();
                  let transform = context.transform;
                  let mut transform = transform.trans((size.0 * x as u32) as f64, (size.0 * y as u32) as f64);
                  if player_y_exact > 4.0 {
                     transform = transform.trans(0.0, -(player_y_exact - 4.0) * tile_size.1 as f64);
                  }
                  image(texture, transform, g2d);
               }
            }
         });      }
      if let Input::Render(args) = e {
         window.draw_2d(&e, |context, g2d| {
            let text_color = [1.0, 0.0, 0.0, 1.0];
            let text_context = context.trans(context.viewport.unwrap().window_size[0] as f64 - 100.0, 20.0);
            let score_string = std::fmt::format(format_args!("SCORE: {} ", player_score));
            text::Text::new_color(text_color, 16).draw(
               &score_string,
               &mut glyphs,
               &text_context.draw_state,
               text_context.transform,
               g2d);
         });
      }
      if let Input::Close(_) = e {
         window.set_should_close(true);
      }
      if let Input::Press(Button::Keyboard(k)) = e {
         match k {
            Key::Left | Key::NumPad4 | Key::A => {
               player_direction = Direction::Left;
            }
            Key::Right | Key::NumPad6 | Key::D => {
               player_direction = Direction::Right;
            }
            _ => {}
         }
      }
      if let Input::Release(Button::Keyboard(k)) = e {
         match k {
            Key::Left | Key::NumPad4 | Key::A => {
               player_direction = Direction::Straight
            }
            Key::Right | Key::NumPad6 | Key::D => {
               player_direction = Direction::Straight
            }
            _ => {}
         }
      }
   }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
   Flag,
   Tree,
   Path,
   Boulder,
   Empty,
   Player,
   Finish
}

enum Direction {
   Left,
   Straight,
   Right
}

struct Course {
   data: Vec<Vec<Tile>>,
   x_len: usize,
   y_len: usize
}

impl Course {
   pub fn new(x_len: usize, y_len: usize) -> Course {
      Course {
         data: vec![vec![Tile::Empty; y_len]; x_len],
         x_len,
         y_len
      }
   }
   #[inline]
   pub fn x_len(&self) -> usize {
      self.x_len
   }
   pub fn y_len(&self) -> usize {
      self.y_len
   }
   pub fn get(&self, x: usize, y: usize) -> Tile {
      self.data[x][y]
   }
   pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
      self.data[x][y] = tile;
   }
   pub fn tile_count(&self) -> usize {
      self.x_len * self.y_len
   }
   pub fn selecct_random_tile(&self) -> (usize, usize) {
      (utility::select_random_in_range(0, self.x_len()),
       utility::select_random_in_range(0, self.y_len()))
   }
}

fn generate_course(course_x_len: usize, course_y_len: usize, tree_density: f64, flag_density: f64) -> Course {
   let mut course = Course::new(course_x_len, course_y_len);
   let trees_max = ((course.tile_count() as f64) * tree_density) as u64;
   let flag_count = ((course.tile_count() as f64) * flag_density) as u64;
   let mut path_x = 1 + (course.x_len() / 2);
   let path_min_x = 3;
   let path_max_x = course.x_len() - path_min_x - 1;
   for y in 0..course.y_len() {
      course.set(0, y, Tile::Tree);
      let max_x = course.x_len() - 1;
      course.set(max_x, y, Tile::Tree);
      let path_change = utility::select_random_3(0, 1, 2);
      path_x = path_x + path_change - 1;
      if path_x < path_min_x { path_x = path_min_x };
      if path_x > path_max_x { path_x = path_max_x };
      course.set(path_x, y, Tile::Path);
      course.set(path_x - 1, y, Tile::Path);
      course.set(path_x + 1, y, Tile::Path);
   }
   for i in 0..flag_count {
      let (x, y) = course.selecct_random_tile();
      if course.get(x, y) == Tile::Empty {
         course.set(x, y, Tile::Flag);
      }
   }
   for i in 0..trees_max {
      let (x, y) = course.selecct_random_tile();
      if course.get(x, y) == Tile::Empty {
         course.set(x, y, Tile::Tree);
      }
   }
   for x in 0..course.x_len() {
      let y = course.y_len() - 1;
      course.set(x, y, Tile::Finish);
   }
   course
}

fn print_course(course: &Course) {
   for y in 0..course.y_len() {
      for x in 0..course.x_len() {
         let tile = course.get(x, y);
         let glyph = match tile {
            Tile::Tree => 'T',
            Tile::Empty => '.',
            Tile::Path => '_',
            _ => '?'
         };
         print!("{}", glyph);
      }
      println!();
   }
}


