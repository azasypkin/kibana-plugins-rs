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

    start() {
      initializerContext.logger.get().debug('Setting up plugin.');
      return wasmPlugin.start();
    },

    stop() {
      wasmPlugin.free();
    },
  };
}
