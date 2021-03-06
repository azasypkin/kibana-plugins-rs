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
  EuiPageHeader,
  EuiTitle,
  EuiText,
  EuiFieldText,
  EuiSpacer,
  EuiLink,
} from '@elastic/eui';

import { CoreStart } from '@kbn/core/public';
import { NavigationPublicPluginStart } from '@kbn/navigation-plugin/public';

import { PLUGIN_ID } from '../../common';
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
  const [operationResult, setOperationResult] = useState<{
    comment?: string;
    client?: number;
    server?: number;
  }>({});

  const onWordAChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setWords({ ...words, wordA: e.target.value });
  };

  const onWordBChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setWords({ ...words, wordB: e.target.value });
  };

  const onClickHandler = async () => {
    const client = await ownStart
      .findSimilarity(words.wordA!, words.wordB!)
      .then((similarity) => similarity.value);
    const server = await http.post<{ comment: string; similarity: number }>('/api/wasm', {
      body: JSON.stringify(words),
    });
    setOperationResult({ client, server: server.similarity, comment: server.comment });
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
                      id="exampleRs.congratulationsTitle"
                      defaultMessage="???? Rust + ??? Kibana = ????"
                    />
                  </h1>
                </EuiTitle>
              </EuiPageHeader>
              <EuiPageContent>
                <EuiPageContentBody>
                  <EuiText>
                    <p>
                      <EuiText>
                        Check similarity of two words using Levenshtein distance algorithm from Rust{' '}
                        <EuiLink href="https://crates.io/crates/strsim" target="_blank">
                          <b>strsim-rs</b>
                        </EuiLink>{' '}
                        crate
                      </EuiText>
                    </p>
                    <EuiHorizontalRule />
                    <p>
                      <FormattedMessage
                        id="exampleRs.timestampText"
                        defaultMessage="Calculated on the {source}: {similarity}"
                        values={{
                          similarity: operationResult.client ?? '?',
                          source: <b>client-side</b>,
                        }}
                      />
                    </p>
                    <p>
                      <FormattedMessage
                        id="exampleRs.timestampText"
                        defaultMessage="Calculated on the {source}: {similarity}"
                        values={{
                          similarity: operationResult.server ?? '?',
                          source: <b>server-side</b>,
                        }}
                      />
                    </p>
                    <p>
                      <FormattedMessage
                        id="exampleRs.localizedText"
                        defaultMessage="String created and localized on the {source}: {user}"
                        values={{
                          user: operationResult.comment ?? '?',
                          source: <b>server-side</b>,
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
