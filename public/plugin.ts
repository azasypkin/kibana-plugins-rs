import { i18n } from '@kbn/i18n';
import { AppMountParameters, CoreSetup, CoreStart, Plugin } from '../../../src/core/public';
import { ExampleRsPluginSetup, ExampleRsPluginStart, AppPluginStartDependencies } from './types';
import { PLUGIN_NAME } from '../common';

import type * as wasm from './wasm';

export class ExampleRsPlugin implements Plugin<ExampleRsPluginSetup, ExampleRsPluginStart> {
  private wasmPlugin: Promise<wasm.Plugin>;

  constructor() {
    this.wasmPlugin = import('./wasm/kibana_plugin_public').then(
      (wasmModule) => new wasmModule.Plugin()
    );
  }

  public setup(core: CoreSetup): ExampleRsPluginSetup {
    this.wasmPlugin.then((wasmPlugin) => wasmPlugin.setup(core));

    // Register an application into the side navigation menu
    core.application.register({
      id: 'exampleRs',
      title: PLUGIN_NAME,
      async mount(params: AppMountParameters) {
        // Load application bundle
        const { renderApp } = await import('./application');
        // Get start services as specified in kibana.json
        const [coreStart, depsStart] = await core.getStartServices();
        // Render the application
        return renderApp(coreStart, depsStart as AppPluginStartDependencies, params);
      },
    });

    // Return methods that should be available to other plugins
    return {
      getGreeting() {
        return i18n.translate('exampleRs.greetingText', {
          defaultMessage: 'Hello from {name}!',
          values: {
            name: PLUGIN_NAME,
          },
        });
      },
    };
  }

  public start(core: CoreStart): ExampleRsPluginStart {
    this.wasmPlugin.then((wasmPlugin) => {
      const wasmStart = wasmPlugin.start();
      console.log(
        `[FROM JS] Calculated similarity of "Kibana" and "Elasticsearch" is ${
          wasmStart.findSimilarity('Kibana', 'Elasticsearch').value
        } (1 means the strings are identical, 0 - the strings are completely different)`
      );
    });
    return {};
  }

  public stop() {
    this.wasmPlugin.then((wasmPlugin) => {
      wasmPlugin.free();
    });
  }
}
