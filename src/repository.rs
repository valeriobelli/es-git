use anyhow::Error;
use git2::{Index, Repository};

use crate::context::GitContext;

#[napi]
pub fn get_sha(git_ref: String, context: GitContext) -> anyhow::Result<String> {
  let repo = Repository::open(&context.dir)?;
  let obj = repo.revparse_single(&git_ref)?;

  Ok(obj.id().to_string())
}

#[napi]
pub fn get_head_sha(context: GitContext) -> anyhow::Result<String> {
  let head = String::from("HEAD");

  get_sha(head, context)
}

#[napi]
pub fn get_git_root_path(context: GitContext) -> anyhow::Result<String> {
  let repo = Repository::open(&context.dir)?;
  let path = repo
    .path()
    .to_str()
    .ok_or(Error::msg("Repository path is not valid"))?;

  Ok(String::from(path))
}

#[napi]
pub fn has_merge_conflicts(
  ref1: String,
  ref2: String,
  context: GitContext,
) -> anyhow::Result<bool> {
  let tree = get_merge_tree(ref1, ref2, context)?;

  Ok(tree.has_conflicts())
}

pub fn get_merge_tree(ref1: String, ref2: String, context: GitContext) -> anyhow::Result<Index> {
  let repo = Repository::open(&context.dir)?;
  let obj1 = repo.revparse_single(&ref1)?;
  let obj2 = repo.revparse_single(&ref2)?;

  let commit1 = obj1.peel_to_commit()?;
  let commit2 = obj2.peel_to_commit()?;

  let base = repo
    .merge_base(commit1.id(), commit2.id())
    .and_then(|oid| repo.find_commit(oid))?;

  let merge_tree = repo.merge_trees(&base.tree()?, &commit1.tree()?, &commit2.tree()?, None)?;

  Ok(merge_tree)
}
