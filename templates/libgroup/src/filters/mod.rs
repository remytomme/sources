use aidoku::{
	FilterValue,
	alloc::{String, Vec, string::ToString},
};

pub struct FilterProcessor;

impl FilterProcessor {
	pub const fn new() -> Self {
		Self
	}

	pub fn process_filters(&self, filters: Vec<FilterValue>) -> Vec<(&'static str, String)> {
		filters
			.into_iter()
			.flat_map(|filter| self.process_single_filter(filter))
			.collect()
	}

	fn process_single_filter(&self, filter: FilterValue) -> Vec<(&'static str, String)> {
		let mut params = Vec::new();

		match filter {
			FilterValue::Text { id, value } => {
				self.process_text_filter(&mut params, &id, &value);
			}
			FilterValue::Sort {
				id,
				index,
				ascending,
			} => {
				self.process_sort_filter(&mut params, &id, index, ascending);
			}
			FilterValue::Check { .. } => {}
			FilterValue::Select { id, value } => {
				self.process_select_filter(&mut params, &id, value)
			}
			FilterValue::MultiSelect {
				id,
				included,
				excluded,
			} => {
				self.process_multiselect_filter(&mut params, &id, &included, &excluded);
			}
		}

		params
	}

	fn process_text_filter(&self, params: &mut Vec<(&'static str, String)>, id: &str, value: &str) {
		let trimmed = value.trim();
		if trimmed.is_empty() {
			return;
		}

		let parsed_value = match trimmed.parse::<i32>() {
			Ok(val) => val,
			Err(_) => return,
		};

		match id {
			"chap_count_min" if parsed_value >= 0 => {
				params.push(("chap_count_min", trimmed.to_string()));
			}
			"chap_count_max" => {
				params.push(("chap_count_max", trimmed.to_string()));
			}
			"year_min" if parsed_value >= 1930 => {
				params.push(("year_min", trimmed.to_string()));
			}
			"year_max" => {
				params.push(("year_max", trimmed.to_string()));
			}
			"rating_min" if parsed_value >= 0 => {
				params.push(("rating_min", trimmed.to_string()));
			}
			"rating_max" if parsed_value <= 10 => {
				params.push(("rating_max", trimmed.to_string()));
			}
			"rate_min" if parsed_value >= 0 => {
				params.push(("rate_min", trimmed.to_string()));
			}
			"rate_max" => {
				params.push(("rate_max", trimmed.to_string()));
			}
			_ => {}
		}
	}

	fn process_sort_filter(
		&self,
		params: &mut Vec<(&'static str, String)>,
		id: &str,
		index: i32,
		ascending: bool,
	) {
		match (id, index) {
			("sort", 0) => {}                                                // По популярности
			("sort", 1) => params.push(("sort_by", "rate_avg".to_string())), // По рейтингу
			("sort", 2) => params.push(("sort_by", "views".to_string())),    // По просмотрам
			("sort", 3) => params.push(("sort_by", "chap_count".to_string())), // Количеству глав
			("sort", 4) => params.push(("sort_by", "releaseDate".to_string())), // Дате релиза
			("sort", 5) => params.push(("sort_by", "last_chapter_at".to_string())), // Дате обновления
			("sort", 6) => params.push(("sort_by", "created_at".to_string())), // Дате добавления
			("sort", 7) => params.push(("sort_by", "name".to_string())),     // По названию (A-Z)
			("sort", 8) => params.push(("sort_by", "rus_name".to_string())), // По названию (А-Я)
			_ => {}
		}

		match id {
			"sort" if ascending && index > 0 => params.push(("sort_type", "asc".to_string())),
			_ => {}
		}
	}

	fn process_select_filter(
		&self,
		params: &mut Vec<(&'static str, String)>,
		id: &str,
		value: String,
	) {
		match (id, value.as_str()) {
			("genres_match_mode", "any") => params.push(("genres_soft_search", "1".to_string())),
			("tags_match_mode", "any") => params.push(("tags_soft_search", "1".to_string())),
			_ => {}
		}
	}

	fn process_multiselect_filter(
		&self,
		params: &mut Vec<(&'static str, String)>,
		id: &str,
		included: &[String],
		excluded: &[String],
	) {
		let mut add_values = |param: &'static str, values: &[String]| {
			params.extend(values.iter().map(|v| (param, v.clone())));
		};

		match id {
			"age_rating" => {
				add_values("caution[]", included);
			}
			"type" => {
				add_values("types[]", included);
			}
			"format" => {
				add_values("format[]", included);
				add_values("format_exclude[]", excluded);
			}
			"title_status" => {
				add_values("status[]", included);
			}
			"translation_status" => {
				add_values("scanlate_status[]", included);
			}
			"genres" => {
				add_values("genres[]", included);
				add_values("genres_exclude[]", excluded);
			}
			"tags" => {
				add_values("tags[]", included);
				add_values("tags_exclude[]", excluded);
			}
			_ => {}
		}
	}
}

impl Default for FilterProcessor {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod test;
