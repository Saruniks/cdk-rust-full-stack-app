import * as cdk from 'aws-cdk-lib'
import { LinuxBuildImage } from 'aws-cdk-lib/aws-codebuild'
import * as codecommit from 'aws-cdk-lib/aws-codecommit'
import * as codepipeline from 'aws-cdk-lib/aws-codepipeline'
import * as actions from 'aws-cdk-lib/aws-codepipeline-actions'
import * as iam from 'aws-cdk-lib/aws-iam'
import { ManagedPolicy, PolicyStatement } from 'aws-cdk-lib/aws-iam'
import * as s3 from 'aws-cdk-lib/aws-s3'
import * as constructs from 'constructs'
import * as path from 'path'

import { BackendBuildStageStack } from './backend/backend-build'
import { FrontendBuildStageStack } from './frontend/frontend-build'
import { MigrationsBuildStack } from './migrations/migrations-build'

export class SourceStage extends cdk.Stage {
  constructor (scope: constructs.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props)
    const stack = new SourceStageStack(this, 'SourceStageStack', {
      env: {
        account: '988317291885',
        region: 'us-east-1'
      }
    })
  }
}

export class SourceStageStack extends cdk.Stack {
  constructor (scope: constructs.Construct, id: string, props: cdk.StackProps) {
    super(scope, id, props)

    const sourceOutput = new codepipeline.Artifact()
    const repository = codecommit.Repository.fromRepositoryArn(this, 'FullStackAppRepository', 'arn:aws:codecommit:us-east-1:988317291885:FullStackAppAndArch')

    const buildImage = LinuxBuildImage.fromAsset(this, 'RustCodeBuildImage', {
      directory: path.join(__dirname, 'assets/codebuild-image')
    })

    const frontendBuildStack = new FrontendBuildStageStack(this, 'FrontendBuildStageStack', {
      buildImage
    })
    const backendBuildStack = new BackendBuildStageStack(this, 'BackendBuildStageStack', {
      buildImage
    })
    const migrationsBuildStack = new MigrationsBuildStack(this, 'MigrationsBuildStageStack')

    // Creating s3 Bucket
    const frontendArtifactsBucket = s3.Bucket.fromBucketName(this, 'S3BucketForPipelineArtifactsFrontend', 'frontend-artifacts-bucket2')
    const backendArtifactsBucket = s3.Bucket.fromBucketName(this, 'S3BucketForPipelineArtifactsBackend', 'backend-artifacts-bucket2')

    const pipeline = new codepipeline.Pipeline(this, 'FullStackAppPipeline')

    pipeline.addStage({
      stageName: 'Source',
      actions: [new actions.CodeCommitSourceAction({
        actionName: 'Source-Action',
        output: sourceOutput,
        repository
      })]
    })
    pipeline.addStage({
      stageName: 'Build',
      actions: [
        frontendBuildStack.getCodeBuildAction(sourceOutput),
        backendBuildStack.getCodeBuildAction(sourceOutput),
        migrationsBuildStack.getCodeBuildAction(sourceOutput)
      ]
    })

    // Create role and instance profile
    const lambdaDeploymentRole = new iam.Role(this, 'LambdaDeploymentRole', {
      assumedBy: new iam.ServicePrincipal('codepipeline.amazonaws.com')
    })

    const managedPolicy = iam.ManagedPolicy.fromAwsManagedPolicyName('AmazonS3FullAccess')
    lambdaDeploymentRole.addManagedPolicy(managedPolicy)

    const migrationsLambdaBucket = new s3.Bucket(this, 'MigrationsLambdaBucket')

    // migrationsLambdaBucket.grantRead(pipeline.role)
    migrationsLambdaBucket.grantRead(lambdaDeploymentRole)

    const policy = new iam.PolicyStatement({
      resources: ['*'],
      actions: [
        'elasticbeanstalk:*',
        'autoscaling:*',
        'elasticloadbalancing:*',
        'rds:*',
        's3:*',
        'cloudwatch:*',
        'cloudformation:*',
        'ec2:*',
        'secretsmanager:*'
      ]
    })

    lambdaDeploymentRole.addToPrincipalPolicy(policy)

    // Deploy Stage
    const stackName = 'Codepipeline-Lambda-Stack'
    const changeSetName = 'StagedChangeSet'

    const createReplaceChangeSetAction = new actions.CloudFormationCreateReplaceChangeSetAction({
      actionName: 'PrepareChanges',
      stackName,
      changeSetName,
      templatePath: migrationsBuildStack.getBuildOutput().atPath('outputtemplate.yml'),
      cfnCapabilities: [
        cdk.CfnCapabilities.NAMED_IAM,
        cdk.CfnCapabilities.AUTO_EXPAND
      ],
      adminPermissions: true,
      runOrder: 1
    })

    const executeChangeSetAction = new actions.CloudFormationExecuteChangeSetAction({
      actionName: 'ExecuteChanges',
      changeSetName,
      stackName,
      runOrder: 2
    })

    pipeline.addStage({
      stageName: 'Deploy',
      actions: [
        createReplaceChangeSetAction,
        executeChangeSetAction
      ]
    })

    createReplaceChangeSetAction.deploymentRole.addManagedPolicy(ManagedPolicy.fromAwsManagedPolicyName('AWSLambdaExecute'))
    createReplaceChangeSetAction.deploymentRole.attachInlinePolicy(this.getCodePipelineCloudFormationInlinePolicy())

    pipeline.addStage({
      stageName: 'DeployArtifacts',
      actions: [
        new actions.S3DeployAction({
          actionName: 'S3_Deploy_Frontend_Artifacts',
          bucket: frontendArtifactsBucket,
          input: frontendBuildStack.getBuildOutput()
        }),
        new actions.S3DeployAction({
          actionName: 'S3_Deploy_Backend_Artifacts',
          bucket: backendArtifactsBucket,
          input: backendBuildStack.getBuildOutput()
        })
      ]
    })
  }

  // Inline permission policy for CloudFormation
  private getCodePipelineCloudFormationInlinePolicy = () => {
    return new iam.Policy(this, 'CodePipelineCloudFormationInlinePolicy', {
      statements: [
        new PolicyStatement({
          effect: iam.Effect.ALLOW,
          actions: [
            'apigateway:*',
            'codedeploy:*',
            'lambda:*',
            'cloudformation:CreateChangeSet',
            'iam:GetRole',
            'iam:CreateRole',
            'iam:DeleteRole',
            'iam:PutRolePolicy',
            'iam:AttachRolePolicy',
            'iam:DeleteRolePolicy',
            'iam:DetachRolePolicy',
            'iam:PassRole',
            's3:GetObject',
            's3:GetObjectVersion',
            's3:GetBucketVersioning'
          ],
          resources: ['*']
        })
      ]
    })
  }
}
