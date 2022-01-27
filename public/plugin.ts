import { AppMountParameters, CoreSetup, CoreStart, Plugin } from '../../../src/core/public';
import { ExampleRsPluginSetup, ExampleRsPluginStart, AppPluginStartDependencies } from './types';
import { PLUGIN_NAME } from '../common';

import type * as wasm from './wasm';

export class ExampleRsPlugin implements Plugin<ExampleRsPluginSetup, ExampleRsPluginStart> {
  private wasmPlugin: Promise<wasm.Plugin>;

  constructor() {
    this.wasmPlugin = import('./wasm/kibana_plugin').then((wasmModule) => new wasmModule.Plugin());
  }

  public setup(
    core: CoreSetup<AppPluginStartDependencies, ExampleRsPluginStart>
  ): ExampleRsPluginSetup {
    // Register an application into the side navigation menu
    core.application.register({
      id: 'exampleRs',
      title: PLUGIN_NAME,
      async mount(params: AppMountParameters) {
        const { renderApp } = await import('./application');
        const [coreStart, depsStart, ownStart] = await core.getStartServices();
        return renderApp(coreStart, depsStart, ownStart, params);
      },
    });

    // Initialize WASM plugin part and rewrite `Promise` to make sure `start` is only invoked after
    // `setup` is complete.
    this.wasmPlugin = this.wasmPlugin.then((wasmPlugin) => {
      wasmPlugin.setup(core);
      return wasmPlugin;
    });
    return {};
  }

  public start(core: CoreStart): ExampleRsPluginStart {
    const wasmStart = this.wasmPlugin.then((wasmPlugin) => wasmPlugin.start());
    return {
      async findSimilarity(stringA, stringB) {
        return (await wasmStart).findSimilarity(stringA, stringB);
      },
    };
  }

  public stop() {
    this.wasmPlugin.then((wasmPlugin) => wasmPlugin.free());
  }
}
