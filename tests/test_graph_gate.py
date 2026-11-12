import json
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
from graph_gate import check_metadata


class DependencyPolicyTests(unittest.TestCase):
    def test_rejects_tokio_even_when_only_a_dev_or_target_dependency(self):
        for name in ("tokio", "tokio-util"):
            metadata = {
                "packages": [{"id": name, "name": name}],
                "workspace_members": [],
                "resolve": {"nodes": []},
            }
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "Tokio package"):
                check_metadata(json.dumps(metadata), {}, set())

    def test_accepts_a_graph_without_tokio(self):
        metadata = {"packages": [], "workspace_members": [], "resolve": {"nodes": []}}
        check_metadata(json.dumps(metadata), {}, set())
