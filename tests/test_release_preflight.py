import os
import shutil
import stat
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "bump_minor_release.sh"


class PublicRegistryPreflight(unittest.TestCase):
    def test_public_cargo_names_crates_io_once(self):
        text = SCRIPT.read_text()
        self.assertEqual(text.count("PUBLIC_REGISTRY=crates-io"), 1)
        self.assertIn('cargo "$@" --registry "$PUBLIC_REGISTRY"', text)
        for line in text.splitlines():
            stripped = line.strip()
            if stripped.startswith("#"):
                continue
            self.assertFalse(
                stripped.startswith("cargo search")
                or stripped.startswith("cargo publish")
                or stripped.startswith("cargo owner"),
                stripped,
            )
        for operation in ("search ", "publish ", "owner "):
            self.assertIn(f"cargo_public {operation}", text)

    def test_preflight_queries_name_crates_io_without_publishing(self):
        real_cargo = shutil.which("cargo")
        self.assertIsNotNone(real_cargo)
        (ROOT / ".work" / "tmp").mkdir(parents=True, exist_ok=True)
        temp = tempfile.TemporaryDirectory(dir=ROOT / ".work" / "tmp")
        self.addCleanup(temp.cleanup)
        checkout = Path(temp.name) / "checkout"
        subprocess.run(
            ["git", "clone", "--local", "--quiet", str(ROOT), str(checkout)],
            check=True,
        )
        shutil.copy(SCRIPT, checkout / "scripts" / "bump_minor_release.sh")
        script_diff = subprocess.run(
            ["git", "diff", "--quiet", "--", "scripts/bump_minor_release.sh"],
            cwd=checkout,
        )
        if script_diff.returncode != 0:
            commit_env = os.environ.copy()
            commit_env["GIT_AUTHOR_NAME"] = "release preflight"
            commit_env["GIT_AUTHOR_EMAIL"] = "release-preflight@example.com"
            commit_env["GIT_COMMITTER_NAME"] = commit_env["GIT_AUTHOR_NAME"]
            commit_env["GIT_COMMITTER_EMAIL"] = commit_env["GIT_AUTHOR_EMAIL"]
            subprocess.run(
                ["git", "add", "--", "scripts/bump_minor_release.sh"],
                cwd=checkout,
                check=True,
                env=commit_env,
                capture_output=True,
                text=True,
            )
            subprocess.run(
                ["git", "commit", "-m", "fixture: public registry selection"],
                cwd=checkout,
                check=True,
                env=commit_env,
                capture_output=True,
                text=True,
            )

        offline = os.environ.copy()
        offline["CARGO_NET_OFFLINE"] = "true"
        replaced = subprocess.run(
            [real_cargo, "search", "laputa-smol", "--limit", "1"],
            cwd=checkout,
            env=offline,
            capture_output=True,
            text=True,
        )
        self.assertEqual(replaced.returncode, 101)
        self.assertIn("include `--registry crates-io`", replaced.stderr)

        log = Path(temp.name) / "cargo-argv"
        stub_dir = Path(temp.name) / "bin"
        stub_dir.mkdir()
        stub = stub_dir / "cargo"
        stub.write_text(
            f"""#!/bin/sh
printf '%s\\n' "$*" >> {log}
cmd=
for arg in "$@"; do
    case $arg in
        -*) ;;
        *) cmd=$arg; break ;;
    esac
done
case $cmd in
    search)
        printf '%s = "9.9.9"\\n' "$2"
        ;;
    owner)
        printf 'test-owner\\n'
        ;;
    check)
        exit 86
        ;;
    *)
        printf 'unexpected cargo %s\\n' "$*" >&2
        exit 99
        ;;
esac
"""
        )
        stub.chmod(stub.stat().st_mode | stat.S_IEXEC)
        head = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=checkout,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        env = os.environ.copy()
        env["PATH"] = f"{stub_dir}{os.pathsep}{env['PATH']}"
        env["CARGO_REGISTRY_TOKEN"] = "test-token"
        env["PYTHONDONTWRITEBYTECODE"] = "1"
        env["GIT_CONFIG_GLOBAL"] = "/dev/null"
        env["GIT_CONFIG_SYSTEM"] = "/dev/null"
        run = subprocess.run(
            ["/bin/sh", str(checkout / "scripts" / "bump_minor_release.sh")],
            cwd=checkout,
            env=env,
            capture_output=True,
            text=True,
        )
        self.assertEqual(run.returncode, 86, run.stdout + run.stderr)
        argv = log.read_text().splitlines()
        searches = [line for line in argv if line.startswith("search ")]
        owners = [line for line in argv if line.startswith("owner ")]
        self.assertGreater(len(searches), 0)
        self.assertEqual(len(owners), len(searches))
        for line in searches + owners:
            self.assertIn("--registry crates-io", line)
        self.assertTrue(any(line.startswith("check ") for line in argv))
        self.assertFalse(any(line.startswith("publish ") for line in argv))
        status = subprocess.run(
            ["git", "status", "--porcelain=v1"],
            cwd=checkout,
            check=True,
            capture_output=True,
            text=True,
            env=env,
        )
        self.assertEqual(status.stdout, "")
        after = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=checkout,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        self.assertEqual(after, head)

    def test_graph_bytecode_stays_ignored_and_other_files_do_not(self):
        (ROOT / ".work" / "tmp").mkdir(parents=True, exist_ok=True)
        temp = tempfile.TemporaryDirectory(dir=ROOT / ".work" / "tmp")
        self.addCleanup(temp.cleanup)
        checkout = Path(temp.name) / "checkout"
        subprocess.run(
            ["git", "clone", "--local", "--quiet", str(ROOT), str(checkout)],
            check=True,
        )
        shutil.copy(ROOT / ".gitignore", checkout / ".gitignore")
        commit_env = os.environ.copy()
        commit_env["GIT_AUTHOR_NAME"] = "release preflight"
        commit_env["GIT_AUTHOR_EMAIL"] = "release-preflight@example.com"
        commit_env["GIT_COMMITTER_NAME"] = commit_env["GIT_AUTHOR_NAME"]
        commit_env["GIT_COMMITTER_EMAIL"] = commit_env["GIT_AUTHOR_EMAIL"]
        subprocess.run(
            ["git", "add", "--", ".gitignore"],
            cwd=checkout,
            check=True,
            env=commit_env,
            capture_output=True,
            text=True,
        )
        subprocess.run(
            ["git", "commit", "-m", "fixture: ignore bytecode"],
            cwd=checkout,
            check=True,
            env=commit_env,
            capture_output=True,
            text=True,
        )
        env = os.environ.copy()
        env.pop("PYTHONDONTWRITEBYTECODE", None)
        env.pop("PYTHONPYCACHEPREFIX", None)
        env["GIT_CONFIG_GLOBAL"] = "/dev/null"
        env["GIT_CONFIG_SYSTEM"] = "/dev/null"
        version = subprocess.run(
            ["python3", "tools/release_graph.py", "version"],
            cwd=checkout,
            env=env,
            check=True,
            capture_output=True,
            text=True,
        )
        self.assertEqual(version.stdout.strip(), "0.3.0")
        self.assertTrue((checkout / "tools" / "__pycache__").is_dir())
        status = subprocess.run(
            ["git", "status", "--porcelain=v1"],
            cwd=checkout,
            check=True,
            capture_output=True,
            text=True,
            env=env,
        )
        self.assertEqual(status.stdout, "")

        (checkout / "stray.txt").write_text("not a release artifact\n")
        log = Path(temp.name) / "cargo-argv"
        stub_dir = Path(temp.name) / "bin"
        stub_dir.mkdir()
        stub = stub_dir / "cargo"
        stub.write_text(
            f"""#!/bin/sh
printf '%s\\n' "$*" >> {log}
cmd=
for arg in "$@"; do
    case $arg in
        -*) ;;
        *) cmd=$arg; break ;;
    esac
done
case $cmd in
    search)
        printf '%s = "9.9.9"\\n' "$2"
        ;;
    owner)
        printf 'test-owner\\n'
        ;;
    check)
        exit 86
        ;;
    *)
        printf 'unexpected cargo %s\\n' "$*" >&2
        exit 99
        ;;
esac
"""
        )
        stub.chmod(stub.stat().st_mode | stat.S_IEXEC)
        env["PATH"] = f"{stub_dir}{os.pathsep}{env['PATH']}"
        env["CARGO_REGISTRY_TOKEN"] = "test-token"
        rejected = subprocess.run(
            ["/bin/sh", str(checkout / "scripts" / "bump_minor_release.sh")],
            cwd=checkout,
            env=env,
            capture_output=True,
            text=True,
        )
        self.assertEqual(rejected.returncode, 1, rejected.stderr)
        self.assertIn("release requires a clean working tree", rejected.stderr)
        argv = log.read_text().splitlines()
        self.assertFalse(any(line.startswith("check ") for line in argv))
        self.assertFalse(any(line.startswith("publish ") for line in argv))
        (checkout / "stray.txt").unlink()
        log.unlink()
        accepted = subprocess.run(
            ["/bin/sh", str(checkout / "scripts" / "bump_minor_release.sh")],
            cwd=checkout,
            env=env,
            capture_output=True,
            text=True,
        )
        self.assertEqual(accepted.returncode, 86, accepted.stderr)
        status = subprocess.run(
            ["git", "status", "--porcelain=v1"],
            cwd=checkout,
            check=True,
            capture_output=True,
            text=True,
            env=env,
        )
        self.assertEqual(status.stdout, "")


if __name__ == "__main__":
    unittest.main()
