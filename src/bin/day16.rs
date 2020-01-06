use std::iter::Iterator;

struct FFTPattern {
    repeat: usize,
    cnt: usize,
    coefficient_id: usize,
}

impl FFTPattern {
    const COEFFICIENTS: [i32; 4] = [0, 1, 0, -1];

    fn new(repeat: usize) -> FFTPattern {
        assert!(repeat != 0);
        FFTPattern {
            repeat,
            cnt: 0,
            coefficient_id: 0,
        }
    }
}

impl Iterator for FFTPattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let coefficient = FFTPattern::COEFFICIENTS[self.coefficient_id];

        if self.cnt == self.repeat - 1 {
            self.coefficient_id = (self.coefficient_id + 1) & 0x03;
        }
        self.cnt = (self.cnt + 1) % self.repeat;

        Some(coefficient)
    }
}

fn compute_fft_phase(input: Vec<i32>) -> Vec<i32> {
    let len = input.len();
    let mut output = Vec::with_capacity(len);

    for i in 0..len {
        let res = input
            .iter()
            .zip(FFTPattern::new(i + 1).skip(1).take(len))
            .fold(0, |res, (e, c)| res + e * c);
        output.push(res.abs() % 10);
    }

    output
}

// compute simplified FFT if pattern can be reduced to triangular
// matrix, see description in part_two()
fn compute_fft_phase_triangular(input: Vec<i32>) -> Vec<i32> {
    let len = input.len();
    let mut output = Vec::with_capacity(len);
    output.resize(len, 0);

    output[len - 1] = input[len - 1];
    for i in (0..len - 1).rev() {
        output[i] = i32::abs(input[i] + output[i + 1]) % 10;
    }

    output
}

fn read_input() -> aoc19::Result<Vec<i32>> {
    let input = std::fs::read_to_string("input/day16")?;

    let nums: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    Ok(nums)
}

fn part_one() -> aoc19::Result<String> {
    let mut input = read_input()?;

    for _ in 0..100 {
        input = compute_fft_phase(input);
    }

    Ok(input[0..8]
        .iter()
        .map(|n| n.to_string())
        .collect::<String>())
}

fn part_two() -> aoc19::Result<String> {
    let mut input = read_input()?;
    input = input.repeat(10_000);

    let offset = input[0..7]
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()?;

    assert!(offset > input.len() / 2);
    // if offset > input.len()/2 we get triangular matrix
    // IN:    A  B  C  A  B  C
    //     0  1  0 -1  0  1  0
    //     0  0  1  1  0  0 -1
    //     0  0  0  1  1  1  0
    //     0  0  0  0  1  1  1 <- starting: offset > input.len()/2
    //     0  0  0  0  0  1  1
    //     0  0  0  0  0  0  1
    //
    // FFT can be simplified to
    //   fft[len-1] = (IN[len-1]) % 10
    //   fft[len-2] = (IN[len-2] + fft[len-1]) % 10
    //   ...
    //   fft[offset] = (IN[offset] + fft[offset+1]) % 10

    input = input[offset..].to_vec();
    for _ in 0..100 {
        input = compute_fft_phase_triangular(input);
    }

    Ok(input[0..8]
        .iter()
        .map(|n| n.to_string())
        .collect::<String>())
}

fn main() -> aoc19::Result<()> {
    println!(
        "Part One: first eigth numbers after 100x FFT '{}'",
        part_one()?
    );
    println!("Part Two: '{}'", part_two()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fft_pattern() {
        assert_eq!(
            FFTPattern::new(1).take(8).collect::<Vec<_>>(),
            vec![0, 1, 0, -1, 0, 1, 0, -1]
        );
        assert_eq!(
            FFTPattern::new(2).take(8).collect::<Vec<_>>(),
            vec![0, 0, 1, 1, 0, 0, -1, -1]
        );
        assert_eq!(
            FFTPattern::new(3).take(12).collect::<Vec<_>>(),
            vec![0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]
        );
    }

    #[test]
    fn test_fft() {
        let input = vec![1, 1, 1, 1];
        assert_eq!(compute_fft_phase(input), vec![0, 2, 2, 1]);
    }

    #[test]
    fn test_example1() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let res = compute_fft_phase(input);
        assert_eq!(res, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        let res = compute_fft_phase(res);
        assert_eq!(res, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        let res = compute_fft_phase(res);
        assert_eq!(res, vec![0, 3, 4, 1, 5, 5, 1, 8]);

        let res = compute_fft_phase(res);
        assert_eq!(res, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }
}
