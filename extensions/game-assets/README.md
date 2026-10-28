# game-assets

Embeds the third-party packs under `assets/third-party/` through their
`pack.json` manifests, together with only the upstream files those manifests
name. Extensions look up assets by manifest id. A test fails when a manifest
names a file that is not embedded.
