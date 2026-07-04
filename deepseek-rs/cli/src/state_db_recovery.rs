//! CLI handling for local state database startup failures.
//!
//! This keeps user-facing backup and lock-contention handling out of the main
//! CLI dispatch path while preserving the TUI startup error as the boundary type.

use deepseek_state::RuntimeDbBackup;
use deepseek_tui::LocalStateDbStartupError;
use std::io::IsTerminal;
use std::path::Path;

pub(crate) fn startup_error(err: &std::io::Error) -> Option<&LocalStateDbStartupError> {
    err.get_ref()
        .and_then(|err| err.downcast_ref::<LocalStateDbStartupError>())
}

pub(crate) fn is_locked(detail: &str) -> bool {
    deepseek_state::sqlite_error_detail_is_lock(detail)
}

pub(crate) fn is_corruption(detail: &str) -> bool {
    deepseek_state::sqlite_error_detail_is_corruption(detail)
}

pub(crate) fn is_auto_backup_recoverable(startup_error: &LocalStateDbStartupError) -> bool {
    is_corruption(startup_error.detail()) || sqlite_home_is_blocking_file(startup_error)
}

fn sqlite_home_is_blocking_file(startup_error: &LocalStateDbStartupError) -> bool {
    startup_error
        .database_path()
        .parent()
        .and_then(|path| std::fs::metadata(path).ok())
        .is_some_and(|metadata| metadata.is_file())
}

pub(crate) fn print_auto_backup_start(startup_error: &LocalStateDbStartupError) {
    eprintln!("DeepSeek CLI не может запуститься — локальная база данных повреждена.");
    eprintln!("Перемещаю повреждённую базу, чтобы создать новую из сохранённых данных.");
    print_technical_details(startup_error);
}

pub(crate) async fn backup_files_for_fresh_start(
    startup_error: &LocalStateDbStartupError,
) -> std::io::Result<Vec<RuntimeDbBackup>> {
    deepseek_state::backup_runtime_db_for_fresh_start(startup_error.database_path()).await
}

pub(crate) fn confirm_fresh_start_rebuild(
    startup_error: &LocalStateDbStartupError,
    backups: &[RuntimeDbBackup],
) -> std::io::Result<()> {
    eprintln!("DeepSeek CLI пересоздал локальную базу данных.");
    eprintln!(
        "DeepSeek CLI обнаружил повреждённую базу, переместил её в папку резервных копий и продолжит запуск с новой базой."
    );
    eprintln!("Путь к базе: {}", startup_error.database_path().display());
    if let Some(backup_folder) = backup_folder(backups) {
        eprintln!("Папка резервной копии: {}", backup_folder.display());
    } else {
        eprintln!("Папка резервной копии: недоступна");
    }

    if std::io::stdin().is_terminal() && std::io::stderr().is_terminal() {
        eprintln!("Нажмите Enter для продолжения.");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
    } else {
        eprintln!("Продолжаю запуск с новой локальной базой...");
    }
    Ok(())
}

pub(crate) fn print_diagnostic_guidance(startup_error: &LocalStateDbStartupError) {
    eprintln!("DeepSeek CLI не может запуститься — локальная база данных повреждена.");
    eprintln!("Запустите `deepseek doctor` для диагностики и получения инструкций.");
    eprintln!("Если проблема повторяется, сообщите технические детали при обращении в поддержку.");
    print_technical_details(startup_error);
}

pub(crate) fn print_locked_guidance(startup_error: &LocalStateDbStartupError) {
    eprintln!("DeepSeek CLI не может запуститься — другой процесс использует базу данных.");
    eprintln!("Закройте все другие экземпляры DeepSeek CLI и попробуйте снова.");
    print_technical_details(startup_error);
}

fn print_technical_details(startup_error: &LocalStateDbStartupError) {
    eprintln!("Технические детали:");
    eprintln!("  Расположение: {}", startup_error.database_path().display());
    eprintln!("  Причина: {}", startup_error.detail());
}

fn backup_folder(backups: &[RuntimeDbBackup]) -> Option<&Path> {
    backups.first()?.backup_path.parent()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[tokio::test]
    async fn backup_backs_up_only_failed_database_file() -> std::io::Result<()> {
        let temp_dir = TempDir::new()?;
        let state_path = deepseek_state::state_db_path(temp_dir.path());
        let failed_db_path = deepseek_state::logs_db_path(temp_dir.path());
        tokio::fs::write(state_path.as_path(), b"state").await?;
        tokio::fs::write(failed_db_path.as_path(), b"logs").await?;

        let startup_error =
            LocalStateDbStartupError::new(failed_db_path.clone(), "corrupt".to_string());
        let backups = backup_files_for_fresh_start(&startup_error).await?;

        assert_eq!(
            backups
                .iter()
                .map(|backup| &backup.original_path)
                .collect::<Vec<_>>(),
            vec![&failed_db_path]
        );
        assert!(!tokio::fs::try_exists(failed_db_path.as_path()).await?);
        assert!(tokio::fs::try_exists(state_path.as_path()).await?);
        assert!(tokio::fs::try_exists(backups[0].backup_path.as_path()).await?);
        Ok(())
    }

    #[tokio::test]
    async fn backup_replaces_blocking_sqlite_home_file() -> std::io::Result<()> {
        let temp_dir = TempDir::new()?;
        let sqlite_home = temp_dir.path().join("sqlite-home");
        tokio::fs::write(sqlite_home.as_path(), b"not-a-directory").await?;
        let startup_error = LocalStateDbStartupError::new(
            deepseek_state::state_db_path(sqlite_home.as_path()),
            "File exists".to_string(),
        );

        assert!(is_auto_backup_recoverable(&startup_error));
        let backups = backup_files_for_fresh_start(&startup_error).await?;

        assert_eq!(backups.len(), 1);
        assert!(tokio::fs::metadata(sqlite_home.as_path()).await?.is_dir());
        assert!(tokio::fs::try_exists(backups[0].backup_path.as_path()).await?);
        Ok(())
    }

    #[test]
    fn backup_folder_uses_parent_of_first_backup_path() {
        let backups = vec![RuntimeDbBackup {
            original_path: PathBuf::from("/tmp/state_5.sqlite"),
            backup_path: PathBuf::from("/tmp/db-backups/sqlite-1-0/state_5.sqlite"),
        }];

        assert_eq!(
            backup_folder(&backups),
            Some(Path::new("/tmp/db-backups/sqlite-1-0"))
        );
    }
}
