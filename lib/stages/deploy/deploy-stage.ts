import * as cdk from 'aws-cdk-lib'
import * as codepipeline from 'aws-cdk-lib/aws-codepipeline'
import * as actions from 'aws-cdk-lib/aws-codepipeline-actions'
import * as lambda from 'aws-cdk-lib/aws-lambda'
import * as s3 from 'aws-cdk-lib/aws-s3'
import * as constructs from 'constructs'

import { ElasticBeanstalkDeployAction } from './backend/elastic-beanstalk-deploy-action'
import { FullStackAppConfig, FullStackArchStack } from './full-stack-arch-stack'

interface DeployStageProps extends cdk.StageProps {
  fullStackAppConfig: FullStackAppConfig,
}

export class DeployStage extends cdk.Stage {
  constructor (scope: constructs.Construct, id: string, props: DeployStageProps) {
    super(scope, id, props)
    const stack = new DeployStack(this, 'DeployStack', {
      fullStackAppConfig: props.fullStackAppConfig,
      env: {
        account: '988317291885',
        region: 'us-east-1'
      },
      stageName: props.stageName ? props.stageName : ''
    })
  }
}

interface DeployStackProps extends cdk.StackProps {
  fullStackAppConfig: FullStackAppConfig,
  stageName: string,
}

export class DeployStack extends cdk.Stack {
  constructor (scope: constructs.Construct, id: string, props: DeployStackProps) {
    super(scope, id, props)

    const app = new FullStackArchStack(this, 'FullStackArchStack', {
      fullStackAppConfig: props.fullStackAppConfig,
      env: {
        account: '988317291885',
        region: 'us-east-1'
      },
      tag: props.stageName
    })

    const pipeline = new codepipeline.Pipeline(this, 'FullStackAppPipeline')

    const frontendBuildOutput = new codepipeline.Artifact()
    const backendBuildOutput = new codepipeline.Artifact()

    pipeline.addStage({
      stageName: 'DeploySource',
      actions: [
        new actions.S3SourceAction({
          actionName: 'FrontendSourceAction',
          output: frontendBuildOutput,
          bucketKey: 'frontend-code.zip',
          bucket: s3.Bucket.fromBucketName(this, 'ImportedFrontendBucketFromName', 'frontend-artifacts-bucket2')
        }),
        new actions.S3SourceAction({
          actionName: 'BackendSourceAction',
          output: backendBuildOutput,
          bucketKey: 'backend-code.zip',
          bucket: s3.Bucket.fromBucketName(this, 'ImportedBackendBucketFromName', 'backend-artifacts-bucket2')
        })
      ]
    })

    pipeline.addStage({
      stageName: 'Invoke',
      actions: [
        new actions.LambdaInvokeAction({
          actionName: 'InvokeMigrations',
          lambda: lambda.Function.fromFunctionName(this, 'MyFunc', 'MyMigrationLambda'),
          userParameters: {
            credentialsParameterName: app.credentialsParameterName
          }
        })
      ]
    })

    pipeline.addStage({
      stageName: 'DeployProd2',
      actions: [
        new ElasticBeanstalkDeployAction({
          id: 'my-elb-app-deploy',
          ebsEnvironmentName: app.ebStack.ebEnv!!,
          ebsApplicationName: app.ebStack.ebApp!!,
          input: backendBuildOutput
        }),
        new actions.S3DeployAction({
          actionName: 'S3_Deploy_Prod',
          bucket: app.bucket,
          input: frontendBuildOutput
        })
      ]
    })
  }
}
