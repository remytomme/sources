use aidoku::{
	ContentRating, Manga, MangaStatus, UpdateStrategy, Viewer,
	alloc::{String, Vec},
};
use serde::Deserialize;

use crate::endpoints::Url;

use super::common::{
	LibGroupAgeRestriction, LibGroupCover, LibGroupMediaType, LibGroupStatus, LibGroupTag,
};

#[derive(Default, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct LibGroupManga {
	pub rus_name: String,
	pub slug_url: String,
	pub cover: LibGroupCover,
	#[serde(rename = "ageRestriction")]
	pub age_restriction: LibGroupAgeRestriction,
	#[serde(rename = "type")]
	pub media_type: LibGroupMediaType,
	pub summary: Option<String>,
	pub tags: Option<Vec<LibGroupTag>>,
	pub authors: Option<Vec<LibGroupAuthor>>,
	pub artists: Option<Vec<LibGroupAuthor>>,
	pub status: LibGroupStatus,
}

#[derive(Default, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct LibGroupAuthor {
	pub name: String,
	pub rus_name: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct LibGroupCoverItem {
	pub cover: LibGroupCover,
	pub order: i32,
}

impl LibGroupManga {
	pub fn into_manga(self, base_url: &str, cover_quality: &str) -> Manga {
		Manga {
			key: self.slug_url.clone(),
			title: self.rus_name.clone(),
			cover: Some(self.cover.get_cover_url(cover_quality)),
			artists: self.artists.map(|artists| {
				artists
					.iter()
					.map(|author| {
						author
							.rus_name
							.clone()
							.unwrap_or_else(|| author.name.clone())
					})
					.collect()
			}),
			authors: self.authors.map(|authors| {
				authors
					.iter()
					.map(|author| {
						author
							.rus_name
							.clone()
							.unwrap_or_else(|| author.name.clone())
					})
					.collect()
			}),
			description: self.summary.clone(),
			url: Some(Url::manga_page(base_url, &self.slug_url)),
			tags: self
				.tags
				.map(|tags| tags.iter().map(|tag| tag.name.clone()).collect()),
			status: match self.status.label.as_str() {
				"Завершён" => MangaStatus::Completed,
				"Продолжается" => MangaStatus::Ongoing,
				"Заморожен" => MangaStatus::Hiatus,
				"Отменён" => MangaStatus::Cancelled,
				_ => MangaStatus::Unknown,
			},
			content_rating: match self.age_restriction.label.as_str() {
				"Нет" | "6+" | "12+" => ContentRating::Safe,
				"16+" => ContentRating::Suggestive,
				"18+" => ContentRating::NSFW,
				_ => ContentRating::Unknown,
			},
			viewer: match self.media_type.label.as_str() {
				"Манга" => Viewer::RightToLeft,
				"Манхва" => Viewer::Webtoon,
				"Маньхуа" => Viewer::Webtoon,
				"Комикс" => Viewer::LeftToRight,
				"Руманга" => Viewer::RightToLeft,
				"OEL-манга" => Viewer::RightToLeft,
				_ => Viewer::Unknown,
			},
			update_strategy: UpdateStrategy::Always,
			..Default::default()
		}
	}
}
