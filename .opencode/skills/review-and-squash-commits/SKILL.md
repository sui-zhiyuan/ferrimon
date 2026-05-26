---
name: review-and-squash-commits
description: Use when user asks to review commits since branching from master/develop, audit all changes, and squash into fewer logical commits.
---

# Review And Squash Commits

Use this skill for branch cleanup before merge: review every commit since branch-off, inspect the full diff, create a temporary backup commit, then rewrite history into fewer high-signal commits.

## Inputs To Confirm

Before rewriting history, confirm these details with the user:

1. Base branch: `master` or `develop`.
2. Target commit count (for example, 1-3 commits).
3. Grouping preference (for example, "core refactor" + "bench updates").
4. Whether force-push is allowed (required if branch is already pushed).

If a detail is missing, ask one focused question and recommend a default.

## Workflow

1. Check workspace safety.
   - Run `git status --short --branch`.
   - If there are unstaged changes, decide with the user whether to include, stash, or commit first.

2. Determine branch scope.
   - Verify base branch exists: `git branch --list master develop`.
   - Compute fork point: `git merge-base HEAD <base-branch>`.

3. Review commit history since base.
   - `git log --oneline --decorate <base-branch>..HEAD`
   - `git log --stat <base-branch>..HEAD`

4. Review full code delta.
   - `git diff --stat <base-branch>...HEAD`
   - `git diff <base-branch>...HEAD`
   - Summarize major change groups and risk areas.

5. Validate behavior before rewriting.
   - Run project checks/tests that match touched areas.
   - Report failures first; fix before history rewrite unless user says otherwise.

6. Create a backup checkpoint before rewrite.
   - Capture pre-rewrite tree hash: `OLD_TREE=$(git rev-parse HEAD^{tree})`.
   - Create backup branch tag/branch: `git branch backup/<branch>-before-rewrite-<date>`.
   - Create one backup commit that contains the current full state if needed for recovery.

7. Propose rewrite plan.
   - Provide commit groups with exact files/intent per group.
   - Allowed rewrite operations: split commits, merge commits, reorder commits, reword commits.
   - Ask for final confirmation only when history rewrite is destructive/pushed.

8. Rewrite into fewer commits.
   - Use either non-interactive soft reset flow or interactive rebase, depending on requested granularity.
   - Interactive option for full rewrite freedom: `git rebase -i <base-branch>`.
   - `BASE=$(git merge-base HEAD <base-branch>)`
   - `git reset --soft "$BASE"`
   - Stage by groups and commit each group in order.
   - Keep commit messages short, imperative, and aligned with repo style.

9. Verify rewritten result matches pre-rewrite code.
   - Compare tree equality: `git rev-parse HEAD^{tree}` should match `$OLD_TREE`.
   - If tree differs unexpectedly, stop and restore from backup branch/commit.

10. Drop backup checkpoint after successful verification.
   - Delete temporary backup branch/commit reference used for rollback.

11. Re-verify and present results.
   - Re-run checks/tests.
   - Show final history with `git log --oneline <base-branch>..HEAD`.
   - If branch was previously pushed, provide push command: `git push --force-with-lease`.
   - Ask user whether to merge branch back to upstream now.
   - If yes: update branch with rebase on upstream, then merge using no-ff.
     - `git fetch origin`
     - `git rebase origin/<base-branch>`
     - `git checkout <base-branch>`
     - `git merge --no-ff <feature-branch> -m "Merge <feature-branch>: <why + key changes>"`

## Guardrails

- Never rewrite history on shared/public branches unless user explicitly confirms.
- Prefer `--force-with-lease` instead of `--force`.
- Do not drop changes silently; account for every old commit in the new grouped commits.
- If commit splitting is needed, use staging by path/hunk to preserve clean logical boundaries.
- Keep untracked/generated artifacts out of squashed commits unless explicitly requested.
- Backup must be removable only after code/tree equivalence is confirmed.
- Preserve semantic equivalence of the branch diff against upstream unless user explicitly requests behavior change.

## Output Format

When finished, report:

- Base branch used.
- Old commit list (short hashes).
- New commit list (short hashes + messages).
- Backup checkpoint id and deletion status.
- Net diff check result (`git diff <base>...HEAD` against expected outcome).
- Tree equivalence check result (`OLD_TREE` vs rewritten `HEAD^{tree}`).
- Any follow-up action needed (for example force-push).
