import { NavigationPublicPluginStart } from '../../../src/plugins/navigation/public';

export interface ExampleRsPluginSetup {
  getGreeting: () => string;
}
// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface ExampleRsPluginStart {}

export interface AppPluginStartDependencies {
  navigation: NavigationPublicPluginStart;
}
