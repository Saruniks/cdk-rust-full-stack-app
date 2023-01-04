import * as cdk from 'aws-cdk-lib'
import * as codebuild from 'aws-cdk-lib/aws-codebuild'
import * as codecommit from 'aws-cdk-lib/aws-codecommit'
import * as pipeline from 'aws-cdk-lib/aws-codepipeline'
import * as actions from 'aws-cdk-lib/aws-codepipeline-actions'
import { ManagedPolicy } from 'aws-cdk-lib/aws-iam'
import * as s3 from 'aws-cdk-lib/aws-s3'
import * as contructs from 'constructs'

export class MigrationsBuildStack extends cdk.NestedStack {
  private readonly buildOutput: pipeline.Artifact

  constructor (scope: contructs.Construct, id: string, props?: cdk.NestedStackProps) {
    super(scope, id, props)

    this.buildOutput = new pipeline.Artifact()
  }

  public getCodeBuildAction = (sourceOutput: pipeline.Artifact): actions.CodeBuildAction => {
    return new actions.CodeBuildAction({
      actionName: 'MigrationsBuildAction',
      input: sourceOutput,
      project: this.createCodeBuildProject(),
      outputs: [this.buildOutput]
    })
  }

  private createCodeBuildProject = (): codebuild.PipelineProject => {
    // TODO: Add S3 Cache
    // const cacheBucketFe = new s3.Bucket(this, 'CodeBuildFeCacheS3Bucket')

    // Creating s3 Bucket
    const artifactsBucket = new s3.Bucket(this, 'S3BucketForPipelineArtifacts')

    // Code build
    const codeBuildProject = new codebuild.Project(this, 'CodeBuildMigrations', {
      projectName: 'CodeBuildMigrations2',
      description: 'Backend',
      buildSpec: codebuild.BuildSpec.fromObject(this.getBuildSpecContent(artifactsBucket.bucketName)),
      environment: {
        buildImage: codebuild.LinuxBuildImage.fromDockerRegistry('ghcr.io/emk/rust-musl-builder:stable'),
        privileged: true
      },
      // TODO: Pass repository as param
      source: codebuild.Source.codeCommit({
        repository: codecommit.Repository.fromRepositoryArn(this, 'FullStackAppRepository', 'arn:aws:codecommit:us-east-1:988317291885:FullStackAppAndArch'),
        branchOrRef: 'master'
      })
    })

    codeBuildProject.role?.addManagedPolicy(ManagedPolicy.fromAwsManagedPolicyName('AmazonS3FullAccess'))

    return codeBuildProject
  }

  public getBuildOutput = (): pipeline.Artifact => {
    return this.buildOutput
  }

  // Creating the build spec content.
  private getBuildSpecContent = (artifactsBucket: string) => {
    return {
      version: '0.2',
      phases: {
        build: {
          commands: [
            'export TZ=Europe/Kiev',
            'ln -snf /usr/share/zoneinfo/Europe/Kiev /etc/localtime && echo Europe/Kiev > /etc/timezone',
            'apt-get update',
            'rustup component add clippy',
            'rustup target add x86_64-unknown-linux-musl',
            'cd app/crates/db_impl',
            'sudo apt-get install python3-pip python-dev -y',
            'pip3 install aws-sam-cli',
            'export LC_ALL=C.UTF-8',
            'export LANG=C.UTF-8',
            'mkdir /asset-output',
            'sam build',
            'export BUCKET=' + artifactsBucket,
            'sam package --s3-bucket $BUCKET --output-template-file outputtemplate.yml',
            'echo Build completed',
            'cp template.yml /asset-output/template.yml',
            'cp outputtemplate.yml /asset-output/outputtemplate.yml'
          ]
        }
      },
      artifacts: {
        'base-directory': '/asset-output',
        files: [
          'template.yml',
          'outputtemplate.yml'
        ]
      }
    }
  }
}
