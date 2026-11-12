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
        gitignore_diff = subprocess.run(
            ["git", "diff", "--cached", "--quiet", "--", ".gitignore"],
            cwd=checkout,
        )
        if gitignore_diff.returncode != 0:
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


class ReleaseModes(unittest.TestCase):
    def setUp(self):
        self.real_cargo = shutil.which("cargo")
        self.assertIsNotNone(self.real_cargo)
        (ROOT / ".work" / "tmp").mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(dir=ROOT / ".work" / "tmp")
        self.addCleanup(self.temp.cleanup)
        self.checkout = Path(self.temp.name) / "checkout"
        subprocess.run(
            ["git", "clone", "--local", "--quiet", str(ROOT), str(self.checkout)],
            check=True,
        )
        shutil.copy(SCRIPT, self.checkout / "scripts" / "bump_minor_release.sh")
        commit_env = os.environ.copy()
        commit_env["GIT_AUTHOR_NAME"] = "release preflight"
        commit_env["GIT_AUTHOR_EMAIL"] = "release-preflight@example.com"
        commit_env["GIT_COMMITTER_NAME"] = commit_env["GIT_AUTHOR_NAME"]
        commit_env["GIT_COMMITTER_EMAIL"] = commit_env["GIT_AUTHOR_EMAIL"]
        script_diff = subprocess.run(
            ["git", "diff", "--quiet", "--", "scripts/bump_minor_release.sh"],
            cwd=self.checkout,
        )
        if script_diff.returncode != 0:
            subprocess.run(
                ["git", "add", "--", "scripts/bump_minor_release.sh"],
                cwd=self.checkout,
                check=True,
                env=commit_env,
            )
            subprocess.run(
                ["git", "commit", "-m", "fixture: release script"],
                cwd=self.checkout,
                check=True,
                env=commit_env,
                capture_output=True,
                text=True,
            )
        self.head = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=self.checkout,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        self.env = os.environ.copy()
        self.env["CARGO_REGISTRY_TOKEN"] = "test-token"
        self.env["PYTHONDONTWRITEBYTECODE"] = "1"
        self.env["GIT_CONFIG_GLOBAL"] = "/dev/null"
        self.env["GIT_CONFIG_SYSTEM"] = "/dev/null"

    def write_stub(self, name, body):
        stub_dir = Path(self.temp.name) / "bin"
        stub_dir.mkdir(exist_ok=True)
        stub = stub_dir / name
        stub.write_text(body)
        stub.chmod(stub.stat().st_mode | stat.S_IEXEC)
        self.env["PATH"] = f"{stub_dir}{os.pathsep}{self.env['PATH']}"
        return stub_dir

    def status(self):
        return subprocess.run(
            ["git", "status", "--porcelain=v1"],
            cwd=self.checkout,
            check=True,
            capture_output=True,
            text=True,
            env=self.env,
        ).stdout

    def test_dry_run_leaves_the_source_tree_unchanged(self):
        cargo_log = Path(self.temp.name) / "cargo-argv"
        self.write_stub(
            "cargo",
            f"""#!/bin/sh
printf '%s\\n' "$*" >> {cargo_log}
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
    *)
        printf 'unexpected cargo %s\\n' "$*" >&2
        exit 99
        ;;
esac
""",
        )
        self.env["PUBLISH_DRY_RUN"] = "1"
        run = subprocess.run(
            ["/bin/sh", str(self.checkout / "scripts" / "bump_minor_release.sh")],
            cwd=self.checkout,
            env=self.env,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(run.returncode, 0, run.stdout + run.stderr)
        self.assertIn("publish dry-run: runtime 9.10.0 http 9.10.0", run.stdout)
        self.assertIn("dry-run staging failed", run.stderr)
        self.assertEqual(self.status(), "")
        after = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=self.checkout,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        self.assertEqual(after, self.head)
        version = subprocess.run(
            ["python3", "tools/inventory.py", "version"],
            cwd=self.checkout,
            check=True,
            capture_output=True,
            text=True,
            env=self.env,
        )
        self.assertEqual(version.stdout.strip(), "0.3.0")
        for line in cargo_log.read_text().splitlines():
            if line.startswith("publish "):
                self.assertNotIn("--registry crates-io", line)

    def test_private_publication_covers_the_inventory_without_a_bump(self):
        cargo_log = Path(self.temp.name) / "cargo-argv"
        stuff_log = Path(self.temp.name) / "stuff-argv"
        self.write_stub(
            "cargo",
            f"""#!/bin/sh
printf '%s\\n' "$*" >> {cargo_log}
cmd=
for arg in "$@"; do
    case $arg in
        -*) ;;
        *) cmd=$arg; break ;;
    esac
done
case $cmd in
    info) exit 0 ;;
    *)
        printf 'unexpected cargo %s\\n' "$*" >&2
        exit 99
        ;;
esac
""",
        )
        self.write_stub(
            "stuff",
            f"""#!/bin/sh
printf '%s\\n' "$*" >> {stuff_log}
exit 0
""",
        )
        self.env["PUBLISH_PRIVATE_ONLY"] = "1"
        self.env["STUFF"] = "stuff"
        self.env["REGISTRY"] = "mycrates"
        run = subprocess.run(
            ["/bin/sh", str(self.checkout / "scripts" / "bump_minor_release.sh")],
            cwd=self.checkout,
            env=self.env,
            capture_output=True,
            text=True,
        )
        self.assertEqual(run.returncode, 0, run.stdout + run.stderr)
        packages = subprocess.run(
            ["python3", "tools/inventory.py", "packages"],
            cwd=self.checkout,
            check=True,
            capture_output=True,
            text=True,
            env=self.env,
        ).stdout.splitlines()
        info = [line for line in cargo_log.read_text().splitlines() if line.startswith("info ")]
        private = stuff_log.read_text().splitlines()
        self.assertEqual(len(info), len(packages))
        self.assertEqual(len(private), len(packages))
        names = [line.split()[0] for line in packages]
        for line, name in zip(info, names, strict=True):
            self.assertIn(f"info {name}@", line)
            self.assertIn("--registry crates-io", line)
        for line, name in zip(private, names, strict=True):
            self.assertIn(f"-p {name}", line)
            self.assertIn("--registry mycrates", line)
            self.assertIn("--manifest-path", line)
        self.assertFalse(any(line.startswith("publish ") for line in cargo_log.read_text().splitlines()))
        self.assertEqual(self.status(), "")
        version = subprocess.run(
            ["python3", "tools/inventory.py", "version"],
            cwd=self.checkout,
            check=True,
            capture_output=True,
            text=True,
            env=self.env,
        )
        self.assertEqual(version.stdout.strip(), "0.3.0")

    def test_private_publication_stops_when_the_exact_public_version_is_missing(self):
        cargo_log = Path(self.temp.name) / "cargo-argv"
        stuff_log = Path(self.temp.name) / "stuff-argv"
        self.write_stub(
            "cargo",
            f"""#!/bin/sh
printf '%s\\n' "$*" >> {cargo_log}
exit 1
""",
        )
        self.write_stub(
            "stuff",
            f"""#!/bin/sh
printf '%s\\n' "$*" >> {stuff_log}
exit 0
""",
        )
        self.env["PUBLISH_PRIVATE_ONLY"] = "1"
        run = subprocess.run(
            ["/bin/sh", str(self.checkout / "scripts" / "bump_minor_release.sh")],
            cwd=self.checkout,
            env=self.env,
            capture_output=True,
            text=True,
        )
        self.assertEqual(run.returncode, 1, run.stdout + run.stderr)
        self.assertIn("is not on crates.io", run.stderr)
        self.assertFalse(stuff_log.exists())
        self.assertEqual(self.status(), "")


if __name__ == "__main__":
    unittest.main()

