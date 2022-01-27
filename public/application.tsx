import React from 'react';
import ReactDOM from 'react-dom';
import { AppMountParameters, CoreStart } from '../../../src/core/public';
import { AppPluginStartDependencies, ExampleRsPluginStart } from './types';
import { ExampleRsApp } from './components/app';

export const renderApp = (
  { notifications, http }: CoreStart,
  { navigation }: AppPluginStartDependencies,
  ownStart: ExampleRsPluginStart,
  { appBasePath, element }: AppMountParameters
) => {
  ReactDOM.render(
    <ExampleRsApp
      ownStart={ownStart}
      basename={appBasePath}
      notifications={notifications}
      http={http}
      navigation={navigation}
    />,
    element
  );

  return () => ReactDOM.unmountComponentAtNode(element);
};
