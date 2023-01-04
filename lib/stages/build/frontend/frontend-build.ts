import * as cdk from 'aws-cdk-lib'
import * as codebuild from 'aws-cdk-lib/aws-codebuild'
import * as codecommit from 'aws-cdk-lib/aws-codecommit'
import * as pipeline from 'aws-cdk-lib/aws-codepipeline'
import * as actions from 'aws-cdk-lib/aws-codepipeline-actions'
import * as ecr from 'aws-cdk-lib/aws-ecr'
import * as contructs from 'constructs'

interface FrontendBuildStageStackProps extends cdk.NestedStackProps {
  buildImage: codebuild.IBuildImage
}

export class FrontendBuildStageStack extends cdk.NestedStack {
  private readonly ecrRepository: ecr.IRepository
  private readonly buildOutput: pipeline.Artifact
  private readonly buildImage: codebuild.IBuildImage

  constructor (scope: contructs.Construct, id: string, props: FrontendBuildStageStackProps) {
    super(scope, id, props)
    const repositoryArn = ecr.Repository.arnForLocalRepository('private-custom-rust-build-image', this)
    this.ecrRepository = ecr.Repository.fromRepositoryAttributes(this, 'EcrRepository', {
      repositoryName: 'private-custom-rust-build-image',
      repositoryArn
    })
    this.buildOutput = new pipeline.Artifact()
    this.buildImage = props.buildImage
  }

  public getCodeBuildAction = (sourceOutput: pipeline.Artifact): actions.CodeBuildAction => {
    return new actions.CodeBuildAction({
      actionName: 'FrontendBuildAction',
      input: sourceOutput,
      project: this.createCodeBuildProject(),
      outputs: [this.buildOutput]
    })
  }

  private createCodeBuildProject = (): codebuild.PipelineProject => {
    // TODO: Add S3 Cache
    // const cacheBucketFe = new s3.Bucket(this, 'CodeBuildFeCacheS3Bucket')

    // Code build
    const codeBuildProject = new codebuild.Project(this, 'CodeBuildFrontend2', {
      projectName: 'CodeBuildFrontend2',
      description: 'Frontend',
      buildSpec: codebuild.BuildSpec.fromSourceFilename('app/buildspec-fe.yml'),
      environment: {
        buildImage: this.buildImage,
        privileged: true
      },
      source: codebuild.Source.codeCommit({
        // TODO: Pass repository as param
        repository: codecommit.Repository.fromRepositoryArn(this, 'FullStackAppRepository', 'arn:aws:codecommit:us-east-1:988317291885:FullStackAppAndArch'),
        branchOrRef: 'master'
      })
      // cache: codebuild.Cache.bucket(cacheBucketFe)
    })

    return codeBuildProject
  }

  public getBuildOutput = (): pipeline.Artifact => {
    return this.buildOutput
  }
}
