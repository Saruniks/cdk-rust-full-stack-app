import * as cdk from 'aws-cdk-lib'
import * as ec2 from 'aws-cdk-lib/aws-ec2'
import * as rds from 'aws-cdk-lib/aws-rds'
import * as secretsmanager from 'aws-cdk-lib/aws-secretsmanager'
import * as ssm from 'aws-cdk-lib/aws-ssm'
import * as constructs from 'constructs'

interface RdsStackProps extends cdk.StackProps {
    yourIpAddres: string,
    securityGroupId: string,
    tag: string,
}

export class RdsStack extends cdk.Stack {
  credentialsParameterName: string
  loadBalancerDnsName: string
  rdsInstance: rds.IDatabaseInstance

  constructor (scope: constructs.Construct, id: string, props: RdsStackProps) {
    super(scope, id, props)

    const vpc = ec2.Vpc.fromLookup(this, 'Vpc' + props.tag, {
      vpcId: 'vpc-09cf5b1fce29bed59'
    })

    const databaseCredentialsSecret = new secretsmanager.Secret(this, 'DBCredentialsSecret2' + props.tag, {
      secretName: 'DbCredentials4' + props.tag,
      generateSecretString: {
        secretStringTemplate: JSON.stringify({
          username: 'postgres'
        }),
        excludePunctuation: true,
        includeSpace: false,
        generateStringKey: 'password'
      }
    })

    // lets output a few properties to help use find the credentials
    new cdk.CfnOutput(this, 'Secret Name', { value: databaseCredentialsSecret.secretName })
    new cdk.CfnOutput(this, 'Secret ARN', { value: databaseCredentialsSecret.secretArn })
    new cdk.CfnOutput(this, 'Secret Full ARN', { value: databaseCredentialsSecret.secretFullArn || '' })

    // TODO: Maybe just use it in secrets manager
    // next, create a new string parameter to be used
    const parameter = new ssm.StringParameter(this, 'DBCredentialsArn2' + props.tag, {
      parameterName: 'Credentials-arn3' + props.tag,
      stringValue: databaseCredentialsSecret.secretArn
    })

    this.credentialsParameterName = parameter.parameterName

    // get the default security group
    const defaultSecurityGroup = ec2.SecurityGroup.fromSecurityGroupId(this, 'SG', props.securityGroupId)

    if (props?.yourIpAddres) {
      // your to access your RDS instance!
      defaultSecurityGroup.addIngressRule(ec2.Peer.ipv4(props.yourIpAddres), ec2.Port.tcp(5432), 'allow 5432 access from my IP')
    }

    // finally, lets configure and create our database!
    const rdsConfig: rds.DatabaseInstanceProps = {
      engine: rds.DatabaseInstanceEngine.postgres({ version: rds.PostgresEngineVersion.VER_13_7 }),
      // optional, defaults to m5.large
      instanceType: ec2.InstanceType.of(ec2.InstanceClass.BURSTABLE3, ec2.InstanceSize.MICRO),
      vpc,
      // make the db publically accessible
      vpcSubnets: {
        subnetType: ec2.SubnetType.PUBLIC
      },
      instanceIdentifier: 'rdsInstanceIdentifier2' + props.tag,
      maxAllocatedStorage: 200,
      securityGroups: [defaultSecurityGroup],
      credentials: rds.Credentials.fromSecret(databaseCredentialsSecret) // Get both username and password from existing secret
    }

    // create the instance
    this.rdsInstance = new rds.DatabaseInstance(this, 'rds-instance' + props.tag, rdsConfig)
    // output the endpoint so we can connect!
    new cdk.CfnOutput(this, 'RDS Endpoint', { value: this.rdsInstance.dbInstanceEndpointAddress })
  }
}
