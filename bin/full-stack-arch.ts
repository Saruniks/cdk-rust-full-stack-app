#!/usr/bin/env node
/* eslint-disable no-new */
import 'source-map-support/register'

import { App } from 'aws-cdk-lib'

import { SourceStageStack } from '../lib/stages/build/build-stage'
import { DeployStack } from '../lib/stages/deploy/deploy-stage'

const app = new App()

// Self mutating pipeline
// new PipelineStack(app, 'PipelineStack', {
//   env: {
//     account: '988317291885',
//     region: 'us-east-1'
//   }
// })

// For development purposes
new SourceStageStack(app, 'SourceStageStack', {
  env: {
    account: '988317291885',
    region: 'us-east-1'
  }
})

new DeployStack(app, 'StgDeployStack', {
  stageName: 'Stg',
  fullStackAppConfig: {
    auth0Domain: 'vendenic.eu.auth0.com',
    auth0ClientId: 'fcp8N3kgCI6gwAEKkqFU6ylFOiscGzv2',
    auth0Authority: 'https://vendenic.eu.auth0.com/',
    serverPort: '80'
  },
  env: {
    account: '988317291885',
    region: 'us-east-1'
  }
})

new DeployStack(app, 'ProdDeployStack', {
  stageName: 'Prod',
  fullStackAppConfig: {
    auth0Domain: 'vendenic.eu.auth0.com',
    auth0ClientId: 'eN3jUJzJAsaCmygamUrGKKeTjLQm4yIb',
    auth0Authority: 'https://vendenic.eu.auth0.com/',
    serverPort: '80'
  },
  env: {
    account: '988317291885',
    region: 'us-east-1'
  }
})

app.synth()
