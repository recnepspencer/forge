use super::bitmap::{alpha_bounds, union_placed_bounds, CompositePlacement, PixelBounds};
use skrifa::bitmap::BitmapData;

pub(crate) fn transparent_and_bordered_bitmap_alpha_has_exact_support() {
    let mut alpha = [0; 16];
    alpha[5] = 1;
    alpha[6] = 255;
    alpha[9] = 127;
    alpha[10] = 1;
    let expected = PixelBounds {
        left: 1,
        top: 1,
        right: 3,
        bottom: 3,
    };
    let transparent = rgba_png(&[0; 16]);
    assert_eq!(
        alpha_bounds(&BitmapData::Png(&transparent), 4, 4),
        Some(None)
    );
    let bordered = rgba_png(&alpha);
    assert_eq!(
        alpha_bounds(&BitmapData::Png(&bordered), 4, 4),
        Some(Some(expected))
    );
    let bgra = alpha
        .iter()
        .flat_map(|alpha| [0, 0, 0, *alpha])
        .collect::<Vec<_>>();
    assert_eq!(
        alpha_bounds(&BitmapData::Bgra(&bgra), 4, 4),
        Some(Some(expected))
    );
    assert_eq!(
        union_placed_bounds(
            None,
            expected,
            CompositePlacement {
                offset_x: -1,
                offset_y: 2,
                width: 8,
                height: 8,
            },
        ),
        Some(PixelBounds {
            left: 0,
            top: 3,
            right: 2,
            bottom: 5,
        }),
        "composite ink must start at the child's first nontransparent pixel",
    );
}

fn rgba_png(alpha: &[u8; 16]) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut output, 4, 4);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let pixels = alpha
            .iter()
            .flat_map(|alpha| [255, 255, 255, *alpha])
            .collect::<Vec<_>>();
        writer.write_image_data(&pixels).unwrap();
    }
    output
}
