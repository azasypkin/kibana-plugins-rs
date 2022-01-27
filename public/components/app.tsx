import React, { useState } from 'react';
import { FormattedMessage, I18nProvider } from '@kbn/i18n-react';
import { BrowserRouter as Router } from 'react-router-dom';
import {
  EuiButton,
  EuiHorizontalRule,
  EuiPage,
  EuiPageBody,
  EuiPageContent,
  EuiPageContentBody,
  EuiPageContentHeader,
  EuiPageHeader,
  EuiTitle,
  EuiText,
  EuiFieldText,
  EuiSpacer,
} from '@elastic/eui';

import { CoreStart } from '../../../../src/core/public';
import { NavigationPublicPluginStart } from '../../../../src/plugins/navigation/public';

import { PLUGIN_ID, PLUGIN_NAME } from '../../common';
import { ExampleRsPluginStart } from '../types';

interface ExampleRsAppDeps {
  basename: string;
  notifications: CoreStart['notifications'];
  http: CoreStart['http'];
  ownStart: ExampleRsPluginStart;
  navigation: NavigationPublicPluginStart;
}

export const ExampleRsApp = ({ basename, http, navigation, ownStart }: ExampleRsAppDeps) => {
  const [words, setWords] = useState<{ wordA?: string; wordB?: string }>({});
  const [similarities, setSimilarities] = useState<{ client?: number; server?: number }>({});

  const onWordAChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setWords({ ...words, wordA: e.target.value });
  };

  const onWordBChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setWords({ ...words, wordB: e.target.value });
  };

  const onClickHandler = () => {
    Promise.all([
      ownStart.findSimilarity(words.wordA!, words.wordB!).then((similarity) => similarity.value),
      http
        .post<{ comment: string; similarity: number }>('/api/wasm', { body: JSON.stringify(words) })
        .then((res) => res.similarity),
    ]).then(([client, server]) => {
      setSimilarities({ client, server });
    });
  };

  // Render the application DOM.
  // Note that `navigation.ui.TopNavMenu` is a stateful component exported on the `navigation` plugin's start contract.
  return (
    <Router basename={basename}>
      <I18nProvider>
        <>
          <navigation.ui.TopNavMenu
            appName={PLUGIN_ID}
            showSearchBar={true}
            useDefaultBehaviors={true}
          />
          <EuiPage restrictWidth="1000px">
            <EuiPageBody>
              <EuiPageHeader>
                <EuiTitle size="l">
                  <h1>
                    <FormattedMessage
                      id="exampleRs.helloWorldText"
                      defaultMessage="{name}"
                      values={{ name: PLUGIN_NAME }}
                    />
                  </h1>
                </EuiTitle>
              </EuiPageHeader>
              <EuiPageContent>
                <EuiPageContentHeader>
                  <EuiTitle>
                    <h2>
                      <FormattedMessage
                        id="exampleRs.congratulationsTitle"
                        defaultMessage="Rust + Typescript + Kibana = ❤️"
                      />
                    </h2>
                  </EuiTitle>
                </EuiPageContentHeader>
                <EuiPageContentBody>
                  <EuiText>
                    <p>
                      <FormattedMessage
                        id="exampleRs.content"
                        defaultMessage="Check similarity of two words using Levenshtein distance."
                      />
                    </p>
                    <EuiHorizontalRule />
                    <p>
                      <FormattedMessage
                        id="exampleRs.timestampText"
                        defaultMessage="Calculated on the client-side: {similarity}"
                        values={{
                          similarity: similarities.client ?? 'Unknown',
                        }}
                      />
                    </p>
                    <p>
                      <FormattedMessage
                        id="exampleRs.timestampText"
                        defaultMessage="Calculated on the server-side: {similarity}"
                        values={{
                          similarity: similarities.server ?? 'Unknown',
                        }}
                      />
                    </p>
                    <EuiFieldText
                      placeholder="Word A"
                      value={words.wordA}
                      onChange={onWordAChange}
                    />
                    <EuiSpacer />
                    <EuiFieldText
                      placeholder="Word B"
                      value={words.wordB}
                      onChange={onWordBChange}
                    />
                    <EuiSpacer />
                    <EuiButton
                      type="primary"
                      size="s"
                      onClick={onClickHandler}
                      isDisabled={!words.wordA || !words.wordB}
                    >
                      <FormattedMessage id="exampleRs.buttonText" defaultMessage="Check" />
                    </EuiButton>
                  </EuiText>
                </EuiPageContentBody>
              </EuiPageContent>
            </EuiPageBody>
          </EuiPage>
        </>
      </I18nProvider>
    </Router>
  );
};
