// Copyright (c) 2016 William (B.J.) Snow Orvis
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


/// Generates a linear range of RGBA colors from a start color to a final color.
///
/// Eg, to create a spectrum from white to black:
///
/// ```
/// use fractal::color::color_range_linear;
///
/// let black = [0,0,0,255];
/// let white = [255,255,255,255];
///
/// let range = color_range_linear(black, white, 256);
///
/// assert_eq!(range[0], black);
/// assert_eq!(range[255], white);
/// assert_eq!(range[10], [10,10,10,255]);
/// ```
pub fn color_range_linear(first: [u8; 4], last: [u8; 4], count: usize) -> Vec<[u8; 4]> {
    if count < 2 {
        panic!("Count must be 2 or more: {}", count);
    }
    let deltas = [((last[0] as f32) - (first[0] as f32)) / ((count - 1) as f32),
                  ((last[1] as f32) - (first[1] as f32)) / ((count - 1) as f32),
                  ((last[2] as f32) - (first[2] as f32)) / ((count - 1) as f32),
                  ((last[3] as f32) - (first[3] as f32)) / ((count - 1) as f32)];

    (0..count)
        .into_iter()
        .map(|i| {
            [((first[0] as f32) + (i as f32) * deltas[0]) as u8,
             ((first[1] as f32) + (i as f32) * deltas[1]) as u8,
             ((first[2] as f32) + (i as f32) * deltas[2]) as u8,
             ((first[3] as f32) + (i as f32) * deltas[3]) as u8]
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic(expected = "Count must be 2 or more")]
    fn test_linear_zero() {
        let black = [0, 0, 0, 255];
        let white = [255, 255, 255, 255];
        let range = color_range_linear(black, white, 0);
        assert!(range.len() == 0);
    }

    #[test]
    #[should_panic(expected = "Count must be 2 or more")]
    fn test_linear_one() {
        let black = [0, 0, 0, 255];
        let white = [255, 255, 255, 255];
        let range = color_range_linear(black, white, 1);
        assert!(range.len() == 1);
    }

    #[test]
    fn test_linear_two() {
        let black = [0, 0, 0, 255];
        let white = [255, 255, 255, 255];
        let range = color_range_linear(black, white, 2);
        assert_eq!(black, range[0]);
        assert_eq!(white, range[1]);
    }
}
