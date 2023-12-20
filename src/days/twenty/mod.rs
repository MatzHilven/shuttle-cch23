use std::fs::File;

use actix_web::{HttpResponse, post, Responder, web};
use async_tempfile::TempFile;
use futures::StreamExt as _;
use git2::Repository;
use tar::Archive;
use tokio::io::AsyncWriteExt;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1_1);
    cfg.service(part_1_2);
    cfg.service(part_2);
}

async fn receive_tar(mut body: web::Payload) -> actix_web::Result<TempFile> {
    let mut result = TempFile::new().await.unwrap();
    while let Some(item) = body.next().await {
        result.write_all(&item?).await?;
    }
    result.sync_all().await?;
    Ok(result)
}

#[post("/20/archive_files")]
async fn part_1_1(body: web::Payload) -> impl Responder {
    let temp_file = receive_tar(body).await.unwrap();
    let file = File::open(temp_file.file_path()).unwrap();
    let mut archive = Archive::new(file);
    let result = archive.entries().unwrap().count();
    HttpResponse::Ok().body(result.to_string())
}

#[post("/20/archive_files_size")]
async fn part_1_2(body: web::Payload) -> impl Responder {
    let temp_file = receive_tar(body).await.unwrap();
    let file = File::open(temp_file.file_path()).unwrap();
    let mut archive = Archive::new(file);
    let result: u64 = archive.entries().unwrap().map(|x| x.unwrap().size()).sum();
    HttpResponse::Ok().body(result.to_string())
}


#[post("/20/cookie")]
async fn part_2(body: web::Payload) -> impl Responder {
    let temp_file = receive_tar(body).await.unwrap();
    let file = File::open(temp_file.file_path()).unwrap();
    let mut archive = Archive::new(file);
    let temp_dir = tempfile::TempDir::new().unwrap();
    archive.unpack(temp_dir.path()).unwrap();
    if let Some((author, hash)) = find_commit_author_and_hash(&temp_dir) {
        return HttpResponse::Ok().body(format!("{} {}", author, hash));
    }
    HttpResponse::Ok().body("".to_string())
}

fn find_commit_author_and_hash(temp_dir: &tempfile::TempDir) -> Option<(String, String)> {
    // Path to the extracted Git repository
    let repo_path = temp_dir.path().join(".git");

    // Open the Git repository
    if let Ok(repo) = Repository::open(repo_path) {
        // Get the branch reference by name (in this case, "christmas")
        if let Ok(branch) = repo.find_branch("christmas", git2::BranchType::Local) {
            // Get the tip commit of the branch
            if let Ok(tip) = branch.into_reference().peel_to_commit() {
                // Start from the tip commit and traverse commit history
                if let Some(commit) = find_commit_with_santa_file(&repo, tip) {
                    let author = commit.author();
                    let hash = commit.id();

                    return Some((author.name().unwrap_or("").to_string(), hash.to_string()));
                }
            }
        }
    }

    None
}

fn find_commit_with_santa_file<'a>(repo: &Repository, commit: git2::Commit<'a>) -> Option<git2::Commit<'a>> {
    // Get the tree associated with the commit
    if let Ok(tree) = commit.tree() {
        // Traverse the tree
        if let Some(path) = find_santa_file_in_tree(repo, &tree, vec![]) {
            // If the file is found in this commit's tree, return the commit
            return Some(commit);
        }
    }

    // If the file is not found in this commit, traverse its parent commits
    for parent in commit.parents() {
        if let Some(found_commit) = find_commit_with_santa_file(repo, parent) {
            return Some(found_commit);
        }
    }

    None
}


fn find_santa_file_in_tree<'a>(
    repo: &'a git2::Repository,
    tree: &'a git2::Tree<'_>,
    mut path: Vec<String>,
) -> Option<Vec<String>> {
    for entry in tree.iter() {
        let entry_name = entry.name().unwrap_or("").to_string();
        path.push(entry_name.clone()); // Record the path

        if let Ok(object) = entry.to_object(repo) {
            if let Some(subtree) = object.as_tree() {
                if let Some(found_path) = find_santa_file_in_tree(repo, subtree, path.clone()) {
                    return Some(found_path); // Forward the found path
                }
            } else if let Some(blob) = object.as_blob() {
                if entry_name == "santa.txt" {
                    if let Ok(content) = std::str::from_utf8(blob.content()) {
                        if content.contains("COOKIE") {
                            return Some(path); // Return the path when found
                        }
                    }
                }
            }
        }

        path.pop(); // Remove the entry from the path for backtracking
    }

    None
}
