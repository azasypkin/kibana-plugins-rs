import { NavigationPublicPluginStart } from '../../../src/plugins/navigation/public';
import type * as wasm from './wasm';

export type ExampleRsPluginSetup = Omit<wasm.PluginSetup, 'free'>;
export type ExampleRsPluginStart = Omit<wasm.PluginStart, 'free' | 'findSimilarity'> & {
  findSimilarity(string_a: string, string_b: string): Promise<wasm.Similarity>;
};

export interface AppPluginStartDependencies {
  navigation: NavigationPublicPluginStart;
}
