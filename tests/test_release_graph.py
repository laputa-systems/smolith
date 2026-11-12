import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
from release_graph import graph


class ReleaseGraph(unittest.TestCase):
    def setUp(self):
        (ROOT / ".work" / "tmp").mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(dir=ROOT / ".work" / "tmp")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.write("Cargo.toml", '''[workspace]
members = ["crates/smol", "crates/leaf"]
[workspace.package]
version = "0.2.0"
[workspace.dependencies]
leaf = { package = "laputa-leaf", version = "0.2.0", path = "crates/leaf" }
''')
        self.write("UPSTREAM.toml", '''[components.smol]
package = "laputa-smol"
version = "0.2.0"
upstream_name = "smol"
upstream_version = "1.0.0"
[components.leaf]
package = "laputa-leaf"
version = "0.2.0"
upstream_name = "leaf"
upstream_version = "1.0.0"
''')
        self.write("tests/consumer/Cargo.toml", '[dependencies]\nsmol = { package = "laputa-smol", version = "0.2.0" }\n')
        self.write("crates/smol/Cargo.toml", '''[package]
name = "laputa-smol"
[target.'cfg(unix)'.dependencies]
leaf = { workspace = true }
''')
        self.write("crates/leaf/Cargo.toml", '[package]\nname = "laputa-leaf"\n[dev-dependencies]\nsmol = "1"\n')

    def write(self, name, text):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)

    def test_target_dependencies_define_publish_order_but_dev_dependencies_do_not(self):
        version, manifests, edges = graph(self.root)
        self.assertEqual(version, "0.2.0")
        self.assertEqual(set(manifests), {"laputa-smol", "laputa-leaf"})
        self.assertEqual(edges, [("laputa-smol", "laputa-leaf")])

    def test_rejects_owned_dependencies_that_do_not_inherit_workspace(self):
        self.write("crates/smol/Cargo.toml", '[package]\nname = "laputa-smol"\n[dependencies]\nleaf = "0.2.0"\n')
        with self.assertRaisesRegex(ValueError, "does not inherit workspace"):
            graph(self.root)

    def test_rejects_component_version_drift(self):
        path = self.root / "UPSTREAM.toml"
        path.write_text(path.read_text().replace('version = "0.2.0"', 'version = "0.3.0"', 1))
        with self.assertRaisesRegex(ValueError, "workspace"):
            graph(self.root)

    def test_http_protocol_aliases_optional_targets_and_external_versions(self):
        self.write("Cargo.toml", '''[workspace]
members = ["crates/smol", "crates/leaf", "crates/h12tiny", "crates/h12tiny-core", "crates/fork", "crates/helper"]
[workspace.package]
version = "0.2.0"
[workspace.dependencies]
leaf = { package = "laputa-leaf", version = "0.2.0", path = "crates/leaf" }
smol = { package = "laputa-smol", version = "0.2.0", path = "crates/smol" }
bytes = "1.12.1"
h12tiny-core = { version = "0.9.0", path = "crates/h12tiny-core", default-features = false }
h2 = { package = "laputa-h2-futures-lite", version = "=0.4.20-beta.2", path = "crates/fork", default-features = false }
''')
        upstream = self.root / "UPSTREAM.toml"
        upstream.write_text(upstream.read_text() + '''
[components.h2]
family = "protocol"
package = "laputa-h2-futures-lite"
version = "0.4.20-beta.2"
upstream_name = "h2"
upstream_version = "0.4.20"
''')
        self.write("crates/h12tiny/Cargo.toml", '''[package]
name = "h12tiny"
version = "0.9.0"
[target.'cfg(unix)'.dependencies]
h12tiny-core = { workspace = true, optional = true }
[dependencies]
bytes = { workspace = true }
''')
        self.write("crates/h12tiny-core/Cargo.toml", '''[package]
name = "h12tiny-core"
version = "0.9.0"
[dependencies]
h2 = { workspace = true, optional = true }
[dev-dependencies]
smol = { workspace = true }
''')
        self.write("crates/fork/Cargo.toml", '''[package]
name = "laputa-h2-futures-lite"
version = "0.4.20-beta.2"
[dependencies]
leaf = { workspace = true }
[build-dependencies]
smol = { workspace = true }
''')
        self.write("crates/helper/Cargo.toml", '''[package]
name = "helper"
version = "0.1.0"
publish = false
[dependencies]
leaf = { workspace = true }
''')
        version, manifests, edges = graph(self.root)
        self.assertEqual(version, "0.2.0")
        self.assertEqual(
            set(manifests),
            {"laputa-smol", "laputa-leaf", "h12tiny", "h12tiny-core", "laputa-h2-futures-lite"},
        )
        self.assertNotIn("helper", manifests)
        self.assertIn(("h12tiny", "h12tiny-core"), edges)
        self.assertIn(("h12tiny-core", "laputa-h2-futures-lite"), edges)
        self.assertIn(("laputa-h2-futures-lite", "laputa-leaf"), edges)
        self.assertIn(("laputa-h2-futures-lite", "laputa-smol"), edges)
        self.assertIn(("h12tiny-core", "laputa-smol"), edges)
        self.assertNotIn(("h12tiny", "bytes"), edges)
        from inventory import load

        order = load(self.root).order
        self.assertLess(order.index("laputa-smol"), order.index("h12tiny-core"))

    def test_http_versions_must_agree(self):
        self.write("Cargo.toml", '''[workspace]
members = ["crates/smol", "crates/leaf", "crates/h12tiny", "crates/other"]
[workspace.package]
version = "0.2.0"
[workspace.dependencies]
leaf = { package = "laputa-leaf", version = "0.2.0", path = "crates/leaf" }
h12tiny = { version = "0.9.0", path = "crates/h12tiny" }
other = { version = "0.8.0", path = "crates/other" }
''')
        self.write("crates/h12tiny/Cargo.toml", '[package]\nname = "h12tiny"\nversion = "0.9.0"\n')
        self.write("crates/other/Cargo.toml", '[package]\nname = "other"\nversion = "0.8.0"\n')
        with self.assertRaisesRegex(ValueError, "HTTP packages"):
            graph(self.root)

    def test_dependency_cycle_is_reported(self):
        self.write("crates/smol/Cargo.toml", '''[package]
name = "laputa-smol"
[dependencies]
leaf = { workspace = true }
''')
        self.write("crates/leaf/Cargo.toml", '''[package]
name = "laputa-leaf"
[dependencies]
smol = { package = "laputa-smol", version = "0.2.0", path = "../smol" }
''')
        with self.assertRaisesRegex(ValueError, "direct owned dependency"):
            graph(self.root)
        self.write("Cargo.toml", '''[workspace]
members = ["crates/smol", "crates/leaf"]
[workspace.package]
version = "0.2.0"
[workspace.dependencies]
leaf = { package = "laputa-leaf", version = "0.2.0", path = "crates/leaf" }
smol = { package = "laputa-smol", version = "0.2.0", path = "crates/smol" }
''')
        self.write("crates/leaf/Cargo.toml", '''[package]
name = "laputa-leaf"
[dependencies]
smol = { workspace = true }
''')
        from inventory import load
        with self.assertRaisesRegex(ValueError, "dependency cycle"):
            load(self.root)

    def test_apply_keeps_external_and_protocol_versions_and_the_source_tree(self):
        self.write("Cargo.toml", '''[workspace]
members = ["crates/smol", "crates/leaf", "crates/h12tiny", "crates/fork"]
[workspace.package]
version = "0.2.0"
[workspace.dependencies]
leaf = { package = "laputa-leaf", version = "0.2.0", path = "crates/leaf" }
smol = { package = "laputa-smol", version = "0.2.0", path = "crates/smol" }
bytes = "1.12.1"
h12tiny = { version = "0.9.0", path = "crates/h12tiny", default-features = false }
h2 = { package = "laputa-h2-futures-lite", version = "=0.4.20-beta.2", path = "crates/fork", default-features = false }
''')
        upstream = self.root / "UPSTREAM.toml"
        upstream.write_text(upstream.read_text() + '''
[components.h2]
family = "protocol"
package = "laputa-h2-futures-lite"
version = "0.4.20-beta.2"
upstream_name = "h2"
upstream_version = "0.4.20"
''')
        self.write("crates/h12tiny/Cargo.toml", '''[package]
name = "h12tiny"
version = "0.9.0"
[dependencies]
bytes = { workspace = true }
h2 = { workspace = true }
''')
        self.write("crates/fork/Cargo.toml", '''[package]
name = "laputa-h2-futures-lite"
version = "0.4.20-beta.2"
''')
        self.write("examples/app/Cargo.toml", '''[package]
name = "app"
version = "0.1.0"
[dependencies]
h12tiny = { version = "0.9.0", path = "../../crates/h12tiny" }
smol = { package = "laputa-smol", version = "0.2.0", path = "../../crates/smol" }
bytes = "1.12.1"
''')
        before = (self.root / "Cargo.toml").read_text()
        from release_apply import apply
        import shutil
        copy = self.root.parent / f"{self.root.name}-copy"
        self.addCleanup(shutil.rmtree, copy, True)
        copy.mkdir()
        for path in self.root.rglob("*"):
            if path.is_file():
                target = copy / path.relative_to(self.root)
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(path.read_text())
        changed = apply(copy, "0.3.0", "0.10.0")
        self.assertEqual((self.root / "Cargo.toml").read_text(), before)
        root_text = (copy / "Cargo.toml").read_text()
        self.assertIn('version = "0.3.0"', root_text)
        self.assertIn('bytes = "1.12.1"', root_text)
        self.assertIn('version = "=0.4.20-beta.2"', root_text)
        self.assertIn('version = "0.10.0"', root_text)
        self.assertNotIn('version = "0.9.0"', root_text)
        self.assertIn('version = "0.10.0"', (copy / "crates/h12tiny/Cargo.toml").read_text())
        self.assertIn('version = "0.4.20-beta.2"', (copy / "crates/fork/Cargo.toml").read_text())
        self.assertIn('version = "0.3.0"', (copy / "UPSTREAM.toml").read_text())
        self.assertIn('version = "0.4.20-beta.2"', (copy / "UPSTREAM.toml").read_text())
        example = (copy / "examples/app/Cargo.toml").read_text()
        self.assertIn('version = "0.10.0"', example)
        self.assertIn('version = "0.3.0"', example)
        self.assertIn('bytes = "1.12.1"', example)
        self.assertTrue(changed)

    def test_a_newer_registry_version_does_not_satisfy_the_intended_version(self):
        from inventory import exact_version_present
        self.assertFalse(exact_version_present("0.4.20-beta.2", ["9.9.9", "0.4.21"]))
        self.assertTrue(exact_version_present("0.4.20-beta.2", ["0.4.20-beta.2"]))


if __name__ == "__main__":
    unittest.main()
