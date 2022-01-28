# Rust + Kibana = 🦀️

An example Kibana plugin written in Rust and Typescript.

---

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
