import sys
import threading
import unittest
import urllib.error
import urllib.request
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
import stage


class RegistryIsolation(unittest.TestCase):
    def test_owned_packages_never_fall_back_to_public_registry(self):
        state = stage.RegistryState()
        server = stage.ThreadingHTTPServer(("127.0.0.1", 0), stage.make_handler(state, ""))
        thread = threading.Thread(target=server.serve_forever)
        thread.start()
        origin = f"http://127.0.0.1:{server.server_port}"
        name = stage.COMPONENTS["async-task"]["package"]
        try:
            with patch.object(stage, "proxy_bytes", return_value=b"public") as proxy:
                for path in (f"/index/{stage.index_prefix(name)}/{name}", f"/crates/{name}/0.2.0/download"):
                    with self.assertRaises(urllib.error.HTTPError) as error:
                        urllib.request.urlopen(origin + path)
                    self.assertEqual(error.exception.code, 404)
                    error.exception.close()
                proxy.assert_not_called()
                with urllib.request.urlopen(origin + "/index/3/f/foo") as response:
                    self.assertEqual(response.read(), b"public")
                state.put(name, "0.2.0", b"local", '{"vers":"0.2.0"}')
                with urllib.request.urlopen(origin + f"/crates/{name}/0.2.0/download") as response:
                    self.assertEqual(response.read(), b"local")
                with urllib.request.urlopen(origin + f"/index/{stage.index_prefix(name)}/{name}") as response:
                    self.assertIn(b'"vers":"0.2.0"', response.read())
        finally:
            server.shutdown()
            thread.join()
            server.server_close()


if __name__ == "__main__":
    unittest.main()
