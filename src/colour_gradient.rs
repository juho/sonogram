/*
 * Copyright (C) Simon Werner, 2019
 *
 * A Rust port of the original C++ code by Christian Briones, 2013.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, see <http://www.gnu.org/licenses/>.
 */

/// Colours required for a PNG file, includes the alpha channel.
#[derive(Clone, PartialEq, Debug)]
pub struct RGBAColour {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

impl RGBAColour {
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }

  pub fn to_vec(&self) -> Vec<u8> {
    vec![self.r, self.g, self.b, self.a]
  }
}

/// ColourGradient allows you to create custom colour gradients for each
/// PNG created.
#[derive(Clone, Debug)]
pub struct ColourGradient {
  colours: Vec<RGBAColour>,
  min: f32,
  max: f32,
}

impl ColourGradient {
  pub fn new() -> Self {
    Self {
      colours: vec![],
      min: 0.0,
      max: 1.0,
    }
  }

  pub fn get_colour(&self, value: f32) -> RGBAColour {
    assert!(self.colours.len() > 1);
    assert!(self.max >= self.min);

    if value >= self.max {
      return self.colours.last().unwrap().clone();
    }
    if value <= self.min {
      return self.colours.first().unwrap().clone();
    }

    // Get the scaled values and indexes to lookup the colour
    let range = self.max - self.min;
    let scaled_value = value / range * (self.colours.len() as f32 - 1.0);
    let idx_value = scaled_value.floor() as usize;
    let ratio = scaled_value - idx_value as f32;

    // Get the colour band
    let first = self.colours[idx_value].clone();
    let second = self.colours[idx_value + 1].clone();

    RGBAColour {
      r: self.interpolate(first.r, second.r, ratio),
      g: self.interpolate(first.g, second.g, ratio),
      b: self.interpolate(first.b, second.b, ratio),
      a: self.interpolate(first.a, second.a, ratio),
    }
  }

  pub fn add_colour(&mut self, colour: RGBAColour) {
    self.colours.push(colour);
  }

  fn interpolate(&self, start: u8, finish: u8, ratio: f32) -> u8 {
    ((f32::from(finish) - f32::from(start)) * ratio + f32::from(start)).round() as u8
  }

  pub fn set_max(&mut self, max: f32) {
    self.max = max
  }

  pub fn set_min(&mut self, min: f32) {
    self.min = min
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_colour() {
    let mut gradient = ColourGradient::new();

    gradient.add_colour(RGBAColour::new(0, 0, 0, 255));
    gradient.add_colour(RGBAColour::new(255, 255, 255, 255));
    gradient.set_min(0.0);
    gradient.set_max(1.0);

    // Test two colours
    assert_eq!(gradient.get_colour(0.0), RGBAColour::new(0, 0, 0, 255));
    assert_eq!(gradient.get_colour(1.0), RGBAColour::new(255, 255, 255, 255));
    assert_eq!(gradient.get_colour(0.5), RGBAColour::new(128, 128, 128, 255));

    // Test three colours
    gradient.add_colour(RGBAColour::new(0, 0, 0, 255));
    assert_eq!(gradient.get_colour(0.0), RGBAColour::new(0, 0, 0, 255));
    assert_eq!(gradient.get_colour(1.0), RGBAColour::new(0, 0, 0, 255));
    assert_eq!(gradient.get_colour(0.5), RGBAColour::new(255, 255, 255, 255));
    assert_eq!(gradient.get_colour(0.125), RGBAColour::new(64, 64, 64, 255));
    assert_eq!(gradient.get_colour(0.25), RGBAColour::new(128, 128, 128, 255));
    assert_eq!(gradient.get_colour(0.75), RGBAColour::new(128, 128, 128, 255));
  }

}