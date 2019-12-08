type Pixel = u32;
type Layer = Vec<Pixel>;

fn parse_space_image_format(image: &[Pixel], layer_w: usize, layer_h: usize) -> Vec<Layer> {
    assert_eq!(image.len() % (layer_h * layer_w), 0);
    image
        .chunks_exact(layer_h * layer_w)
        .map(|pixels| pixels.to_vec())
        .collect::<Vec<Layer>>()
}

fn read_input() -> std::io::Result<Vec<Pixel>> {
    let input = std::fs::read_to_string("input/day8")?;
    let res = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    Ok(res)
}

fn part_one() -> std::io::Result<usize> {
    let pixels = read_input()?;
    let layers = parse_space_image_format(&pixels, 25, 6);

    let cnt_n = |layer: &Layer, n| layer.iter().filter(|&&p| p == n).count();

    let min_0_layer = {
        let mut min_0 = std::usize::MAX;
        let mut min_0_id = 0;
        for (id, layer) in layers.iter().enumerate() {
            let cnt_0 = cnt_n(layer, 0);
            if cnt_0 < min_0 {
                min_0 = cnt_0;
                min_0_id = id;
            }
        }
        min_0_id
    };

    let num_1 = cnt_n(&layers[min_0_layer], 1);
    let num_2 = cnt_n(&layers[min_0_layer], 2);

    Ok(num_1 * num_2)
}

fn part_two() -> std::io::Result<()> {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;

    let pixels = read_input()?;
    let layers = parse_space_image_format(&pixels, WIDTH, HEIGHT);

    let mut image = Vec::new();
    image.reserve(WIDTH * HEIGHT);
    for p in 0..WIDTH * HEIGHT {
        let visible_layer = layers
            .iter()
            .find(|layer| layer[p] != 2 /* transparent */)
            .unwrap();
        image.push(visible_layer[p]);
    }

    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            let p = image[h * WIDTH + w];
            if p == 1 {
                print!("\u{2588}");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Part One: result {}", part_one()?);
    println!("Part Two:");
    part_two()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let image = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let layers = parse_space_image_format(&image, 2, 1);
        assert_eq!(layers.len(), 4);
        let expectation = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(layers.cmp(&expectation), std::cmp::Ordering::Equal);

        let layers = parse_space_image_format(&image, 2, 4);
        assert_eq!(layers.len(), 1);
        let expectation = vec![vec![1, 2, 3, 4, 5, 6, 7, 8]];
        assert_eq!(layers.cmp(&expectation), std::cmp::Ordering::Equal);

        let layers = parse_space_image_format(&image, 4, 2);
        assert_eq!(layers.len(), 1);
        let expectation = vec![vec![1, 2, 3, 4, 5, 6, 7, 8]];
        assert_eq!(layers.cmp(&expectation), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_example1() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let w = 3;
        let h = 2;

        let layers = parse_space_image_format(&input, w, h);
        assert_eq!(layers.len(), 2);
        let expectation = vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]];
        assert_eq!(layers.cmp(&expectation), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_example2() {
        let input = vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];

        let layers = parse_space_image_format(&input, 2, 2);

        let mut image = Vec::new();
        for p in 0..4 {
            let color = layers
                .iter()
                .find(|layer| layer[p] != 2 /* transparent */)
                .unwrap();
            image.push(color[p]);
        }
        assert_eq!(image, vec![0, 1, 1, 0]);
    }
}
