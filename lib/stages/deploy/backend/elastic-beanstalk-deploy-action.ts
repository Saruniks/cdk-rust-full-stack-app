import * as codepipeline from 'aws-cdk-lib/aws-codepipeline'
import * as events from 'aws-cdk-lib/aws-events'
import * as iam from 'aws-cdk-lib/aws-iam'
import * as contructs from 'constructs'

export interface ElasticBeanstalkDeployActionProps {
    id: string;
    ebsApplicationName: string;
    ebsEnvironmentName: string;
    input: codepipeline.Artifact;
    role?: iam.IRole;
}

export class ElasticBeanstalkDeployAction implements codepipeline.IAction {
  readonly actionProperties: codepipeline.ActionProperties
  private readonly props: ElasticBeanstalkDeployActionProps

  constructor (props: ElasticBeanstalkDeployActionProps) {
    this.actionProperties = {
      ...props,
      category: codepipeline.ActionCategory.DEPLOY,
      actionName: `${props.id}-elasticbeanstalk-deploy-action`,
      owner: 'AWS',
      provider: 'ElasticBeanstalk',

      artifactBounds: {
        minInputs: 1,
        maxInputs: 1,
        minOutputs: 0,
        maxOutputs: 0
      },
      inputs: [props.input]
    }
    this.props = props
  }

  bind (scope: contructs.Construct, stage: codepipeline.IStage, options: codepipeline.ActionBindOptions): codepipeline.ActionConfig {
    options.bucket.grantRead(options.role)
    options.role.addToPrincipalPolicy(new iam.PolicyStatement({
      resources: ['*'],
      // actions: ['elasticbeanstalk:CreateApplicationVersion', 'elasticbeanstalk:UpdateEnvironment'],
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
    }))
    return {
      configuration: {
        ApplicationName: this.props.ebsApplicationName,
        EnvironmentName: this.props.ebsEnvironmentName
      }
    }
  }

  onStateChange (name: string, target?: events.IRuleTarget, options?: events.RuleProps): events.Rule {
    throw new Error('not supported')
  }
}
