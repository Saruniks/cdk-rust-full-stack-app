import * as cdk from 'aws-cdk-lib'
import * as codecommit from 'aws-cdk-lib/aws-codecommit'
import { CodePipeline, CodePipelineSource, ManualApprovalStep, ShellStep } from 'aws-cdk-lib/pipelines'
import * as contructs from 'constructs'

import { SourceStage } from './stages/build/build-stage'
import { DeployStage } from './stages/deploy/deploy-stage'

export class PipelineStack extends cdk.Stack {
  constructor (scope: contructs.Construct, id: string, props: cdk.StackProps) {
    super(scope, id, props)

    const codeCommitRepo = codecommit.Repository.fromRepositoryArn(this, 'FullStackAppRepo', 'arn:aws:codecommit:us-east-1:988317291885:FullStackAppAndArch')

    const pipeline = new CodePipeline(this, 'Pipeline', {
      pipelineName: 'MyCdkPipeline',
      synth: new ShellStep('Synth', {
        input: CodePipelineSource.codeCommit(codeCommitRepo, 'master'),
        commands: ['npm ci', 'npm run build', 'npx cdk synth']
      })
    })

    const sourceStage = new SourceStage(this, 'SourceStage')
    pipeline.addStage(
      sourceStage
    )

    const deployStagingStage = new DeployStage(this, 'Stg', {
      stageName: 'Stg',
      fullStackAppConfig: {
        auth0Domain: 'vendenic.eu.auth0.com',
        auth0ClientId: 'fcp8N3kgCI6gwAEKkqFU6ylFOiscGzv2',
        auth0Authority: 'https://vendenic.eu.auth0.com/',
        serverPort: '80'
      }
    })
    pipeline.addStage(
      deployStagingStage
    )

    const deployProdStage = new DeployStage(this, 'Prod', {
      stageName: 'Prod',
      fullStackAppConfig: {
        auth0Domain: 'vendenic.eu.auth0.com',
        auth0ClientId: 'eN3jUJzJAsaCmygamUrGKKeTjLQm4yIb',
        auth0Authority: 'https://vendenic.eu.auth0.com/',
        serverPort: '80'
      }
    })
    pipeline.addStage(
      deployProdStage,
      {
        pre: [new ManualApprovalStep('CdkToProd')]
      }
    )
  }
}
