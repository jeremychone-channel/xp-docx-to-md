use markdownify::docx;
use std::path::Path;

const DOC_PATH: &str = ".data/doc-simple.docx";
const MD_PATH: &str = ".data/c01-markdownify.md";

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Convert a Word document
	let path = Path::new(DOC_PATH);
	let markdown = docx::docx_convert(path)?;

	// save mardown to MD_PATH
	std::fs::write(MD_PATH, &markdown)?;

	println!("Save to {MD_PATH}");

	Ok(())
}
