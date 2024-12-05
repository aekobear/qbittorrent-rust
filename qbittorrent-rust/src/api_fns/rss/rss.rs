use std::{borrow::Borrow, collections::HashMap};

use proc_macros::{experimental, Builder};
use serde::Serialize;
use serde_json::Value;

use crate::{core::api::QbitApi, error_handling::error_type::ErrorType, hashmap, request_error_focus, Error};

/// ## Info
/// Describes a rss auto download rule.
#[derive(Debug, Clone, Serialize)]
pub struct RssAutoDownloadRule {
    enabled: bool,
    must_contain: String,
    must_not_contain: String,
    use_regex: bool,
    episode_filter: String,
    smart_filter: bool,
    previously_matched_episodes: Vec<String>,
    affected_feeds: Vec<String>,
    ignore_days: usize,
    last_match: String,
    add_paused: bool,
    assigned_category: String,
    save_path: String,
} impl RssAutoDownloadRule {

    /// ## Usage
    /// Returns the builder struct for [`RssAutoDownloadRule`]: [`RssAutoDownloadRuleBuilder`].
    pub fn builder() -> RssAutoDownloadRuleBuilder {
        RssAutoDownloadRuleBuilder::new()
    }
}

/// ## Info
/// Builder struct for [`RssAutoDownloadRule`].
/// 
/// ## Fields
/// | Field                     | Type     | Description                                                        |
/// |---------------------------|----------|--------------------------------------------------------------------|
/// | `enabled`                 | `Bool`     | Whether the rule is enabled                                        |
/// | `mustContain`             | `String`   | The substring that the torrent name must contain                   |
/// | `mustNotContain`          | `String`   | The substring that the torrent name must not contain               |
/// | `useRegex`                | `Bool`     | Enable regex mode in "mustContain" and "mustNotContain"             |
/// | `episodeFilter`           | `String`   | Episode filter definition                                          |
/// | `smartFilter`             | `Bool`     | Enable smart episode filter                                        |
/// | `previouslyMatchedEpisodes` | `Vec<String>`   | The list of episode IDs already matched by smart filter            |
/// | `affectedFeeds`           | `Vec<String>`     | The feed URLs the rule applied to                                   |
/// | `ignoreDays`              | `Integer`   | Ignore subsequent rule matches                                     |
/// | `lastMatch`               | `String`   | The rule last match time                                           |
/// | `addPaused`               | `Bool`     | Add matched torrent in paused mode                                 |
/// | `assignedCategory`        | `String`   | Assign category to the torrent                                     |
/// | `savePath`                | `String`   | Save torrent to the given directory
#[derive(Debug, Clone, Builder)]
pub struct RssAutoDownloadRuleBuilder{
    enabled: Option<bool>,
	must_contain: Option<String>,
	must_not_contain: Option<String>,
	use_regex: Option<bool>,
	episode_filter: Option<String>,
	smart_filter: Option<bool>,
	previously_matched_episodes: Option<Vec<String>>,
	affected_feeds: Option<Vec<String>>,
	ignore_days: Option<usize>,
	last_match: Option<String>,
	add_paused: Option<bool>,
	assigned_category: Option<String>,
	save_path: Option<String>,
} impl RssAutoDownloadRuleBuilder {
    /// ## Usage
    /// Creates a new blank instance of [`RssAutoDownloadRuleBuilder`].
    pub fn new() -> Self {
        RssAutoDownloadRuleBuilder { enabled: None, must_contain: None, must_not_contain: None, use_regex: None, episode_filter: None, smart_filter: None, previously_matched_episodes: None, affected_feeds: None, ignore_days: None, last_match: None, add_paused: None, assigned_category: None, save_path: None }
    }

    /// ## Usage
    /// Finalizes the builder and returns a [`RssAutoDownloadRule`].
    pub fn build(self) -> RssAutoDownloadRule {
        RssAutoDownloadRule { enabled: self.enabled.unwrap_or(false), must_contain: self.must_contain.unwrap_or_default(), must_not_contain: self.must_not_contain.unwrap_or_default(), use_regex: self.use_regex.unwrap_or(false), episode_filter: self.episode_filter.unwrap_or_default(), smart_filter: self.smart_filter.unwrap_or(false), previously_matched_episodes: self.previously_matched_episodes.unwrap_or(vec![]), affected_feeds: self.affected_feeds.unwrap_or(vec![]), ignore_days: self.ignore_days.unwrap_or(0), last_match: self.last_match.unwrap_or_default(), add_paused: self.add_paused.unwrap_or(false), assigned_category: self.assigned_category.unwrap_or_default(), save_path: self.save_path.unwrap_or_default() }
    }
}

impl QbitApi {
    /// ## Usage
    /// Adds a new rss folder.
    #[experimental]
    pub async fn rss_add_folder(&mut self, path: impl Into<String>) -> Result<(), Error> {
        let path: String = path.into();

        let hashmap = hashmap!(("path", path));

        request_error_focus!(self, rss_add_folder, "/rss/addFolder", hashmap, (409, ErrorType::MiscError("failure to add feed".to_string())))?;

        Ok(())
    }

    /// ## Usage
    /// Adds a new feed.
    #[experimental]
    pub async fn rss_add_feed(&mut self, url: impl Into<String>, path: Option<impl Into<String>>) -> Result<(), Error> {
        match path {
            Some(path) => {
                let x: String = path.into();
                let hashmap = hashmap!(("url", Into::<String>::into(url)), ("path", x));
                request_error_focus!(
                    self,
                    rss_add_feed,
                    "/search/addFeed",
                    hashmap,
                    (
                        409,
                        ErrorType::MiscError(
                            "Failure to add feed"
                                .to_string()
                        )
                    )
                )?;
                Ok(())
            },

            None => {
                let hashmap = hashmap!(("url", Into::<String>::into(url)));
                request_error_focus!(
                    self,
                    rss_add_feed,
                    "/search/addFeed",
                    hashmap,
                    (
                        409,
                        ErrorType::MiscError(
                            "Failure to add feed"
                                .to_string()
                        )
                    )
                )?;
                Ok(())
            }
        }
    }

    /// ## Usage
    /// Removes a feed or folder.
    #[experimental]
    pub async fn rss_remove_item(&mut self, path: impl Into<String>) -> Result<(), Error> {
        let path: String = path.into();

        let hashmap = hashmap!(("path", path));

        request_error_focus!(self, rss_add_folder, "/rss/removeItem", hashmap, (409, ErrorType::MiscError("failure to remove item".to_string())))?;

        Ok(())
    }

    /// ## Usage
    /// Moves/renames folder or feed.
    #[experimental]
    pub async fn rss_move_item(&mut self, original_path: impl Into<String>, destination_path: impl Into<String>) -> Result<(), Error> {
        let path_orig: String = original_path.into();
        let path_dest: String = destination_path.into();

        let hashmap = hashmap!(("itemPath", path_orig), ("destPath", path_dest));

        request_error_focus!(self, rss_add_folder, "/rss/moveItem", hashmap, (409, ErrorType::MiscError("failure to move item".to_string())))?;

        Ok(())
    }

    /// ## Usage
    /// Gets all items as a [`String`].
    #[experimental]
    pub async fn rss_get_all_items_raw(&mut self, with_data: Option<bool>) -> Result<String, Error> {
        if let Some(x) = with_data {
            let y = self.make_request_with_form("/rss/items", "rss_get_all_items", hashmap!(("withData", x))).await?;
            return Ok(y);
        } else {
            let y = self.make_request("/rss/items", "rss_get_all_items_raw").await?;
            Ok(y)
        }
    }

    /// ## Usage
    /// Gets all items as a json [`Value`]
    #[experimental]
    pub async fn rss_get_all_items(&mut self, with_data: Option<bool>) -> Result<Value, Error> {
        serde_json::from_str(self.rss_get_all_items_raw(with_data).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    /// ## Usage
    /// If article_id is provided only the article is marked as read otherwise the whole feed is going to be marked as read.
    #[experimental]
    pub async fn rss_mark_as_read(&mut self, item_path: impl Into<String>, article_id: Option<impl Into<String>>) -> Result<(), Error> {
        let path = item_path.into() as String;
        let mut hashmap = hashmap!(("itemPath", path));

        if let Some(x) = article_id {
            let id = x.into() as String;
            hashmap.insert("articleId", id);
            self.make_request_with_form("/rss/markAsRead", "rss_mark_as_read", hashmap).await?;
            return Ok(());
        } else {
            self.make_request_with_form("/rss/markAsRead", "rss_mark_as_read", hashmap).await?;
            return Ok(());
        }
    }

    /// ## Usage 
    /// Refreshes folder or feed.
    #[experimental]
    pub async fn rss_refresh_item(&mut self, item_path: impl Into<String>) -> Result<(), Error> {
        let path = item_path.into() as String;
        let hashmap = hashmap!(("itemPath", path));
        self.make_request_with_form("/rss/refreshItem", "rss_refresh_item", hashmap).await?;
        Ok(())
    }

    /// ## Usage
    /// Sets a new auto-downloading rule based on a [`RssAutoDownloadRule`].
    #[experimental]
    pub async fn rss_set_auto_downloading_rule(&mut self, rule_name: impl Into<String>, rule: impl Borrow<RssAutoDownloadRule>) -> Result<(), Error> {
        let name = rule_name.into() as String;
        let rule: RssAutoDownloadRule = rule.borrow().clone();
        let rule = serde_json::to_string(&rule).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))?;
        let hashmap = hashmap!(("ruleName", name), ("ruleDef", rule));
        self.make_request_with_form("/rss/setRule", "rss_set_auto_downloading_rule", hashmap).await?;
        Ok(())
    }

    /// ## Usage
    /// Renames an auto-downloading rule.
    #[experimental]
    pub async fn rss_rename_auto_downloading_rule(&mut self, original_name: impl Into<String>, new_name: impl Into<String>) -> Result<(), Error> {
        let name_orig = original_name.into() as String;
        let name_new = new_name.into() as String;
        
        let hashmap = hashmap!(("ruleName", name_orig), ("newRuleName", name_new));
        self.make_request_with_form("/rss/renameRule", "rss_rename_auto_downloading_rule", hashmap).await?;
        Ok(())
    }

    /// ## Usage
    /// Removes an auto-downloading rule.
    #[experimental]
    pub async fn rss_remove_auto_downloading_rule(&mut self, rule_name: impl Into<String>) -> Result<(), Error> {
        let name = rule_name.into() as String;
        
        let hashmap = hashmap!(("ruleName", name));
        self.make_request_with_form("/rss/removeRule", "rss_remove_auto_downloading_rule", hashmap).await?;
        Ok(())
    }

    /// ## Usage
    /// Gets all auto-downloading rules as a [`String`].
    #[experimental]
    pub async fn rss_get_all_auto_downloading_rules_raw(&mut self) -> Result<String, Error> {
        let x = self.make_request("/rss/rules", "rss_get_all_auto_downloading_rules_raw").await?;
        Ok(x)
    }

    /// ## Usage
    /// Gets all auto-downloading rules as a json [`Value`].
    #[experimental]
    pub async fn rss_get_all_auto_downloading_rules(&mut self) -> Result<Value, Error> {
        serde_json::from_str(self.rss_get_all_auto_downloading_rules_raw().await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    /// ## Usage
    /// Gets all articles matching a rule as a [`String`].
    #[experimental]
    pub async fn rss_get_all_articles_matching_a_rule_raw(&mut self, rule_name: impl Into<String>) -> Result<String, Error> {
        let name = rule_name.into() as String;
        
        let hashmap = hashmap!(("ruleName", name));
        let x = self.make_request_with_form("/rss/matchingArticles", "rss_get_all_articles_matching_a_rule_raw", hashmap).await?;
        Ok(x)
    }

    /// ## Usage
    /// Gets all articles matching a rule as a json [`Value`].
    #[experimental]
    pub async fn rss_get_all_articles_matching_a_rule(&mut self, rule_name: impl Into<String>) -> Result<Value, Error> {
        serde_json::from_str(self.rss_get_all_articles_matching_a_rule_raw(rule_name).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }
}