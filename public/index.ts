import './index.scss';

import { ExampleRsPlugin } from './plugin';

// This exports static code and TypeScript types,
// as well as, Kibana Platform `plugin()` initializer.
export function plugin() {
  return new ExampleRsPlugin();
}
export { ExampleRsPluginSetup, ExampleRsPluginStart } from './types';
