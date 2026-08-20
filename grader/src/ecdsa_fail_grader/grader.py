"""ecdsa.fail point-addition grader.

Runs the *same* harness the ecdsa.fail platform runs (``./benchmark.sh`` from
the Layr-Labs/ecdsafail-challenge repo) against the agent's commit and reads
the resulting ``score.json``::

    { "score": 10704574395, "metrics": { "toffoli": 3942753, "qubits": 2715 } }

Score = average Toffoli count x peak qubit width. **Lower is better**
(``grader.direction: minimize``).

Two extra things the platform does that we mirror here:

* **Editable-surface check.** ecdsa.fail only accepts submissions that touch
  ``src/point_add/``. We diff the graded commit against ``args.base_ref``
  (the seed tip, ``main`` in the run repo) and fail the attempt if anything
  outside the editable paths changed, so agents can't "win" by editing the
  harness — such a win would be rejected upstream anyway.
* **Shared cargo target dir.** Each eval runs in a fresh detached worktree,
  which would mean a cold ``cargo build`` every time. We symlink
  ``<codebase>/target`` to a persistent dir under ``.coral/private/`` so the
  harness crates compile once and only ``point_add`` rebuilds per eval.
  (Keep ``grader.parallel.max_workers`` at 1 — cargo serializes on the
  target dir anyway.)
"""

from __future__ import annotations

import json
import os
import shutil
import subprocess
from pathlib import Path

from coral.grader import TaskGrader
from coral.types import ScoreBundle

DEFAULT_EDITABLE_PATHS = ["src/point_add"]
# Files CORAL itself may leave in a worktree; never count them as harness edits.
IGNORED_PATHS = {"CORAL.md", ".coral_dir", "results.tsv", "score.json", "ops.bin"}


class Grader(TaskGrader):
    def evaluate(self) -> ScoreBundle:
        codebase = Path(self.codebase_path)
        editable = list(self.args.get("editable_paths", DEFAULT_EDITABLE_PATHS))
        base_ref = self.args.get("base_ref", "main")
        benchmark_cmd = self.args.get("benchmark_command", "bash -lc ./benchmark.sh")

        if not (codebase / "benchmark.sh").exists():
            return self.fail(
                "benchmark.sh not found in the codebase — did you run "
                "examples/ecdsa_fail/bootstrap.sh to populate seed/?"
            )

        # --- 1. editable-surface enforcement -------------------------------
        violations, check_note = _non_editable_changes(codebase, base_ref, editable)
        if violations:
            listing = "\n".join(f"  - {p}" for p in violations[:30])
            more = f"\n  ... and {len(violations) - 30} more" if len(violations) > 30 else ""
            return self.fail(
                "Submission changed files outside the editable paths "
                f"({', '.join(editable)}). ecdsa.fail would reject this, so it "
                f"scores as a failure here too. Offending paths:\n{listing}{more}\n"
                "Revert those files (`git checkout main -- <path>`) and re-eval."
            )

        # --- 2. persistent cargo target dir ---------------------------------
        target_note = _link_shared_target_dir(codebase, Path(self.private_dir))

        # --- 3. run the real harness ----------------------------------------
        env = os.environ.copy()
        env.setdefault("CARGO_TERM_COLOR", "never")
        stdout_path = self.eval_logs_dir / "benchmark.stdout.log"
        stderr_path = self.eval_logs_dir / "benchmark.stderr.log"
        try:
            with stdout_path.open("w") as out, stderr_path.open("w") as err:
                proc = subprocess.run(
                    benchmark_cmd,
                    shell=True,
                    cwd=codebase,
                    env=env,
                    stdout=out,
                    stderr=err,
                    timeout=self.timeout,
                )
        except subprocess.TimeoutExpired:
            return self.fail(
                f"benchmark.sh timed out after {self.timeout}s. "
                f"Partial logs: {self.eval_logs_worktree_path(stderr_path)}"
            )

        tail = _tail(stderr_path) or _tail(stdout_path)
        if proc.returncode != 0:
            return self.fail(
                f"benchmark.sh exited with status {proc.returncode}. "
                f"Last output:\n{tail}\n"
                f"Full logs: {self.eval_logs_worktree_path(stderr_path)}"
            )

        # --- 4. read score.json ---------------------------------------------
        score_file = codebase / "score.json"
        if not score_file.exists():
            return self.fail(
                "benchmark.sh finished but wrote no score.json (the circuit "
                f"probably failed validation). Last output:\n{tail}"
            )
        shutil.copy2(score_file, self.eval_logs_dir / "score.json")
        results_tsv = codebase / "results.tsv"
        if results_tsv.exists():
            shutil.copy2(results_tsv, self.eval_logs_dir / "results.tsv")

        try:
            data = json.loads(score_file.read_text())
            score = float(data["score"])
        except (json.JSONDecodeError, KeyError, TypeError, ValueError) as e:
            return self.fail(f"Could not parse score.json: {e}")

        metrics = data.get("metrics", {}) or {}
        toffoli = metrics.get("toffoli")
        qubits = metrics.get("qubits")
        parts = [f"score={score:,.0f} (toffoli x qubits, lower is better)"]
        if toffoli is not None and qubits is not None:
            parts.append(f"toffoli={float(toffoli):,.0f}")
            parts.append(f"qubits={qubits}")
        best = self.args.get("reference_best")
        if best:
            ratio = score / float(best)
            parts.append(f"vs leaderboard best {float(best):,.0f}: x{ratio:.3f}")
            if ratio < 1.0:
                parts.append("BEATS THE LEADERBOARD — submit it!")
        for note in (check_note, target_note):
            if note:
                parts.append(note)
        return self.score(score, " | ".join(parts))


# ---------------------------------------------------------------------------


def _git(codebase: Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(["git", "-C", str(codebase), *args], capture_output=True, text=True)


def _non_editable_changes(
    codebase: Path, base_ref: str, editable: list[str]
) -> tuple[list[str], str]:
    """Return (paths changed outside `editable`, note). Empty list == clean.

    Compares ``merge-base(base_ref, HEAD)..HEAD`` so later upstream commits on
    the seed branch don't count against the agent. If ``base_ref`` can't be
    resolved we skip the check and say so rather than blocking evals.
    """
    mb = _git(codebase, "merge-base", base_ref, "HEAD")
    if mb.returncode != 0:
        return [], f"(editable-path check skipped: cannot resolve {base_ref!r})"
    base = mb.stdout.strip()
    excludes = [f":(exclude){p}" for p in editable]
    diff = _git(
        codebase, "diff", "--no-renames", "--name-only", "-z", base, "HEAD", "--", ".", *excludes
    )
    if diff.returncode != 0:
        return [], f"(editable-path check skipped: git diff failed: {diff.stderr.strip()})"
    changed = [p for p in diff.stdout.split("\0") if p and p not in IGNORED_PATHS]
    return changed, ""


def _link_shared_target_dir(codebase: Path, private_dir: Path) -> str:
    """Point <codebase>/target at a persistent dir so cargo builds are incremental."""
    shared = private_dir / "ecdsa_fail" / "cargo_target"
    try:
        shared.mkdir(parents=True, exist_ok=True)
        target = codebase / "target"
        if target.is_symlink() or target.is_file():
            target.unlink()
        elif target.is_dir():
            shutil.rmtree(target)
        target.symlink_to(shared, target_is_directory=True)
        return ""
    except OSError as e:
        return f"(shared cargo target dir unavailable, building cold: {e})"


def _tail(path: Path, lines: int = 25, limit: int = 3000) -> str:
    try:
        text = path.read_text(errors="replace")
    except OSError:
        return ""
    return "\n".join(text.strip().splitlines()[-lines:])[-limit:]
