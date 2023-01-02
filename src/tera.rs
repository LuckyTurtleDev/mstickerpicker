use tera::{Context, Tera};

#[cfg(not(debug_assertions))]
use once_cell::sync::Lazy;

#[cfg(debug_assertions)]
macro_rules! add_template {
	($tera:expr, $path:expr, $name:expr) => {{
		$tera
			.add_template_file($path, Some($name))
			.expect(&format!("failed to load template {:?} to tera", $path));
	}};
}

#[cfg(not(debug_assertions))]
macro_rules! add_template {
	($tera:expr, $path:expr, $name:expr) => {{
		$tera
			.add_raw_template(include_str!($path), $name)
			.expect(&format!("failed to load template {:?}", $name));
	}};
}

pub fn init_tera() -> Tera {
	let mut tera = Tera::default();
	add_template!(tera, "templates/style.css.tera", "style.css");
	add_template!(tera, "templates/base.html.tera", "base.html");
	add_template!(tera, "templates/index.html.tera", "index.html");
	add_template!(tera, "templates/picker.html.tera", "picker.html");
	tera
}

#[cfg(not(debug_assertions))]
static TERA: Lazy<Tera> = Lazy::new(|| init_tera());

/// hot reload on debug, but bundle templates on release
pub fn render_template(template: &str, context: &Context) -> Result<String, tera::Error> {
	#[cfg(debug_assertions)]
	{
		let tera = init_tera();
		return tera.render(template, context);
	}
	#[cfg(not(debug_assertions))]
	TERA.render(template, context)
}
