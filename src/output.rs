use ocrs::{TextItem, TextLine};
use rten_imageproc::RotatedRect;
use serde_json::{json, Value};

/// Return the coordinates of vertices of a `RotatedRect` as an array of
/// `[x, y]` points.
///
/// This matches the format of the "vertices" data in the HierText dataset.
/// See `RotatedRect::corners` for details of the vertex order.
fn rounded_vertex_coords(rr: &RotatedRect) -> [[i32; 2]; 4] {
    rr.corners()
        .map(|point| [point.x.round() as i32, point.y.round() as i32])
}

/// Format OCR outputs as plain text.
///
/// Each recognized text line becomes one line of output. Lines where
/// recognition produced no output are skipped.
pub fn format_text_output(text_lines: &[Option<TextLine>]) -> String {
    let lines: Vec<String> = text_lines
        .iter()
        .flatten()
        .map(|line| line.to_string())
        .collect();
    lines.join("\n")
}

/// Input data for [`format_json_output`].
pub struct FormatJsonArgs<'a> {
    /// Image dimensions as `[height, width]`.
    pub image_hw: [usize; 2],

    /// Lines of text recognized by OCR engine.
    pub text_lines: &'a [Option<TextLine>],
}

/// Format extracted text and hierarchical layout information as JSON.
///
/// The JSON format roughly follows the structure of the ground truth data in
/// the [HierText](https://github.com/google-research-datasets/hiertext)
/// dataset, on which ocrs's models were trained.
pub fn format_json_output(args: FormatJsonArgs) -> Value {
    let FormatJsonArgs {
        image_hw,
        text_lines,
    } = args;

    let line_items: Vec<_> = text_lines
        .iter()
        .filter_map(|line| line.as_ref())
        .map(|line| {
            let word_items: Vec<_> = line
                .words()
                .map(|word| {
                    json!({
                        "text": word.to_string(),
                        "vertices": rounded_vertex_coords(&word.rotated_rect()),
                    })
                })
                .collect();

            json!({
                "text": line.to_string(),
                "words": word_items,
                "vertices": rounded_vertex_coords(&line.rotated_rect()),
            })
        })
        .collect();

    let [height, width] = image_hw;

    json!({
        "image_width": width,
        "image_height": height,
        "paragraphs": [{
            "lines": Value::Array(line_items),
        }]
    })
}
