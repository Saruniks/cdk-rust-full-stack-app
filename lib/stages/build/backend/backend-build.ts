import * as cdk from 'aws-cdk-lib'
import * as codebuild from 'aws-cdk-lib/aws-codebuild'
import * as codecommit from 'aws-cdk-lib/aws-codecommit'
import * as pipeline from 'aws-cdk-lib/aws-codepipeline'
import * as actions from 'aws-cdk-lib/aws-codepipeline-actions'
import * as ecr from 'aws-cdk-lib/aws-ecr'
import * as contructs from 'constructs'

interface BackendBuildStageStackProps extends cdk.NestedStackProps {
  buildImage: codebuild.IBuildImage
}

export class BackendBuildStageStack extends cdk.NestedStack {
  private readonly ecrRepository: ecr.IRepository
  private readonly buildOutput: pipeline.Artifact
  private readonly buildImage: codebuild.IBuildImage

  constructor (scope: contructs.Construct, id: string, props: BackendBuildStageStackProps) {
    super(scope, id, props)

    const repositoryArn = ecr.Repository.arnForLocalRepository('private-custom-rust-build-image', this)
    const ecrRepository = ecr.Repository.fromRepositoryAttributes(this, 'EcrRepository', {
      repositoryName: 'private-custom-rust-build-image',
      repositoryArn
    })

    this.ecrRepository = ecrRepository
    this.buildOutput = new pipeline.Artifact()
    this.buildImage = props.buildImage
  }

  public getCodeBuildAction = (sourceOutput: pipeline.Artifact): actions.CodeBuildAction => {
    return new actions.CodeBuildAction({
      actionName: 'BackendBuildAction',
      input: sourceOutput,
      project: this.createCodeBuildProject(),
      outputs: [this.buildOutput]
    })
  }

  private createCodeBuildProject = (): codebuild.PipelineProject => {
    // TODO: Fix cache
    // const cacheBucket = new s3.Bucket(this, 'CodeBuildCacheS3Bucket')

    const buildBackend = new codebuild.Project(this, 'CodeBuildBackend', {
      projectName: 'CodeBuildBackend',
      description: 'Backend',
      buildSpec: codebuild.BuildSpec.fromSourceFilename('app/buildspec.yml'),
      environment: {
        buildImage: this.buildImage,
        privileged: true
      },
      // TODO: Pass repository as param
      source: codebuild.Source.codeCommit({
        repository: codecommit.Repository.fromRepositoryArn(this, 'FullStackAppRepository', 'arn:aws:codecommit:us-east-1:988317291885:FullStackAppAndArch'),
        branchOrRef: 'master'
      })
      // cache: codebuild.Cache.bucket(cacheBucket)
    })

    return buildBackend
  }

  public getBuildOutput = (): pipeline.Artifact => {
    return this.buildOutput
  }
}
