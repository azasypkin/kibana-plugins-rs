import {
  PluginInitializerContext,
  CoreSetup,
  CoreStart,
  Plugin,
  Logger,
} from '../../../src/core/server';

import { greet } from '../pkg';

import { ExampleRsPluginSetup, ExampleRsPluginStart } from './types';
import { defineRoutes } from './routes';

export class ExampleRsPlugin implements Plugin<ExampleRsPluginSetup, ExampleRsPluginStart> {
  private readonly logger: Logger;

  constructor(initializerContext: PluginInitializerContext) {
    this.logger = initializerContext.logger.get();
  }

  public setup(core: CoreSetup) {
    this.logger.info('exampleRs: Setup');
    const router = core.http.createRouter();

    greet('Oleg');

    // Register server side APIs
    defineRoutes(router);

    return {};
  }

  public start(core: CoreStart) {
    this.logger.info('exampleRs: Started');
    return {};
  }

  public stop() {}
}
