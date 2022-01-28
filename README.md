# 🦀 Rust + ✨ Kibana = 🔥

An example Kibana plugin written in Rust and Typescript.

---

## Structure

Here is the high-level structure of the plugin:

```
/
├── common/                              (1)
├── {public,server}/                     (2)
│   └── wasm/                            (3)
├── workspace/                           (4)
│   ├── kibana_core_types/               (5)
│   │   ├── src/{public,server}/
│   │   └── Cargo.toml
│   └── kibana_plugin/                   (6)
│       ├── src/{common,public,server}/
│       └── Cargo.toml
├── Cargo.toml                           (7)
├── kibana.json                          (8)
└── package.json                         (9)

1) Common functionality written in TypeScript and TSX
2) Client and server side specific functionality written
   in TypeScript and TSX
3) Client and server side functionality compiled to WASM
   and corresponding TypeScript definitions
4) Rust workspace folder that includes all custom crates
   required by the plugin
5) Rust crate that includes Kibana Core type definitions
   and bindings (can be published in https://crates.io)
6) Rust crate that includes common, client and serve side
   specific functionality
7) Manifest file for the entire Rust workspace
8) Kibana plugin manifest file
9) Kibana plugin package file
```

## Development

See the [kibana contributing guide](https://github.com/elastic/kibana/blob/main/CONTRIBUTING.md) and [wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/) for instructions setting up your development environment.

## Scripts

<dl>
  <dt><code>yarn kbn bootstrap</code></dt>
  <dd>Execute this to install node_modules and setup the dependencies in your plugin and in Kibana</dd>

  <dt><code>yarn build-wasm</code></dt>
  <dd>Execute this to build Rust portion of the plugin (both client and server sides) to WASM modules</dd>

  <dt><code>yarn plugin-helpers build</code></dt>
  <dd>Execute this to create a distributable version of this plugin that can be installed in Kibana</dd>
</dl>
