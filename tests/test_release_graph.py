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


if __name__ == "__main__":
    unittest.main()
