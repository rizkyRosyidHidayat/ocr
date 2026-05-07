use anyhow::Context;
use ocrs::{DimOrder, ImageSource, OcrEngine, TextLine};
use rten_tensor::prelude::*;
use rten_tensor::NdTensor;

/// Convert a decoded image into an HWC tensor.
///
/// This matches the pattern used by `ocrs-cli` for feeding images into the
/// OCR engine via `ImageSource::from_tensor`.
pub fn image_to_tensor(image: image::DynamicImage) -> NdTensor<u8, 3> {
    let image = image.into_rgb8();
    let (width, height) = image.dimensions();
    NdTensor::from_data(
        [height as usize, width as usize, 3],
        image.into_vec(),
    )
}

/// Structured result from OCR processing.
pub struct OcrResult {
    /// Image dimensions as `[height, width]`.
    pub image_hw: [usize; 2],

    /// Recognized text lines. `None` entries indicate lines where recognition
    /// produced no output.
    pub line_texts: Vec<Option<TextLine>>,
}

/// Run the full OCR pipeline on raw image bytes.
///
/// Steps (following `ocrs-cli` patterns):
/// 1. Decode image bytes → `DynamicImage`
/// 2. Convert to HWC `NdTensor<u8, 3>`
/// 3. Create `ImageSource` using `from_tensor` with `DimOrder::Hwc`
/// 4. Prepare input, detect words, find text lines, recognize text
pub fn perform_ocr(engine: &OcrEngine, image_bytes: &[u8]) -> anyhow::Result<OcrResult> {
    // Decode image from raw bytes.
    let img = image::load_from_memory(image_bytes)
        .context("Failed to decode image")?;

    // Convert to HWC tensor (the correct format for ocrs).
    let color_img = image_to_tensor(img);
    let [height, width, _channels] = color_img.shape();

    // Create image source using the correct ocrs-cli pattern.
    let img_source = ImageSource::from_tensor(color_img.view(), DimOrder::Hwc)
        .context("Failed to create image source from tensor")?;

    // Prepare input for OCR engine.
    let ocr_input = engine.prepare_input(img_source)
        .context("Failed to prepare OCR input")?;

    // Detect words in the image.
    let word_rects = engine.detect_words(&ocr_input)
        .context("Failed to detect words")?;

    // Group words into text lines.
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    // Recognize text in each line.
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)
        .context("Failed to recognize text")?;

    Ok(OcrResult {
        image_hw: [height, width],
        line_texts,
    })
}
