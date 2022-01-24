import { PluginInitializerContext } from '../../../src/core/server';
import { ExampleRsPlugin } from './plugin';

//  This exports static code and TypeScript types,
//  as well as, Kibana Platform `plugin()` initializer.

export function plugin(initializerContext: PluginInitializerContext) {
  return new ExampleRsPlugin(initializerContext);
}

export { ExampleRsPluginSetup, ExampleRsPluginStart } from './types';
