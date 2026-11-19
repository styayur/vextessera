# SPDX-License-Identifier: Apache-2.0
import sys
import tomllib
import hashlib, json
from pathlib import Path
root = Path(__file__).resolve().parents[1]
binding = json.loads((root / 'security/glib-provenance.json').read_text())
payload = root / 'vendor/glib'
assert {p.relative_to(payload).as_posix() for p in payload.rglob('*') if p.is_file()} == set(binding['payloadSha256']), 'payload inventory'
for name, expected in binding['payloadSha256'].items():
    assert hashlib.sha256((payload / name).read_bytes()).hexdigest() == expected, name
source = (payload / 'src/variant_iter.rs').read_text()
assert 'let mut p: *mut libc::c_char' in source and '                &mut p,' in source
assert binding['version'] == '0.18.5'
manifest = tomllib.loads((root / 'src-tauri/Cargo.toml').read_text())
assert (root / 'src-tauri' / manifest['patch']['crates-io']['glib']['path']).resolve() == payload.resolve(), 'application must use this source'
lock = tomllib.loads((root / 'src-tauri/Cargo.lock').read_text())
packages = [p for p in lock['package'] if p['name'] == 'glib']
assert len(packages) == 1 and packages[0]['version'] == '0.18.5'
assert 'source' not in packages[0] and 'checksum' not in packages[0], 'local path package identity'
inventory = json.loads((root / 'licenses/rust-build-inventory.json').read_text())
notices = [p for p in inventory if p['name'] == 'glib']
assert len(notices) == 1 and notices[0]['source'] is None and notices[0]['license'] == 'MIT'
assert notices[0]['version'] == '0.18.5' and set(notices[0]['texts']) == {'COPYRIGHT', 'LICENSE'}
sys.stdout.write('Verified GTK3-compatible GLib source backport: ' + binding['upstreamFix'] + '\n')
