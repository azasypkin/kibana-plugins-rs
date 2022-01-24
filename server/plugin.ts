import {
  PluginInitializerContext,
  CoreSetup,
  CoreStart,
  Plugin,
  Logger,
} from '../../../src/core/server';

import rust from '../pkg';

import { ExampleRsPluginSetup, ExampleRsPluginStart } from './types';
import { defineRoutes } from './routes';

export class ExampleRsPlugin implements Plugin<ExampleRsPluginSetup, ExampleRsPluginStart> {
  private readonly logger: Logger;
  private readonly plugin: rust.Plugin;

  constructor(initializerContext: PluginInitializerContext) {
    this.logger = initializerContext.logger.get();
    this.plugin = new rust.Plugin();
  }

  public setup(core: CoreSetup) {
    this.logger.debug('exampleRs: Setup');

    const pluginSetup = this.plugin.setup();
    const similarity = pluginSetup.findSimilarity('Kibana', 'Elasticsearch');
    this.logger.info(
      `Calculated similarity of "Kibana" and "Elasticsearch" is ${similarity.value} (1 means the strings are identical, 0 - the strings are completely different)`
    );

    defineRoutes(core.http.createRouter());

    return {};
  }

  public start(core: CoreStart) {
    this.logger.debug('exampleRs: Started');
    return {};
  }

  public stop() {}
}
