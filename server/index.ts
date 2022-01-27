import { CoreSetup, CoreStart, PluginInitializerContext } from '../../../src/core/server';

import wasm from './wasm';
export type ExampleRsPluginSetup = Omit<wasm.PluginSetup, 'free'>;
export type ExampleRsPluginStart = Omit<wasm.PluginStart, 'free'>;

export function plugin(initializerContext: PluginInitializerContext) {
  const wasmPlugin = new wasm.Plugin(initializerContext);
  return {
    setup(core: CoreSetup) {
      return wasmPlugin.setup(core);
    },

    start(core: CoreStart) {
      const wasmStart = wasmPlugin.start();

      initializerContext.logger
        .get()
        .info(
          `Calculated similarity of "Kibana" and "Elasticsearch" is ${
            wasmStart.findSimilarity('Kibana', 'Elasticsearch').value
          } (1 means the strings are identical, 0 - the strings are completely different)`
        );

      return wasmStart;
    },

    stop() {
      wasmPlugin.free();
    },
  };
}
