use super::super::primitive::{date::Date, id::Id};
use crate::shared::error::Result;

pub struct Task {
    /// ID
    id: Id,
    /// タイトル
    title: String,
    /// 詳細
    description: String,
    /// 完了したかどうか
    is_completed: bool,
    /// 作成日
    created_at: Date,
    /// 更新日
    updated_at: Date,
}

impl Task {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        let now = Date::default();
        Self {
            id: Id::default(),
            title: title.into(),
            description: description.into(),
            is_completed: false,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn from_repository(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        is_completed: bool,
        created_at: i64,
        updated_at: i64,
    ) -> Result<Self> {
        Ok(Self {
            id: id.into().try_into()?,
            title: title.into(),
            description: description.into(),
            is_completed,
            created_at: created_at.try_into()?,
            updated_at: updated_at.try_into()?,
        })
    }

    // Setter
    /// updated_atを現在時刻で更新する
    /// 各種setterから呼び出されるプライベートメソッド
    fn change_updated_at(mut self) -> Self {
        self.updated_at = Date::default();
        self
    }

    /// titleを更新する
    pub fn change_title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self.change_updated_at()
    }

    /// descriptionを更新する
    pub fn change_description(mut self, value: impl Into<String>) -> Self {
        self.description = value.into();
        self.change_updated_at()
    }

    /// is_completedを更新する
    pub fn change_is_completed(mut self, value: bool) -> Self {
        self.is_completed = value;
        self.change_updated_at()
    }

    // Getter
    pub fn id(&self) -> Id {
        self.id.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn is_completed(&self) -> bool {
        self.is_completed
    }
    pub fn created_at(&self) -> Date {
        self.created_at.clone()
    }
    pub fn updated_at(&self) -> Date {
        self.updated_at.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::shared::error::Error;
    use crate::shared::test::*;

    #[test]
    fn change_title() -> Result<()> {
        let table = vec![TestStruct {
            title: "title更新",
            input: (
                Task::new("title", "description"),
                "updated_title".to_string(),
            ),
            output: Some((
                "updated_title".to_string(),
                "description".to_string(),
                false,
            )),
            has_error: false,
        }];
        let func = |value: (Task, String)| {
            let model = value.0.change_title(value.1);
            Ok::<(String, String, bool), Error>((
                model.title,
                model.description,
                model.is_completed,
            ))
        };
        test_runner(table, func)?;
        Ok(())
    }

    #[test]
    fn change_description() -> Result<()> {
        let table = vec![TestStruct {
            title: "description更新",
            input: (
                Task::new("title", "description"),
                "updated_description".to_string(),
            ),
            output: Some((
                "title".to_string(),
                "updated_description".to_string(),
                false,
            )),
            has_error: false,
        }];
        let func = |value: (Task, String)| {
            let model = value.0.change_description(value.1);
            Ok::<(String, String, bool), Error>((
                model.title,
                model.description,
                model.is_completed,
            ))
        };
        test_runner(table, func)?;
        Ok(())
    }

    #[test]
    fn change_is_completed() -> Result<()> {
        let table = vec![TestStruct {
            title: "is_completed更新",
            input: (Task::new("title", "description"), true),
            output: Some(("title".to_string(), "description".to_string(), true)),
            has_error: false,
        }];
        let func = |value: (Task, bool)| {
            let model = value.0.change_is_completed(value.1);
            Ok::<(String, String, bool), Error>((
                model.title,
                model.description,
                model.is_completed,
            ))
        };
        test_runner(table, func)?;
        Ok(())
    }
}
