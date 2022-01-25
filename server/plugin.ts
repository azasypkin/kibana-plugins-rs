import {
  PluginInitializerContext,
  CoreSetup,
  CoreStart,
  Plugin,
  Logger,
} from '../../../src/core/server';

import wasm from './wasm';

import { ExampleRsPluginSetup, ExampleRsPluginStart } from './types';
import { defineRoutes } from './routes';

export class ExampleRsPlugin implements Plugin<ExampleRsPluginSetup, ExampleRsPluginStart> {
  private readonly logger: Logger;
  private readonly wasmPlugin: wasm.Plugin;

  constructor(initializerContext: PluginInitializerContext) {
    this.logger = initializerContext.logger.get();
    this.wasmPlugin = new wasm.Plugin(this.logger.get('wasm'));
  }

  public setup(core: CoreSetup) {
    this.wasmPlugin.setup();

    defineRoutes(core.http.createRouter());

    return {};
  }

  public start(core: CoreStart) {
    const wasmStart = this.wasmPlugin.start();

    this.logger.info(
      `Calculated similarity of "Kibana" and "Elasticsearch" is ${
        wasmStart.findSimilarity('Kibana', 'Elasticsearch').value
      } (1 means the strings are identical, 0 - the strings are completely different)`
    );

    return {};
  }

  public stop() {
    this.wasmPlugin.free();
  }
}
