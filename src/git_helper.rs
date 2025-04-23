use git2::Repository;

pub fn run(repo: &str, clean_branches: bool, summary: bool) {
    match Repository::open(repo) {
        Ok(repo) => {
            if summary {
                print_summary(&repo);
            }
            if clean_branches {
                clean_merged_branches(&repo);
            }
        }
        Err(e) => eprintln!("Failed to open git repo: {}", e),
    }
}

fn print_summary(repo: &Repository) {
    if let Ok(head) = repo.head() {
        println!("Current branch: {}", head.shorthand().unwrap_or("(detached)"));
    }
    if let Ok(statuses) = repo.statuses(None) {
        println!("Changed files: {}", statuses.len());
        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                println!("- {}", path);
            }
        }
    }
}

fn clean_merged_branches(repo: &Repository) {
    let head = match repo.head() {
        Ok(h) => h.shorthand().unwrap_or("").to_string(),
        Err(_) => return,
    };
    let branches = repo.branches(Some(git2::BranchType::Local));
    if let Ok(branches) = branches {
        for branch_result in branches.flatten() {
            let (mut branch, _) = branch_result;
            // Extract branch name before any mutable borrow
            let name = match branch.name() {
                Ok(Some(name)) => name.to_string(),
                _ => continue,
            };
            if name != head {
                if let Some(true) = is_merged(repo, &name, &head) {
                    // Only mutably borrow after all immutable borrows are done
                    if let Err(e) = branch.delete() {
                        eprintln!("Failed to delete branch {}: {}", name, e);
                    } else {
                        println!("Deleted merged branch: {}", name);
                    }
                }
            }
        }
    }
}

fn is_merged(repo: &Repository, branch: &str, base: &str) -> Option<bool> {
    let branch_oid = repo.revparse_single(branch).ok()?.id();
    let base_oid = repo.revparse_single(base).ok()?.id();
    Some(repo.graph_descendant_of(base_oid, branch_oid).unwrap_or(false))
}
