import * as cdk from 'aws-cdk-lib'
import * as eb from 'aws-cdk-lib/aws-elasticbeanstalk'
import * as elbv2 from 'aws-cdk-lib/aws-elasticloadbalancingv2'
import * as iam from 'aws-cdk-lib/aws-iam'
import * as s3 from 'aws-cdk-lib/aws-s3-assets'
import * as constructs from 'constructs'

import { FullStackAppConfig } from '../full-stack-arch-stack'

interface EbStackProps extends cdk.StackProps {
    fullStackAppConfig: FullStackAppConfig,
    credentialsParameterName: string,
    tag: string,
    certificate?: elbv2.IListenerCertificate,
}

export class EbStack extends cdk.Stack {
  loadBalancerDnsName: string
  ebApp: string
  ebEnv?: string

  constructor (scope: constructs.Construct, id: string, props: EbStackProps) {
    super(scope, id, props)

    // Construct an S3 asset from the ZIP located from directory up.
    const webAppZipArchive = new s3.Asset(this, 'WebAppZip', {
      path: `${__dirname}/../../../../dummy-app/rust-backend.zip`
    })

    // Create a ElasticBeanStalk app.
    const appName = 'MyWebApp2' + props.tag
    const app = new eb.CfnApplication(this, 'Application' + props.tag, {
      applicationName: appName
    })

    // Create an app version from the S3 asset defined earlier
    const appVersionProps = new eb.CfnApplicationVersion(this, 'AppVersion' + props.tag, {
      applicationName: appName,
      sourceBundle: {
        s3Bucket: webAppZipArchive.s3BucketName,
        s3Key: webAppZipArchive.s3ObjectKey
      }
    })

    // Make sure that Elastic Beanstalk app exists before creating an app version
    appVersionProps.addDependsOn(app)

    // Create role and instance profile
    const myRole = new iam.Role(this, `${appName}-aws-elasticbeanstalk-ec2-role` + props.tag, {
      assumedBy: new iam.ServicePrincipal('ec2.amazonaws.com')
    })

    const managedPolicy = iam.ManagedPolicy.fromAwsManagedPolicyName('AWSElasticBeanstalkWebTier')
    myRole.addManagedPolicy(managedPolicy)
    const managedSSMPolicy = iam.ManagedPolicy.fromAwsManagedPolicyName('AmazonSSMManagedInstanceCore')
    myRole.addManagedPolicy(managedSSMPolicy)
    const managedSecretsManagerPolicy = iam.ManagedPolicy.fromAwsManagedPolicyName('SecretsManagerReadWrite')
    myRole.addManagedPolicy(managedSecretsManagerPolicy)

    // need to add secrets manager policy
    const myProfileName = `${appName}-InstanceProfile` + props.tag

    const instanceProfile = new iam.CfnInstanceProfile(this, myProfileName, {
      instanceProfileName: myProfileName,
      roles: [
        myRole.roleName
      ]
    })

    const optionSettingProperties: eb.CfnEnvironment.OptionSettingProperty[] = [
      {
        namespace: 'aws:autoscaling:launchconfiguration',
        optionName: 'IamInstanceProfile',
        value: myProfileName
      },
      {
        namespace: 'aws:autoscaling:asg',
        optionName: 'MinSize',
        value: '1'
      },
      {
        namespace: 'aws:autoscaling:asg',
        optionName: 'MaxSize',
        value: '1'
      },
      {
        namespace: 'aws:ec2:instances',
        optionName: 'InstanceTypes',
        value: 't2.micro'
      },
      {
        namespace: 'aws:elb:loadbalancer',
        optionName: 'CrossZone',
        value: 'true'
      },
      {
        namespace: 'aws:autoscaling:launchconfiguration:', // This doesn't work, did manually
        optionName: 'DisableIMDSv1',
        value: 'true'
      },
      {
        namespace: 'aws:elasticbeanstalk:application:environment',
        optionName: 'SERVER_PORT',
        value: props.fullStackAppConfig.serverPort
      },
      {
        namespace: 'aws:elasticbeanstalk:application:environment',
        optionName: 'AUTH0_DOMAIN',
        value: props.fullStackAppConfig.auth0Domain
      },
      {
        namespace: 'aws:elasticbeanstalk:application:environment',
        optionName: 'AUTH0_CLIENT_ID',
        value: props.fullStackAppConfig.auth0ClientId
      },
      {
        namespace: 'aws:elasticbeanstalk:application:environment',
        optionName: 'AUTH0_AUTHORITY',
        value: props.fullStackAppConfig.auth0Authority
      },
      {
        namespace: 'aws:elasticbeanstalk:application:environment',
        optionName: 'CREDENTIALS_PARAMETER_NAME',
        value: props.credentialsParameterName
      }
    ]

    if (props.certificate) {
      optionSettingProperties.push(
        {
          namespace: 'aws:elb:listener:443',
          optionName: 'ListenerProtocol',
          value: 'HTTPS'
        },
        {
          namespace: 'aws:elb:listener:443',
          optionName: 'SSLCertificateId',
          value: props.certificate?.certificateArn
        },
        {
          namespace: 'aws:elb:listener:443',
          optionName: 'InstancePort',
          value: '80'
        },
        {
          namespace: 'aws:elb:listener:443',
          optionName: 'InstanceProtocol',
          value: 'HTTP'
        }
      )
    }

    // Create an Elastic Beanstalk environment to run the application
    const elbEnv = new eb.CfnEnvironment(this, 'Environment' + props.tag, {
      environmentName: 'MyWebAppEnvironmentYew2' + props.tag,
      applicationName: app.applicationName || appName,
      solutionStackName: '64bit Amazon Linux 2 v3.4.15 running Docker',
      optionSettings: optionSettingProperties,
      versionLabel: appVersionProps.ref
    })

    this.loadBalancerDnsName = elbEnv.attrEndpointUrl
    this.ebApp = elbEnv.applicationName
    this.ebEnv = elbEnv.environmentName
  }
}
