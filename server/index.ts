import { PluginInitializerContext } from '../../../src/core/server';
import { ExampleRsPlugin } from './plugin';

export function plugin(initializerContext: PluginInitializerContext) {
  return new ExampleRsPlugin(initializerContext);
}

export { ExampleRsPluginSetup, ExampleRsPluginStart } from './types';
