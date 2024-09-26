use crate::shared::error::{Error, Result};
use futures::future::join_all;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;

///
/// テーブルテスト用の構造体
///
#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct TestStruct<'a, In, Out> {
    /// テスト名
    pub title: &'a str,
    /// テスト関数の引数
    pub input: In,
    /// テスト関数の戻り値。エラーが発生する場合は、Noneを設定する
    pub output: Option<Out>,
    /// エラーが発生するかどうか？
    pub has_error: bool,
}

///
/// テーブルテスト用のランナー
/// TestStructの配列と関数を引数に取りテストを実行する
///
#[cfg(test)]
pub fn test_runner<In, Out: Debug + PartialEq, F>(
    args: Vec<TestStruct<In, Out>>,
    func: F,
) -> Result<()>
where
    F: Fn(In) -> Result<Out>,
{
    test_logger_init();
    for t in args {
        log::debug!("{} started", t.title);
        let res = func(t.input);
        if t.has_error {
            assert!(res.is_err())
        } else {
            assert_eq!(res?, t.output.ok_or(Error::Unknown)?);
        }
        log::debug!("{} completed", t.title);
    }
    Ok(())
}

///
/// テーブルテスト用の非同期ランナー
/// TestStructの配列と非同期関数を引数に取りテストを実行する
///
#[cfg(test)]
pub async fn test_async_runner<'a, In, Out: Debug + PartialEq, F, Fut>(
    args: Vec<TestStruct<'a, In, Out>>,
    func: F,
) -> Result<()>
where
    F: Fn(In) -> Fut,
    Fut: Future<Output = Result<Out>>,
{
    test_logger_init();
    let func = Arc::new(func);
    let execute = |t: TestStruct<'a, In, Out>| {
        let func = Arc::clone(&func);
        async move {
            log::debug!("{} started", t.title);
            let res = func(t.input).await;
            if t.has_error {
                assert!(res.is_err())
            } else {
                assert_eq!(res?, t.output.ok_or(Error::Unknown)?);
            }
            log::debug!("{} completed", t.title);
            Ok::<(), Error>(())
        }
    };
    let futures = args.into_iter().map(execute).collect::<Vec<_>>();
    join_all(futures).await;
    Ok(())
}

///
/// テスト時のログの設定
///
#[cfg(test)]
pub fn test_logger_init() {
    std::env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::shared::error::Error;

    #[test]
    fn test_sync_helpers() -> Result<()> {
        let table = vec![
            TestStruct {
                title: "has_error",
                input: 0,
                output: None,
                has_error: true,
            },
            TestStruct {
                title: "do_not_have_error",
                input: 1,
                output: Some(1),
                has_error: false,
            },
        ];
        let func = |i: i32| {
            if i == 0 {
                Err(Error::Unknown)
            } else {
                Ok::<i32, Error>(i)
            }
        };
        test_runner(table, func)?;
        Ok(())
    }
    #[tokio::test]
    async fn test_async_helpers() -> Result<()> {
        test_logger_init();
        let table = vec![
            TestStruct {
                title: "async_has_error",
                input: "".to_string(),
                output: None,
                has_error: true,
            },
            TestStruct {
                title: "async_do_not_have_error",
                input: "test".to_string(),
                output: Some("test".to_string()),
                has_error: false,
            },
        ];
        let func = |i: String| async move {
            if i.is_empty() {
                Err(Error::Unknown)
            } else {
                Ok::<String, Error>(i)
            }
        };
        test_async_runner(table, func).await?;
        Ok(())
    }
}
