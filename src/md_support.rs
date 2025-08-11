/// creates a markdown table
/// # usage:
/// ```
/// use markdownify::sheets::to_markdown_table;
///
/// let headers = vec!["Names".to_string(), "Salary".to_string()];
/// let rows = vec![
///     vec!["Sarah".to_string(), "100".to_string()],
///     vec!["Jeff".to_string(), "200".to_string()],
/// ];
/// let md = to_markdown_table(&headers, &rows);
/// println!("{}", md);
/// ```
pub fn to_markdown_table(headers: &[String], rows: &[Vec<String>]) -> String {
	let mut output = String::new();
	output += &format!("| {} |\n", headers.join(" | "));
	output += &format!("|{}|\n", vec!["---"; headers.len()].join("|"));

	for row in rows {
		output += &format!("| {} |\n", row.join(" | "));
	}

	output
}
