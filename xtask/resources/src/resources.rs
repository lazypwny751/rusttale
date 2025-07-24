#[derive(Debug)]
pub struct Resource<'a> {
	pub hash: &'a str,
	pub path: &'a str,
	pub url:  &'a str
}

pub static RESOURCES: &[Resource] = &[
	Resource {
		hash: "abc",
		path: "abc",
		url:  "abc"
	},
];
