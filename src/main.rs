use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Local};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "fledge-scratch",
    version,
    about = "Throwaway scratch notes scoped to the current repo"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Sub>,
}

#[derive(clap::Subcommand)]
enum Sub {
    /// Force-create a new scratch (default `scratch` resumes the most recent one)
    New,
    /// List scratches for this repo, newest first
    List,
    /// Open the Nth most recent scratch (1-based)
    Open { index: usize },
    /// Print the scratch directory for this repo
    Path,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        None => resume_or_create(),
        Some(Sub::New) => create(),
        Some(Sub::List) => list(),
        Some(Sub::Open { index }) => open_recent(index),
        Some(Sub::Path) => print_path(),
    }
}

fn resume_or_create() -> Result<()> {
    let dir = scratch_dir()?;
    let entries = collect_scratches(&dir)?;
    if let Some(latest) = entries.first() {
        let path = latest.path();
        println!("Resuming: {}", path.display());
        open_in_editor(&path)?;
        println!("Saved: {}", path.display());
        return Ok(());
    }
    create()
}

fn create() -> Result<()> {
    let dir = scratch_dir()?;
    fs::create_dir_all(&dir).context("creating scratch directory")?;

    let stamp = Local::now().format("%Y-%m-%d_%H%M%S");
    let path = dir.join(format!("{stamp}.md"));

    let header = format!("# scratch — {}\n\n", Local::now().format("%Y-%m-%d %H:%M"));
    fs::write(&path, header).context("writing scratch file")?;

    println!("New scratch: {}", path.display());
    open_in_editor(&path)?;
    println!("Saved: {}", path.display());
    Ok(())
}

fn list() -> Result<()> {
    let dir = scratch_dir()?;
    let entries = collect_scratches(&dir)?;
    if entries.is_empty() {
        println!("No scratches yet for this repo.");
        println!("Open one with: fledge scratch");
        return Ok(());
    }
    println!("Scratches in {}:", dir.display());
    for (i, entry) in entries.iter().enumerate() {
        let modified: DateTime<Local> = entry
            .metadata()
            .and_then(|m| m.modified())
            .map(DateTime::from)
            .unwrap_or_else(|_| Local::now());
        println!(
            "  {:>2}.  {}  {}",
            i + 1,
            modified.format("%Y-%m-%d %H:%M"),
            entry.file_name().to_string_lossy()
        );
    }
    Ok(())
}

fn open_recent(index: usize) -> Result<()> {
    if index == 0 {
        bail!("Index is 1-based — try `fledge scratch open 1` for the most recent.");
    }
    let dir = scratch_dir()?;
    let entries = collect_scratches(&dir)?;
    let entry = entries
        .get(index - 1)
        .ok_or_else(|| anyhow!("Only {} scratches exist for this repo.", entries.len()))?;
    open_in_editor(&entry.path())?;
    Ok(())
}

fn print_path() -> Result<()> {
    let dir = scratch_dir()?;
    println!("{}", dir.display());
    Ok(())
}

fn scratch_dir() -> Result<PathBuf> {
    let base = base_dir()?;
    let bucket = repo_bucket();
    Ok(base.join(bucket))
}

fn base_dir() -> Result<PathBuf> {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("$HOME is not set"))?;
    Ok(home.join(".fledge").join("scratches"))
}

fn repo_bucket() -> String {
    match find_git_root() {
        Some(root) => root
            .file_name()
            .map(|n| sanitize_name(&n.to_string_lossy()))
            .unwrap_or_else(|| "_global".to_string()),
        None => "_global".to_string(),
    }
}

fn find_git_root() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let mut current: &Path = &cwd;
    loop {
        if current.join(".git").exists() {
            return Some(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn collect_scratches(dir: &Path) -> Result<Vec<fs::DirEntry>> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut entries: Vec<fs::DirEntry> = fs::read_dir(dir)
        .context("reading scratch directory")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .collect();

    entries.sort_by_key(|e| {
        e.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    entries.reverse();
    Ok(entries)
}

fn open_in_editor(path: &Path) -> Result<()> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let parts: Vec<&str> = editor.split_whitespace().collect();
    let (program, args) = match parts.split_first() {
        Some((p, a)) => (*p, a),
        None => ("vi", &[][..]),
    };
    let status = Command::new(program)
        .args(args)
        .arg(path)
        .status()
        .with_context(|| format!("launching editor '{editor}'"))?;
    if !status.success() {
        bail!("editor exited with non-zero status");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_keeps_alphanumeric() {
        assert_eq!(sanitize_name("my-repo_v2"), "my-repo_v2");
        assert_eq!(sanitize_name("hello world"), "hello_world");
        assert_eq!(sanitize_name("path/with/slashes"), "path_with_slashes");
        assert_eq!(sanitize_name("foo.bar"), "foo.bar");
    }

    #[test]
    fn collect_returns_empty_for_missing_dir() {
        let tmp = std::env::temp_dir().join("does-not-exist-asdf-12345");
        let entries = collect_scratches(&tmp).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn collect_orders_newest_first() {
        let tmp = tempdir();
        std::fs::write(tmp.join("a.md"), "first").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(tmp.join("b.md"), "second").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(tmp.join("c.md"), "third").unwrap();

        let entries = collect_scratches(&tmp).unwrap();
        let names: Vec<String> = entries
            .iter()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, vec!["c.md", "b.md", "a.md"]);

        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn collect_skips_non_markdown() {
        let tmp = tempdir();
        std::fs::write(tmp.join("note.md"), "yes").unwrap();
        std::fs::write(tmp.join("ignore.txt"), "no").unwrap();
        std::fs::write(tmp.join("ignore"), "no ext").unwrap();

        let entries = collect_scratches(&tmp).unwrap();
        let names: Vec<String> = entries
            .iter()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, vec!["note.md"]);

        std::fs::remove_dir_all(&tmp).ok();
    }

    fn tempdir() -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        // A monotonic counter guarantees a unique path per call even when the
        // system clock resolution is too coarse to distinguish two tests that
        // run in parallel (as happens on macOS), which would otherwise collide
        // and pollute each other's directories.
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir()
            .join("fledge-scratch-tests")
            .join(format!(
                "{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                unique
            ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}
